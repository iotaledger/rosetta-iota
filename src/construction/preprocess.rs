// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options, is_bad_network, require_offline_mode};
use crate::error::ApiError;

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionPreprocessRequest {
    pub network_identifier: NetworkIdentifier,
    pub operations: Vec<Operation>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionPreprocessResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<PreprocessOptions>,
}

pub async fn construction_preprocess_request(
    construction_preprocess_request: ConstructionPreprocessRequest,
    options: Options,
) -> Result<ConstructionPreprocessResponse, ApiError> {
    debug!("/construction/preprocess");

    let _ = require_offline_mode(&options)?;
    is_bad_network(&options, &construction_preprocess_request.network_identifier)?;

    let mut utxo_inputs = Vec::new();
    for operation in construction_preprocess_request.operations {
        match &operation.type_[..] {
            "UTXO_INPUT" => {
                let output_id = operation.coin_change.ok_or(ApiError::BadConstructionRequest("coin_change not set".to_string()))?.coin_identifier.identifier;
                utxo_inputs.push(output_id);
            }
            _ => continue
        }
    }

    Ok(ConstructionPreprocessResponse {
        options: Some(PreprocessOptions {
            utxo_inputs
        })
    })
}