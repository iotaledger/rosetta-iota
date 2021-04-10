// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{build_iota_client, currency::iota_currency, error::ApiError, is_wrong_network, options::Options, types::{AccountIdentifier, NetworkIdentifier, *}, is_offline_mode_enabled};

use bee_rest_api::types::dtos::{AddressDto, OutputDto};

use log::debug;
use serde::{Deserialize, Serialize};

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
    options: Options,
) -> Result<AccountCoinsResponse, ApiError> {
    debug!("/account/coins");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()))
    }

    if is_offline_mode_enabled(&options) {
        return Err(ApiError::NonRetriable("endpoint is not available in offline mode".to_string()))
    }

    let iota_client = build_iota_client(&options).await?;

    let node_info = match iota_client.get_info().await {
        Ok(node_info) => node_info,
        Err(e) => return Err(ApiError::NonRetriable(format!("unable to get node info: {}", e))),
    };

    let confirmed_milestone = match iota_client.get_milestone(node_info.confirmed_milestone_index).await {
        Ok(confirmed_milestone) => confirmed_milestone,
        Err(e) => return Err(ApiError::NonRetriable(format!("unable to get milestone: {}", e))),
    };

    let address = request.account_identifier.address;
    let outputs = match iota_client.find_outputs(&[], &[address.clone()]).await {
        Ok(outputs) => outputs,
        Err(e) => return Err(ApiError::NonRetriable(format!("unable to outputs from address: {}", e))),
    };

    let mut coins = Vec::new();
    for output_info in outputs {
        let amount = match output_info.output {
            OutputDto::Treasury(_) => panic!("Can't be used as input"),
            OutputDto::SignatureLockedSingle(r) => match r.address {
                AddressDto::Ed25519(_) => r.amount,
            },
            OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                AddressDto::Ed25519(_) => r.amount,
            },
        };

        let output_id = format!("{}{}", output_info.transaction_id, hex::encode(output_info.output_index.to_le_bytes()));

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
            index: confirmed_milestone.index,
            hash: confirmed_milestone.message_id.to_string(),
        },
        coins,
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::RosettaMode;

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

        let server_options = Options {
            node: "https://api.hornet-rosetta.testnet.chrysalis2.com".to_string(),
            network: "testnet7".to_string(),
            indexation: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };

        let _response = account_coins(request, server_options).await.unwrap();
        // todo: assertions
    }
}
