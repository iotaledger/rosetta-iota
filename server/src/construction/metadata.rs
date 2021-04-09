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
        return Err(ApiError::BadNetwork)
    }

    if is_offline_mode_enabled(&options) {
        return Err(ApiError::UnavailableOffline)
    }

    let iota_client = build_iota_client(&options).await?;

    let mut utxo_inputs_metadata = HashMap::new();
    for input_id in request.options.inputs {
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
