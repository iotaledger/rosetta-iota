// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{build_iota_client, error::ApiError, is_wrong_network, types::*, Options, is_offline_mode_enabled};

use bee_message::prelude::*;

use log::debug;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionMetadataRequest {
    pub network_identifier: NetworkIdentifier,
    pub options: PreprocessOptions,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionMetadataResponse {
    pub metadata: ConstructionMetadata,
}

pub(crate) async fn construction_metadata_request(
    request: ConstructionMetadataRequest,
    options: Options,
) -> Result<ConstructionMetadataResponse, ApiError> {
    debug!("/construction/metadata");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()))
    }

    if is_offline_mode_enabled(&options) {
        return Err(ApiError::NonRetriable("endpoint is not available in offline mode".to_string()))
    }

    let iota_client = build_iota_client(&options).await?;

    let mut utxo_inputs_metadata = HashMap::new();
    for output_id_string in request.options.utxo_inputs {
        let output_id = output_id_string
            .parse::<OutputId>()
            .map_err(|e| ApiError::NonRetriable(format!("can not parse output id: {}", e)))?;

        let output_metadata = iota_client
            .get_output(&(output_id.into()))
            .await
            .map_err(|e| ApiError::NonRetriable(format!("can not get output: {}", e)))?;

        utxo_inputs_metadata.insert(output_id_string, output_metadata);
    }

    Ok(ConstructionMetadataResponse {
        metadata: ConstructionMetadata {
            utxo_inputs_metadata,
        },
    })
}
