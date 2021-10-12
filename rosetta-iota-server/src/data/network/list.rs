// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{config::RosettaConfig, consts, error::ApiError, filters::EmptyRequest, types::NetworkIdentifier};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}

pub async fn network_list(_empty: EmptyRequest, options: RosettaConfig) -> Result<NetworkListResponse, ApiError> {
    debug!("/network/list");

    let response = NetworkListResponse {
        network_identifiers: vec![NetworkIdentifier {
            blockchain: consts::BLOCKCHAIN.to_string(),
            network: options.network.clone(),
            sub_network_identifier: None,
        }],
    };

    Ok(response)
}