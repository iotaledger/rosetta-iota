use crate::Config;
use crate::error::ApiError;

use bee_message::prelude::*;
use bee_rest_api::types::responses::*;

use iota::Client;
use bee_rest_api::types::dtos::PeerDto;


pub async fn build_client(
    options: &Config
) -> Result<Client, ApiError> {
    let builder = iota::Client::builder()
        .with_network(&options.network)
        .with_node(&options.node_url)
        .map_err(|e| ApiError::NonRetriable(format!("unable to build client: {}", e)))?;
    Ok(builder.finish().await.map_err(|e| ApiError::NonRetriable(format!("unable to build client: {}", e)))?)
}

pub async fn get_output(
    output_id: OutputId,
    client: &Client,
) -> Result<OutputResponse, ApiError> {
    client
        .get_output(&(output_id.into()))
        .await
        .map_err(|e| ApiError::NonRetriable(format!("can not get output: {}", e)))
}

pub async fn get_outputs_of_address(
    bech32_addr: &str,
    client: &Client,
) -> Result<Vec<OutputResponse>, ApiError> {
    match client.find_outputs(&[], &[bech32_addr.to_string()]).await {
        Ok(outputs) => Ok(outputs),
        Err(e) => return Err(ApiError::NonRetriable(format!("can not get outputs of address: {}", e))),
    }
}

pub async fn get_balance_of_address(
    bech32_addr: &str,
    client: &Client,
) -> Result<BalanceAddressResponse, ApiError> {
    match client.get_address().balance(bech32_addr).await {
        Ok(balance) => Ok(balance),
        Err(_) => return Err(ApiError::NonRetriable("unable to get balance".to_string())),
    }
}

pub async fn get_milestone(
    milestone_index: u32,
    client: &Client
) -> Result<iota::MilestoneResponse, ApiError> {
    match client.get_milestone(milestone_index).await {
        Ok(milestone) => Ok(milestone),
        Err(_) => return Err(ApiError::NonRetriable("can not get milestone".to_string())),
    }
}

pub async fn get_genesis_milestone(
    client: &Client
) -> Result<iota::MilestoneResponse, ApiError> {
    if get_node_info(&client).await?.pruning_index > 0 {
        Ok(iota::MilestoneResponse {
            index: 1,
            message_id: MessageId::null(),
            timestamp: 0
        })
    } else {
        get_milestone(1, &client).await
    }
}

pub async fn get_confirmed_milestone_index(
    client: &Client
) -> Result<u32, ApiError> {
    match client.get_info().await {
        Ok(res) => Ok(res.nodeinfo.confirmed_milestone_index),
        Err(e) => return Err(ApiError::NonRetriable(format!("unable to get node info: {}", e))),
    }
}

pub async fn get_confirmed_milestone(
    client: &Client
) -> Result<iota::MilestoneResponse, ApiError> {
    let confirmed_milestone_index = get_confirmed_milestone_index(&client).await?;
    get_milestone(confirmed_milestone_index, &client).await
}

pub async fn get_latest_milestone_index(
    client: &Client
) -> Result<u32, ApiError> {
    match client.get_info().await {
        Ok(res) => Ok(res.nodeinfo.latest_milestone_index),
        Err(e) => return Err(ApiError::NonRetriable(format!("unable to get node info: {}", e))),
    }
}

pub async fn get_latest_milestone(
    client: &Client
) -> Result<iota::MilestoneResponse, ApiError> {
    let latest_milestone_index = get_latest_milestone_index(&client).await?;
    get_milestone(latest_milestone_index, &client).await
}

pub async fn get_node_info(
    client: &Client
) -> Result<InfoResponse, ApiError> {
    match client.get_info().await {
        Ok(res) => Ok(res.nodeinfo),
        Err(e) => return Err(ApiError::NonRetriable(format!("unable to get node info: {}", e))),
    }
}

pub async fn get_peers(
    client: &Client
) -> Result<Vec<PeerDto>, ApiError> {
    client.get_peers().await.map_err(|e| ApiError::NonRetriable(format!("unable to get peers: {}", e)))
}

pub async fn get_utxo_changes(
    milestone_index: u32,
    client: &Client,
) -> Result<UtxoChangesResponse, ApiError> {
    let confirmed_index = get_confirmed_milestone_index(client).await?;
    if milestone_index > confirmed_index {
        return Err(ApiError::Retriable(format!("milestone with index {} not available yet", milestone_index)))
    } else {
        client.get_milestone_utxo_changes(milestone_index).await.map_err(|e| ApiError::NonRetriable(format!("can not get uxto-changes: {}", e)))
    }
}

