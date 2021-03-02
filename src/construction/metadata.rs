// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options, is_bad_network, consts};
use crate::error::ApiError;

use log::debug;

pub(crate) async fn construction_metadata_request(
    construction_metadata_request: ConstructionMetadataRequest,
    options: Options,
) -> Result<ConstructionMetadataResponse, ApiError> {
    debug!("/construction/metadata");

    is_bad_network(&options, &construction_metadata_request.network_identifier)?;

    if options.mode != consts::ONLINE_MODE {
        return Err(ApiError::UnavailableOffline);
    }

    Ok(ConstructionMetadataResponse {
        metadata: ConstructionMetadata {}
    })
}