// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::RosettaConfig,
    consts,
    error::ApiError,
    is_wrong_network,
    operations::*,
    types::{NetworkIdentifier, *},
};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkOptionsRequest {
    pub network_identifier: NetworkIdentifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkOptionsResponse {
    pub version: Version,
    pub allow: Allow,
}

pub async fn network_options(
    request: NetworkOptionsRequest,
    rosetta_config: RosettaConfig,
) -> Result<NetworkOptionsResponse, ApiError> {
    debug!("/network/options");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()));
    }

    let version = Version {
        rosetta_version: consts::ROSETTA_VERSION.to_string(),
        node_version: consts::NODE_VERSION.to_string(),
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
        call_methods: vec![],
        balance_exemptions: vec![],
        mempool_coins: false,
    };

    let response = NetworkOptionsResponse { version, allow };

    Ok(response)
}
