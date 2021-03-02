// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options, is_bad_network};
use crate::error::ApiError;

use log::debug;

pub(crate) async fn construction_preprocess_request(
    construction_preprocess_request: ConstructionPreprocessRequest,
    options: Options,
) -> Result<ConstructionPreprocessResponse, ApiError> {
    debug!("/construction/preprocess");

    is_bad_network(&options, &construction_preprocess_request.network_identifier)?;

    Ok(ConstructionPreprocessResponse {
        options: None
    })
}