// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{consts, error::ApiError, filters::{ EmptyRequest}, options::Options, types::{
    NetworkIdentifier,

}};
use serde::{Deserialize, Serialize};
use log::debug;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}

pub async fn network_list(_empty: EmptyRequest, options: Options) -> Result<NetworkListResponse, ApiError> {
    debug!("/network/list");

    // todo: double check if this is really necessary
    // let _ = require_offline_mode(&options)?;

    let response = NetworkListResponse {
        network_identifiers: vec![NetworkIdentifier {
            blockchain: consts::BLOCKCHAIN.to_string(),
            network: options.network.clone(),
            sub_network_identifier: None,
        }],
    };

    Ok(response)
}