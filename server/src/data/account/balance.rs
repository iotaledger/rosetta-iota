// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{build_iota_client, currency::iota_currency, error::ApiError, is_wrong_network, options::Options, types::{AccountIdentifier, Amount, BlockIdentifier, NetworkIdentifier, PartialBlockIdentifier}, is_offline_mode_enabled};

use log::debug;
use serde::{Deserialize, Serialize};

use iota::{Client, MilestoneResponse};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountBalanceRequest {
    pub network_identifier: NetworkIdentifier,
    pub account_identifier: AccountIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<PartialBlockIdentifier>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountBalanceResponse {
    pub block_identifier: BlockIdentifier,
    pub balances: Vec<Amount>,
}

pub async fn account_balance(
    request: AccountBalanceRequest,
    options: Options,
) -> Result<AccountBalanceResponse, ApiError> {
    debug!("/account/balance");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()))
    }

    if is_offline_mode_enabled(&options) {
        return Err(ApiError::NonRetriable("endpoint does not support offline mode".to_string()))
    }

    // historical balance lookup is not supported
    if request.block_identifier.is_some() {
        return Err(ApiError::NonRetriable("historical balance lookup not supported".to_string()));
    }

    let (amount, confirmed_milestone) =
        balance_at_milestone(&request.account_identifier.address, &options).await?;

    let response = AccountBalanceResponse {
        block_identifier: BlockIdentifier {
            index: confirmed_milestone.index,
            hash: confirmed_milestone.message_id.to_string(),
        },
        balances: vec![amount],
    };

    Ok(response)
}

async fn balance_at_milestone(address: &str, options: &Options) -> Result<(Amount, MilestoneResponse), ApiError> {
    let iota_client = build_iota_client(options).await?;

    // to make sure the balance of an address does not change in the meantime, check the index of the confirmed
    // milestone before and after fetching the balance

    let index_before_balance_check = get_confirmed_milestone(&iota_client).await?;

    let balance_response = match iota_client.get_address().balance(address).await {
        Ok(balance) => balance,
        Err(_) => return Err(ApiError::NonRetriable("unable to get balance".to_string())),
    };

    let index_after_balance_check = get_confirmed_milestone(&iota_client).await?;

    if index_before_balance_check.index != index_after_balance_check.index {
        return Err(ApiError::NonRetriable("milestone index changed while performing the request".to_string()));
    }

    let amount = Amount {
        value: balance_response.balance.to_string(),
        currency: iota_currency(),
        metadata: None,
    };

    Ok((amount, index_before_balance_check))
}

async fn get_confirmed_milestone(iota_client: &Client) -> Result<MilestoneResponse, ApiError> {
    let confirmed_milestone_index = {
        let node_info = match iota_client.get_info().await {
            Ok(node_info) => node_info,
            Err(e) => return Err(ApiError::NonRetriable(format!("unable to get node info: {}", e))),
        };

        node_info.confirmed_milestone_index
    };

    match iota_client.get_milestone(confirmed_milestone_index).await {
        Ok(confirmed_milestone) => Ok(confirmed_milestone),
        Err(e) => return Err(ApiError::NonRetriable(format!("unable to get milestone: {}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::RosettaMode;

    #[tokio::test]
    async fn test_balance() {
        let request = AccountBalanceRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet7".to_string(),
                sub_network_identifier: None,
            },
            account_identifier: AccountIdentifier {
                address: String::from("atoi1qqp4g5xv4zjweaj5tu44yn365afdhe3n3t9nmp9wqreahzp8a3egc5zrx2h"),
                sub_account: None,
            },
            block_identifier: None,
        };

        let server_options = Options {
            node: "https://api.hornet-rosetta.testnet.chrysalis2.com".to_string(),
            network: "testnet7".to_string(),
            indexation: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };

        let response = account_balance(request, server_options).await.unwrap();

        assert_eq!("IOTA", response.balances[0].currency.symbol);
        assert_eq!(0, response.balances[0].currency.decimals);
        // todo: more assertions
    }
}
