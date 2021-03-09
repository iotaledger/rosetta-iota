// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options, is_bad_network, require_online_mode};
use crate::error::ApiError;

use log::debug;

pub(crate) async fn construction_metadata_request(
    construction_metadata_request: ConstructionMetadataRequest,
    options: Options,
) -> Result<ConstructionMetadataResponse, ApiError> {
    debug!("/construction/metadata");

    let _ = require_online_mode(&options)?;

    is_bad_network(&options, &construction_metadata_request.network_identifier)?;

    Ok(ConstructionMetadataResponse {
        metadata: ConstructionMetadata {}
    })
}