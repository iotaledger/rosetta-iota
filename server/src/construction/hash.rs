// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    construction::deserialize_signed_transaction, error::ApiError, is_bad_network, require_offline_mode, types::*,
    Options,
};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionHashRequest {
    pub network_identifier: NetworkIdentifier,
    pub signed_transaction: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionHashResponse {
    pub transaction_identifier: TransactionIdentifier,
}

pub(crate) async fn construction_hash_request(
    construction_hash_request: ConstructionHashRequest,
    options: Options,
) -> Result<ConstructionHashResponse, ApiError> {
    debug!("/construction/hash");

    let _ = require_offline_mode(&options)?;

    is_bad_network(&options, &construction_hash_request.network_identifier)?;

    let signed_transaction = deserialize_signed_transaction(&construction_hash_request.signed_transaction);

    Ok(ConstructionHashResponse {
        transaction_identifier: TransactionIdentifier {
            hash: signed_transaction.transaction().id().to_string(),
        },
    })
}
