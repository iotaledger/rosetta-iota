// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{currency::iota_currency, error::ApiError, options::Options, types::{Amount, BlockIdentifier}, build_iota_client, require_online_mode, is_bad_network};
use crate::types::{NetworkIdentifier, AccountIdentifier, PartialBlockIdentifier};

use log::debug;
use serde::{Deserialize, Serialize};

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
    account_balance_request: AccountBalanceRequest,
    options: Options,
) -> Result<AccountBalanceResponse, ApiError> {
    debug!("/account/balance");

    let _ = require_online_mode(&options)?;
    is_bad_network(&options, &account_balance_request.network_identifier)?;

    // historical balance lookup is not supported
    if account_balance_request.block_identifier.is_some() {
        return Err(ApiError::HistoricalBalancesUnsupported);
    }

    let iota_client = build_iota_client(&options).await?;

    let node_info = match iota_client.get_info().await {
        Ok(node_info) => node_info,
        Err(_) => return Err(ApiError::UnableToGetNodeInfo),
    };

    let confirmed_milestone = match iota_client.get_milestone(node_info.confirmed_milestone_index).await {
        Ok(confirmed_milestone) => confirmed_milestone,
        Err(_) => return Err(ApiError::UnableToGetMilestone(node_info.confirmed_milestone_index)),
    };

    let address = account_balance_request.account_identifier.address;
    let balance = match iota_client.get_address().balance(&address.into()).await {
        Ok(balance) => balance,
        Err(_) => return Err(ApiError::UnableToGetBalance),
    };

    let response = AccountBalanceResponse {
        block_identifier: BlockIdentifier {
            index: confirmed_milestone.index,
            hash: confirmed_milestone.message_id.to_string(),
        },
        balances: vec![
            Amount {
                value: balance.balance.to_string(),
                currency: iota_currency(),
                metadata: None
            }
        ]
    };

    Ok(response)
}