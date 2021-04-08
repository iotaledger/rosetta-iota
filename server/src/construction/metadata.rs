// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{build_iota_client, error::ApiError, is_wrong_network, require_online_mode, types::*, Options};

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
    construction_metadata_request: ConstructionMetadataRequest,
    options: Options,
) -> Result<ConstructionMetadataResponse, ApiError> {
    debug!("/construction/metadata");

    let _ = require_online_mode(&options)?;
    is_wrong_network(&options, &construction_metadata_request.network_identifier)?;

    let iota_client = build_iota_client(&options).await?;

    let mut utxo_inputs_metadata = HashMap::new();
    for input_id in construction_metadata_request.options.inputs {
        let input = input_id
            .parse::<UtxoInput>()
            .map_err(|_| ApiError::BadConstructionRequest("can not parse input".to_string()))?;
        let input_metadata = iota_client
            .get_output(&input)
            .await
            .map_err(|e| ApiError::IotaClientError(e))?;
        utxo_inputs_metadata.insert(input_id, input_metadata);
    }

    Ok(ConstructionMetadataResponse {
        metadata: ConstructionMetadata {
            inputs_metadata: utxo_inputs_metadata,
        },
    })
}
