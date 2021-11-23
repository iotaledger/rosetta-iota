// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::RosettaConfig,
    consts::iota_currency,
    error::ApiError,
    is_offline_mode_enabled, is_wrong_network,
    types::{AccountIdentifier, Amount, BlockIdentifier, NetworkIdentifier},
};
use crate::client::{build_client, get_balance_of_address};
use crate::types::Currency;

use bee_message::milestone::MilestoneIndex;

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccountBalanceRequest {
    pub network_identifier: NetworkIdentifier,
    pub account_identifier: AccountIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<Currency>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccountBalanceResponse {
    pub block_identifier: BlockIdentifier,
    pub balances: Vec<Amount>,
}

pub async fn account_balance(
    request: AccountBalanceRequest,
    rosetta_config: RosettaConfig,
) -> Result<AccountBalanceResponse, ApiError> {
    debug!("/account/balance");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()));
    }

    if is_offline_mode_enabled(&rosetta_config) {
        return Err(ApiError::NonRetriable(
            "endpoint does not support offline mode".to_string(),
        ));
    }

    // only support IOTA currency
    if let Some(currencies) = request.currencies {
        for currency in currencies {
            if !currency.eq(&iota_currency()) {
                return Err(ApiError::NonRetriable(
                    "invalid currency provided: only `IOTA` currency supported".to_string()
                ));
            }
        }
    }

    let (amount, ledger_index) = address_balance_with_ledger_index(&request.account_identifier.address, &rosetta_config).await?;

    Ok(AccountBalanceResponse {
        block_identifier: BlockIdentifier {
            index: *ledger_index,
            hash: (*ledger_index).to_string(),
        },
        balances: vec![amount],
    })
}

async fn address_balance_with_ledger_index(address: &str, options: &RosettaConfig) -> Result<(Amount, MilestoneIndex), ApiError> {
    let client = build_client(options).await?;

    let balance_response = get_balance_of_address(address, &client).await?;

    let amount = Amount {
        value: balance_response.balance.to_string(),
        currency: iota_currency(),
    };

    Ok((amount, MilestoneIndex(balance_response.ledger_index)))
}