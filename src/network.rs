use crate::{
    consts,
    error::ApiError,
    filters::{handle, with_empty_request, with_options, EmptyRequest},
    options::Options,
    types::{
        Allow, BlockIdentifier, NetworkIdentifier, NetworkListResponse, NetworkOptionsResponse,
        NetworkRequest, NetworkStatusResponse, OperationStatus, Peer, Version,
    },
};
use log::debug;
use warp::Filter;
use iota::Client;
use bee_rest_api::types;

pub fn routes(
    options: Options,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(
            warp::path!("network" / "list")
                .and(with_empty_request())
                .and(with_options(options.clone()))
                .and_then(handle(network_list)),
        )
        .or(warp::path!("network" / "options")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(network_options)))
        .or(warp::path!("network" / "status")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(network_status)))
}

async fn network_list(
    _empty: EmptyRequest,
    options: Options,
) -> Result<NetworkListResponse, ApiError> {
    debug!("/network/list");
    let response = NetworkListResponse {
        network_identifiers: vec![NetworkIdentifier {
            blockchain: consts::BLOCKCHAIN.to_string(),
            network: options.network.clone(),
            sub_network_identifier: None,
        }],
    };

    Ok(response)
}

async fn network_options(
    network_request: NetworkRequest,
    options: Options,
) -> Result<NetworkOptionsResponse, ApiError> {
    debug!("/network/options");
    if network_request.network_identifier.blockchain != consts::BLOCKCHAIN
        || network_request.network_identifier.network != options.network
    {
        return Err(ApiError::BadNetwork);
    }

    let version = Version {
        rosetta_version: consts::ROSETTA_VERSION.to_string(),
        node_version: consts::NODE_VERSION.to_string(),
        middleware_version: consts::MIDDLEWARE_VERSION.to_string(),
    };

    let mut operation_statuses = Vec::new();
    // for op in diem::vmstatus_all_strs() {
    //     operation_statuses.push(OperationStatus {
    //         status: op.to_string(),
    //         successful: op == "executed",
    //     });
    // }

    let operation_types = vec![
        "message".to_string(),
        "indexed_message".to_string(),
        "transaction".to_string(),
        // "streams_channel".to_string(), // Streams operations?
    ];

    let errors = ApiError::all_errors();

    let allow = Allow {
        operation_statuses,
        operation_types,
        errors,
        historical_balance_lookup: false,
        timestamp_start_index: Some(3), // FIXME: hardcoded based on current testnet
        call_methods: vec![],
        balance_exemptions: vec![],
    };

    let response = NetworkOptionsResponse { version, allow };

    Ok(response)
}

async fn network_status(
    network_request: NetworkRequest,
    options: Options,
) -> Result<NetworkStatusResponse, ApiError> {
    debug!("/network/status");
    if network_request.network_identifier.blockchain != consts::BLOCKCHAIN
        || network_request.network_identifier.network != options.network
    {
        return Err(ApiError::BadNetwork);
    }

    let iota_client = match iota::Client::builder()
        .with_network(&options.network)
        .with_node(&options.iota_endpoint)
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await {
        Ok(iota_client) => iota_client,
        Err(_) => return Err(ApiError::UnableToBuildClient),
    };

    let node_info = match iota_client.get_info().await {
        Ok(node_info) => node_info,
        Err(_) => return Err(ApiError::UnableToGetNodeInfo),
    };

    let genesis_milestone = iota_client.get_milestone(1).await.unwrap();
    let solid_milestone_index = node_info.solid_milestone_index as u64;
    let solid_milestone = iota_client.get_milestone(solid_milestone_index).await.unwrap();
    let current_block_timestamp = solid_milestone.timestamp;
    let peers = iota_client.get_peers().await.unwrap();


    let genesis_block_identifier = BlockIdentifier {
        index: genesis_milestone.index as u64,
        hash: genesis_milestone.message_id.to_string(),
    };

    let current_block_identifier = BlockIdentifier {
        index: solid_milestone.index as u64,
        hash: solid_milestone.message_id.to_string(),
    };

    let response = NetworkStatusResponse {
        current_block_identifier,
        current_block_timestamp,
        genesis_block_identifier,
        peers,
    };

    Ok(response)
}
