// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, is_wrong_network, types::*, Config};

use bee_message::prelude::*;

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionPreprocessRequest {
    pub network_identifier: NetworkIdentifier,
    pub operations: Vec<Operation>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionPreprocessResponse {
    pub options: PreprocessOptions,
}

pub async fn construction_preprocess_request(
    request: ConstructionPreprocessRequest,
    options: Config,
) -> Result<ConstructionPreprocessResponse, ApiError> {
    debug!("/construction/preprocess");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()))
    }

    let mut utxo_inputs = Vec::new();
    for operation in request.operations {
        match &operation.type_[..] {
            "INPUT" => {
                let coin_change = operation.coin_change.ok_or(ApiError::NonRetriable("coin change not populated".to_string()))?;
                let output_id = coin_change
                    .coin_identifier
                    .identifier
                    .parse::<OutputId>()
                    .map_err(|e| ApiError::NonRetriable(format!("can not parse output id from coin identifier: {}", e)))?;
                utxo_inputs.push(output_id.to_string());
            }
            _ => continue,
        }
    }

    Ok(ConstructionPreprocessResponse {
        options: PreprocessOptions {
            utxo_inputs,
        },
    })
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::config::RosettaMode;

    #[tokio::test]
    async fn test_preprocess() {

        let data = r#"{"network_identifier":{"blockchain":"iota","network":"testnet7"},"operations":[{"operation_identifier":{"index":0,"network_index":0},"type":"INPUT","account":{"address":"atoi1qr49znuapruu3fhwcfd4vsq2y3a0l9k8zc6pv6ak70g4hd9jq8fr2lqf6et"},"amount":{"value":"-10000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"8bec7fd0a9fdc351adaaf07f595afefa7844eafd183625949e51dcb3b9632b890000"},"coin_action":"coin_spent"}},{"operation_identifier":{"index":1},"type":"SIG_LOCKED_SINGLE_OUTPUT","account":{"address":"atoi1qpmppfmvwlg5qjkwd8084ceh0emw6y9gegpmesn2vvrlacfep834wyqsxww"},"amount":{"value":"8604736","currency":{"symbol":"IOTA","decimals":0}}},{"operation_identifier":{"index":2},"type":"SIG_LOCKED_SINGLE_OUTPUT","account":{"address":"atoi1qp08ypmqn53kxxmj7d60wqp6hwtcc25sv8y950j7e35fjnj3dmpxyp7l5y9"},"amount":{"value":"395264","currency":{"symbol":"IOTA","decimals":0}}},{"operation_identifier":{"index":3},"type":"DUST_ALLOWANCE_OUTPUT","account":{"address":"atoi1qp08ypmqn53kxxmj7d60wqp6hwtcc25sv8y950j7e35fjnj3dmpxyp7l5y9"},"amount":{"value":"1000000","currency":{"symbol":"IOTA","decimals":0}}}]}"#;
        let request: ConstructionPreprocessRequest = serde_json::from_str(data).unwrap();

        let server_options = Config {
            node_url: "http://127.0.0.1:3029".to_string(),
            network: "testnet7".to_string(),
            tx_tag: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };

        let response = construction_preprocess_request(request, server_options).await.unwrap();

        assert_eq!(
            1,
            response.options.utxo_inputs.len()
        );

        assert_eq!(
            "8bec7fd0a9fdc351adaaf07f595afefa7844eafd183625949e51dcb3b9632b890000",
            response.options.utxo_inputs[0]
        )

    }
}

