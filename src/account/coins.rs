// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{currency::iota_currency, error::ApiError, options::Options, types::{AccountCoinsRequest, AccountCoinsResponse,
                                                                                        Amount, BlockIdentifier, Coin, CoinIdentifier}, build_iota_client, require_online_mode, is_bad_network};
use iota::{Bech32Address, OutputDto, AddressDto};
use log::debug;

pub async fn account_coins(
    account_coins_request: AccountCoinsRequest,
    options: Options,
) -> Result<AccountCoinsResponse, ApiError> {
    debug!("/account/coins");

    let _ = require_online_mode(&options)?;

    is_bad_network(&options, &account_coins_request.network_identifier)?;

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
