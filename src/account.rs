// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    currency::iota_currency,
    consts,
    error::ApiError,
    filters::{handle, with_options},
    options::Options,
    types::{AccountBalanceRequest, AccountBalanceResponse, AccountCoinsRequest, AccountCoinsResponse,
            Amount, BlockIdentifier, Coin, CoinIdentifier},
};
use iota::{Bech32Address, OutputDto, AddressDto};
use log::debug;
use warp::Filter;

pub fn routes(options: Options) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post().
        and(
        warp::path!("account" / "balance")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(account_balance)),
        )
        .or(warp::path!("account" / "coins")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(account_coins)))
}

async fn account_balance(
    account_balance_request: AccountBalanceRequest,
    options: Options,
) -> Result<AccountBalanceResponse, ApiError> {
    debug!("/account/balance");

    if options.mode != consts::ONLINE_MODE {
        return Err(ApiError::UnavailableOffline);
    }

    let network_identifier = account_balance_request.network_identifier;
    if network_identifier.blockchain != consts::BLOCKCHAIN || network_identifier.network != options.network {
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
        .await
    {
        Ok(iota_client) => iota_client,
        Err(_) => return Err(ApiError::UnableToBuildClient),
    };

    let node_info = match iota_client.get_info().await {
        Ok(node_info) => node_info,
        Err(_) => return Err(ApiError::UnableToGetNodeInfo),
    };

    let solid_milestone_index = node_info.solid_milestone_index;
    let solid_milestone = match iota_client.get_milestone(solid_milestone_index).await {
        Ok(solid_milestone) => solid_milestone,
        Err(_) => return Err(ApiError::UnableToGetMilestone(solid_milestone_index)),
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

async fn account_coins(
    account_coins_request: AccountCoinsRequest,
    options: Options,
) -> Result<AccountCoinsResponse, ApiError> {
    debug!("/account/coins");

    if options.mode != consts::ONLINE_MODE {
        return Err(ApiError::UnavailableOffline);
    }

    let network_identifier = account_coins_request.network_identifier;
    if network_identifier.blockchain != consts::BLOCKCHAIN || network_identifier.network != options.network {
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

    let solid_milestone_index = node_info.solid_milestone_index;
    let solid_milestone = match iota_client.get_milestone(solid_milestone_index).await {
        Ok(solid_milestone) => solid_milestone,
        Err(_) => return Err(ApiError::UnableToGetMilestone(solid_milestone_index)),
    };

    let block_identifier = BlockIdentifier {
        index: solid_milestone.index,
        hash: solid_milestone.message_id.to_string(),
    };

    let address = Bech32Address(account_coins_request.account_identifier.address);
    let outputs = match iota_client.find_outputs(&[], &[address.clone()]).await {
        Ok(outputs) => outputs,
        Err(_) => return Err(ApiError::UnableToGetOutputsFromAddress),
    };

    let mut coins = vec![];
    for output in outputs {
        let amount = match output.output {
            OutputDto::Treasury(_) => panic!("Can't be used as input"),
            OutputDto::SignatureLockedSingle(r) => match r.address {
                AddressDto::Ed25519(_) => r.amount,
            },
            OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                AddressDto::Ed25519(_) => r.amount,
            },
        };

        coins.push(Coin {
            coin_identifier: CoinIdentifier {
                identifier: output.transaction_id
            },
            amount: Amount {
                value: amount.to_string(),
                currency: iota_currency(),
            }
        });
    }

    let response = AccountCoinsResponse {
        block_identifier,
        coins
    };

    Ok(response)
}
