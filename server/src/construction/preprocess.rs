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
            "UTXO_INPUT" => {
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
