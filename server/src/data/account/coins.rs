// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{currency::iota_currency, error::ApiError, is_wrong_network, config::Config, types::{AccountIdentifier, NetworkIdentifier, *}, is_offline_mode_enabled};

use bee_rest_api::types::dtos::{AddressDto, OutputDto};

use log::debug;
use serde::{Deserialize, Serialize};
use crate::client::{build_client, get_outputs_of_address, get_confirmed_milestone_index, get_milestone};
use bee_rest_api::types::responses::OutputResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCoinsRequest {
    pub network_identifier: NetworkIdentifier,
    pub account_identifier: AccountIdentifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCoinsResponse {
    pub block_identifier: BlockIdentifier,
    pub coins: Vec<Coin>,
}

pub async fn account_coins(
    request: AccountCoinsRequest,
    options: Config,
) -> Result<AccountCoinsResponse, ApiError> {
    debug!("/account/coins");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()))
    }

    if is_offline_mode_enabled(&options) {
        return Err(ApiError::NonRetriable("endpoint is not available in offline mode".to_string()))
    }

    let (outputs, milestone) = outputs_of_address_at_milestone(&request.account_identifier.address, &options).await?;

    let mut coins = Vec::new();
    for output_res in outputs {
        let amount = match output_res.output {
            OutputDto::Treasury(_) => return Err(ApiError::NonRetriable("treasury output can not be used to feed a transaction".to_string())),
            OutputDto::SignatureLockedSingle(r) => match r.address {
                AddressDto::Ed25519(_) => r.amount,
            },
            OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                AddressDto::Ed25519(_) => r.amount,
            },
        };

        let output_id = format!("{}{}", output_res.transaction_id, hex::encode(output_res.output_index.to_le_bytes()));

        coins.push(Coin {
            coin_identifier: CoinIdentifier {
                identifier: output_id,
            },
            amount: Amount {
                value: amount.to_string(),
                currency: iota_currency(),
                metadata: None,
            },
        });
    }

    let response = AccountCoinsResponse {
        block_identifier: BlockIdentifier {
            index: milestone.index,
            hash: milestone.message_id.to_string(),
        },
        coins,
    };

    Ok(response)
}

async fn outputs_of_address_at_milestone(address: &str, options: &Config) -> Result<(Vec<OutputResponse>, iota::MilestoneResponse), ApiError> {
    let client = build_client(options).await?;

    // to make sure the outputs of an address do not change in the meantime, check the index of the confirmed
    // milestone before and after performing the request

    let index_before = get_confirmed_milestone_index(&client).await?;
    let outputs = get_outputs_of_address(&address, &client).await?;
    let index_after = get_confirmed_milestone_index(&client).await?;

    if index_before != index_after {
        return Err(ApiError::Retriable("confirmed milestone changed while performing the request".to_string()));
    }

    let milestone = get_milestone(index_before, &client).await?;

    Ok((outputs, milestone))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RosettaMode;

    #[tokio::test]
    async fn test_coins() {
        let request = AccountCoinsRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet7".to_string(),
                sub_network_identifier: None,
            },
            account_identifier: AccountIdentifier {
                address: String::from("atoi1qzgrk7whadapf4qw5sqvlxkrr0ve3nv09xgdfyc09gfp3e2369ghsj5g2rf"),
                sub_account: None,
            },
        };

        let server_options = Config {
            node: "https://api.hornet-rosetta.testnet.chrysalis2.com".to_string(),
            network: "testnet7".to_string(),
            tx_indexation: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };

        let _response = account_coins(request, server_options).await.unwrap();
        // todo: assertions
    }
}
