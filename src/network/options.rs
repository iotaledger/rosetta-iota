// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{consts, error::ApiError, operations::*, options::Options, types::{
    Allow, NetworkOptionsResponse, NetworkRequest,
    Version,
}, is_bad_network};

use log::debug;

pub async fn network_options(
    network_request: NetworkRequest,
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

    let mut operation_statuses = Vec::new();
    operation_statuses.push(operation_status_success());
    operation_statuses.push(operation_status_skipped());

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