// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{currency::iota_currency, error::ApiError, options::Options, types::{AccountBalanceRequest, AccountBalanceResponse,
                                                                                        Amount, BlockIdentifier}, build_iota_client, require_online_mode, is_bad_network};
use log::debug;

pub async fn account_balance(
    account_balance_request: AccountBalanceRequest,
    options: Options,
) -> Result<AccountBalanceResponse, ApiError> {
    debug!("/account/balance");

    let _ = require_online_mode(&options)?;

    is_bad_network(&options, &account_balance_request.network_identifier)?;

    // no historical balance lookup
    if account_balance_request.block_identifier.is_some() {
        return Err(ApiError::HistoricalBalancesUnsupported);
    }

    let iota_client = build_iota_client(&options, true).await?;

    let node_info = match iota_client.get_info().await {
        Ok(node_info) => node_info,
        Err(_) => return Err(ApiError::UnableToGetNodeInfo),
    };

    let confirmed_milestone_index = node_info.confirmed_milestone_index;
    let solid_milestone = match iota_client.get_milestone(confirmed_milestone_index).await {
        Ok(solid_milestone) => solid_milestone,
        Err(_) => return Err(ApiError::UnableToGetMilestone(confirmed_milestone_index)),
    };

    let block_identifier = BlockIdentifier {
        index: solid_milestone.index,
        hash: solid_milestone.message_id.to_string(),
    };

    let address = account_balance_request.account_identifier.address;

    let balance = match iota_client.get_address().balance(&address.into()).await {
        Ok(balance) => balance,
        Err(_) => return Err(ApiError::UnableToGetBalance),
    };

    let response = AccountBalanceResponse {
        block_identifier,
        balances: vec![Amount {
            value: balance.balance.to_string(),
            currency: iota_currency()
        }]
    };

    Ok(response)
}