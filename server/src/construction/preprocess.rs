// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, is_bad_network, require_offline_mode, types::*, Options};

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
    construction_preprocess_request: ConstructionPreprocessRequest,
    options: Options,
) -> Result<ConstructionPreprocessResponse, ApiError> {
    debug!("/construction/preprocess");

    let _ = require_offline_mode(&options)?;
    is_bad_network(&options, &construction_preprocess_request.network_identifier)?;

    let mut transaction_inputs = Vec::new();
    for operation in construction_preprocess_request.operations {
        match &operation.type_[..] {
            "UTXO_INPUT" => {
                let coin_change = operation.coin_change.ok_or(ApiError::BadConstructionRequest(
                    "coin_change not populated for UTXO_INPUT".to_string(),
                ))?;
                let output_id = coin_change
                    .coin_identifier
                    .identifier
                    .parse::<OutputId>()
                    .map_err(|_| ApiError::BadConstructionRequest("invalid output id".to_string()))?;
                transaction_inputs.push(output_id.to_string());
            }
            _ => continue,
        }
    }

    Ok(ConstructionPreprocessResponse {
        options: PreprocessOptions {
            inputs: transaction_inputs,
        },
    })
}
