// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::Config,
    currency::iota_currency,
    error::ApiError,
    is_offline_mode_enabled, is_wrong_network,
    types::{AccountIdentifier, NetworkIdentifier, *},
};
use crate::client::{build_client, get_unspent_outputs_of_address, get_output};

use bee_message::milestone::MilestoneIndex;
use bee_message::output::OutputId;
use bee_rest_api::types::dtos::{AddressDto, OutputDto};
use bee_rest_api::types::responses::{OutputResponse, OutputsAddressResponse};

use iota_client::Client;

use log::debug;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

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

pub async fn account_coins(request: AccountCoinsRequest, options: Config) -> Result<AccountCoinsResponse, ApiError> {
    debug!("/account/coins");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()));
    }

    if is_offline_mode_enabled(&options) {
        return Err(ApiError::NonRetriable(
            "endpoint is not available in offline mode".to_string(),
        ));
    }

    let (outputs, ledger_index) = address_outputs_with_ledger_index(&request.account_identifier.address, &options).await?;

    let mut coins = Vec::new();
    for (output_id, output_response) in outputs {
        let amount = match output_response.output {
            OutputDto::SignatureLockedSingle(r) => match r.address {
                AddressDto::Ed25519(_) => r.amount,
            },
            OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                AddressDto::Ed25519(_) => r.amount,
            },
            _ => unimplemented!()
        };

        coins.push(Coin {
            coin_identifier: CoinIdentifier { identifier: output_id.to_string() },
            amount: Amount {
                value: amount.to_string(),
                currency: iota_currency(),
                metadata: None,
            },
        });
    }

    Ok(AccountCoinsResponse {
        block_identifier: BlockIdentifier {
            index: *ledger_index,
            hash: (*ledger_index).to_string(),
        },
        coins,
    })
}

async fn address_outputs_with_ledger_index(
    address: &str,
    options: &Config,
) -> Result<(HashMap<OutputId, OutputResponse>, MilestoneIndex), ApiError> {
    let client: Client = build_client(options).await?;

    loop {

        let outputs_address_response: OutputsAddressResponse = get_unspent_outputs_of_address(&address, &client).await?;
        let mut output_responses = HashMap::new();

        let mut try_again = false;
        for id in outputs_address_response.output_ids {
            let output_id = id
                .parse::<OutputId>()
                .map_err(|e| ApiError::NonRetriable(format!("can not parse output id: {}", e)))?;

            let output_response = get_output(output_id, &client).await?;

            // if the output was spent in the meantime, retry
            if output_response.is_spent {
                try_again = true;
                break;
            } else {
                output_responses.insert(output_id, output_response);
            }
        }

        if try_again {
            continue
        } else {
            break Ok((output_responses, MilestoneIndex(outputs_address_response.ledger_index)))
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::RosettaMode, mocked_node::start_mocked_node};
    use serial_test::serial;
    use tokio::sync::oneshot;

    #[tokio::test]
    #[serial]
    async fn test_coins() {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        tokio::task::spawn(start_mocked_node(shutdown_rx));

        let request = AccountCoinsRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet7".to_string(),
                sub_network_identifier: None,
            },
            account_identifier: AccountIdentifier {
                address: String::from("atoi1qppx6868hzy497e3yamzxj3dp4ameljlh4x6ac7sdrrtg25fnk2tjlpxcek"),
                sub_account: None,
            },
        };

        let server_options = Config {
            node_url: "http://127.0.0.1:3029".to_string(),
            network: "testnet7".to_string(),
            tx_tag: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };

        let response = account_coins(request, server_options).await.unwrap();

        assert_eq!(68910, response.block_identifier.index);
        assert_eq!(
            "68910",
            response.block_identifier.hash
        );
        assert_eq!(1, response.coins.len());
        assert_eq!("10000000", response.coins[0].amount.value);
        assert_eq!(
            "f3a53f04402be2f59634ee9b073898c84d2e08b4ba06046d440b1ac27bc5ded60000",
            response.coins[0].coin_identifier.identifier
        );

        let _ = shutdown_tx.send(());
    }
}
