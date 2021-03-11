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

    Ok(ConstructionPreprocessResponse {
        options: None
    })
}