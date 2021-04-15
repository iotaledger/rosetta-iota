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
    use crate::mocknet::start_mocknet_node;

    #[tokio::test]
    async fn test_network_options() {
        tokio::task::spawn(start_mocknet_node());

        let request = NetworkOptionsRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet7".to_string(),
                sub_network_identifier: None,
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

        let response = network_options(request, server_options).await.unwrap();

        assert_eq!("1.4.10", response.version.rosetta_version);
        assert_eq!("0.6.0-alpha", response.version.node_version);
        assert_eq!("0.6.0-alpha", response.version.middleware_version);

        assert_eq!("Success", response.allow.operation_statuses[0].status);
        assert_eq!(true, response.allow.operation_statuses[0].successful);

        assert_eq!("UTXO_INPUT", response.allow.operation_types[0]);
        assert_eq!("UTXO_OUTPUT", response.allow.operation_types[1]);
        assert_eq!("DUST_ALLOWANCE_OUTPUT", response.allow.operation_types[2]);

        assert_eq!(1, response.allow.errors[0].code);
        assert_eq!("non retriable error", response.allow.errors[0].message);
        assert_eq!(false, response.allow.errors[0].retriable);
        assert_eq!(false, response.allow.errors[0].details.is_some());
    }
}
