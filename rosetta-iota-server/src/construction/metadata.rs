// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, is_offline_mode_enabled, is_wrong_network, types::*, RosettaConfig};

use bee_message::prelude::*;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::client::{build_client, get_output};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionMetadataRequest {
    pub network_identifier: NetworkIdentifier,
    pub options: PreprocessOptions,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionMetadataResponse {
    pub metadata: ConstructionMetadata,
}

pub async fn metadata(
    request: ConstructionMetadataRequest,
    rosetta_config: RosettaConfig,
) -> Result<ConstructionMetadataResponse, ApiError> {
    debug!("/construction/metadata");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()));
    }

    if is_offline_mode_enabled(&rosetta_config) {
        return Err(ApiError::NonRetriable(
            "endpoint is not available in offline mode".to_string(),
        ));
    }

    let client = build_client(&rosetta_config).await?;

    let mut utxo_inputs_metadata = HashMap::new();
    for output_id_string in request.options.utxo_inputs {
        let output_id = output_id_string
            .parse::<OutputId>()
            .map_err(|e| ApiError::NonRetriable(format!("can not parse output id: {}", e)))?;

        let output = get_output(output_id, &client).await?;

        utxo_inputs_metadata.insert(output_id_string, output);
    }

    Ok(ConstructionMetadataResponse {
        metadata: ConstructionMetadata { utxo_inputs_metadata },
    })
}
