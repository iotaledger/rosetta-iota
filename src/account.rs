use iota::Client;
use crate::{
    consts,
    error::ApiError,
    filters::{handle, with_options},
    options::Options,
    types::{AccountBalanceRequest, AccountBalanceResponse, Amount, BlockIdentifier, Currency},
};
use log::debug;
use warp::Filter;

pub fn routes(
    options: Options,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post().and(
        warp::path!("account" / "balance")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(account_balance)),
    )
}

async fn account_balance(
    account_balance_request: AccountBalanceRequest,
    options: Options,
) -> Result<AccountBalanceResponse, ApiError> {
    debug!("/account/balance");

    let network_identifier = account_balance_request.network_identifier;
    if network_identifier.blockchain != consts::BLOCKCHAIN
        || network_identifier.network != options.network
    {
        return Err(ApiError::BadNetwork);
    }

    // no historical balance lookup
    if account_balance_request.block_identifier.is_some() {
        return Err(ApiError::HistoricalBalancesUnsupported);
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

    let solid_milestone_index = node_info.solid_milestone_index as u64;
    let solid_milestone = match iota_client.get_milestone(solid_milestone_index).await {
        Ok(solid_milestone) => solid_milestone,
        Err(_) => return Err(ApiError::UnableToGetMilestone),
    };

    let block_identifier = BlockIdentifier {
        index: solid_milestone.index as u64,
        hash: solid_milestone.message_id.to_string(),
    };

    let address = account_balance_request.account_identifier.address;
    let balances = vec![];

    let response = AccountBalanceResponse {
        block_identifier,
        balances,
    };

    Ok(response)
}