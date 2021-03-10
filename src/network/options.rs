// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{consts, error::ApiError, types::*, operations::*, options::Options,
 is_bad_network};
use serde::{Deserialize, Serialize};

use log::debug;
use crate::types::{NetworkIdentifier};

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
    network_request: NetworkOptionsRequest,
    options: Options,
) -> Result<NetworkOptionsResponse, ApiError> {
    debug!("/network/options");

    // todo: double check if this is really necessary
    // let _ = require_offline_mode(&options)?;

    is_bad_network(&options, &network_request.network_identifier)?;

    let version = Version {
        rosetta_version: consts::ROSETTA_VERSION.to_string(),
        node_version: consts::NODE_VERSION.to_string(),
        middleware_version: consts::MIDDLEWARE_VERSION.to_string(),
    };

    let operation_statuses = vec![
        OperationStatus {
            status: operation_status_success(),
            successful: true
        },
        OperationStatus {
            status: operation_status_skipped(),
            successful: false
        }
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
    };

    let response = NetworkOptionsResponse { version, allow };

    Ok(response)
}