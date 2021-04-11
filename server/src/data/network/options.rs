// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{consts, error::ApiError, is_wrong_network, operations::*, config::Config, types::{NetworkIdentifier, *}};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkOptionsRequest {
    pub network_identifier: NetworkIdentifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkOptionsResponse {
    pub version: Version,
    pub allow: Allow,
}

pub async fn network_options(
    request: NetworkOptionsRequest,
    options: Config,
) -> Result<NetworkOptionsResponse, ApiError> {
    debug!("/network/options");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()))
    }

    let version = Version {
        rosetta_version: consts::ROSETTA_VERSION.to_string(),
        node_version: consts::NODE_VERSION.to_string(),
        middleware_version: consts::MIDDLEWARE_VERSION.to_string(),
    };

    let operation_statuses = vec![
        OperationStatus {
            status: operation_status_success(),
            successful: true,
        },
        OperationStatus {
            status: operation_status_skipped(),
            successful: false,
        },
    ];

    let operation_types = operation_type_list();

    let errors = ApiError::all_errors();

    let allow = Allow {
        operation_statuses,
        operation_types,
        errors,
        historical_balance_lookup: false,
        timestamp_start_index: Some(0),
        call_methods: vec![],
        balance_exemptions: vec![],
        mempool_coins: false,
    };

    let response = NetworkOptionsResponse { version, allow };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RosettaMode;

    #[tokio::test]
    async fn test_network_options() {
        let request = NetworkOptionsRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet6".to_string(),
                sub_network_identifier: None,
            },
        };

        let server_options = Config {
            node: "https://api.hornet-rosetta.testnet.chrysalis2.com".to_string(),
            network: "testnet6".to_string(),
            tx_indexation: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };
        let _response = network_options(request, server_options).await.unwrap();

        // todo: assertions
    }
}
