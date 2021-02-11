// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    consts,
    error::ApiError,
    filters::{handle, with_empty_request, with_options, EmptyRequest},
    operations::*,
    options::Options,
    types::{
        Allow, BlockIdentifier, NetworkIdentifier, NetworkListResponse, NetworkOptionsResponse, NetworkRequest,
        NetworkStatusResponse, Peer, PeerMetadata, Version,
    },
};
use bee_message::prelude::{MessageId, MESSAGE_ID_LENGTH};
use iota::{self, client::MilestoneResponse};
use log::debug;
use warp::Filter;

pub fn routes(options: Options) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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

async fn network_list(_empty: EmptyRequest, options: Options) -> Result<NetworkListResponse, ApiError> {
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
    operation_statuses.push(operation_status_success());
    operation_statuses.push(operation_status_fail());

    let operation_types = operation_type_list();

    let errors = ApiError::all_errors();

    let allow = Allow {
        operation_statuses,
        operation_types,
        errors,
        historical_balance_lookup: false,
        timestamp_start_index: Some(0),
        call_methods: vec![],
        balance_exemptions: vec![],
    };

    let response = NetworkOptionsResponse { version, allow };

    Ok(response)
}

async fn network_status(network_request: NetworkRequest, options: Options) -> Result<NetworkStatusResponse, ApiError> {
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
        .await
    {
        Ok(iota_client) => iota_client,
        Err(_) => return Err(ApiError::UnableToBuildClient),
    };

    let node_info = match iota_client.get_info().await {
        Ok(node_info) => node_info,
        Err(_) => return Err(ApiError::UnableToGetNodeInfo),
    };

    let genesis_milestone = match iota_client.get_milestone(1).await {
        Ok(genesis_milestone) => genesis_milestone,
        Err(_) => MilestoneResponse {
            index: 1,
            message_id: MessageId::new([0; MESSAGE_ID_LENGTH]),
            timestamp: 0,
        },
    };

    let latest_milestone_index = node_info.latest_milestone_index as u64;
    let latest_milestone = match iota_client.get_milestone(latest_milestone_index).await {
        Ok(latest_milestone) => latest_milestone,
        Err(_) => return Err(ApiError::UnableToGetMilestone),
    };

    let solid_milestone_index = node_info.solid_milestone_index as u64;
    let solid_milestone = match iota_client.get_milestone(solid_milestone_index).await {
        Ok(solid_milestone) => solid_milestone,
        Err(_) => return Err(ApiError::UnableToGetMilestone),
    };

    let current_block_timestamp = latest_milestone.timestamp * 1000;
    let peers_bee = match iota_client.get_peers().await {
        Ok(peers) => peers,
        Err(_) => return Err(ApiError::UnableToGetPeers),
    };

    let mut peers = vec![];
    for peer_bee in peers_bee {
        peers.push(Peer {
            peer_id: peer_bee.id,
            metadata: PeerMetadata {
                multi_addresses: peer_bee.multi_addresses,
                alias: peer_bee.alias,
                connected: peer_bee.connected,
            },
        });
    }

    let genesis_block_identifier = BlockIdentifier {
        index: genesis_milestone.index as u64,
        hash: genesis_milestone.message_id.to_string(),
    };

    let current_block_identifier = BlockIdentifier {
        index: latest_milestone.index as u64,
        hash: latest_milestone.message_id.to_string(),
    };

    let response = NetworkStatusResponse {
        current_block_identifier,
        current_block_timestamp,
        genesis_block_identifier,
        peers,
    };

    Ok(response)
}
