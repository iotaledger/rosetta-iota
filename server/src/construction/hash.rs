// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{construction::deserialize_signed_transaction, error::ApiError, is_wrong_network, types::*, Config};

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
    request: ConstructionHashRequest,
    options: Config,
) -> Result<ConstructionHashResponse, ApiError> {
    debug!("/construction/hash");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()));
    }

    let signed_transaction = deserialize_signed_transaction(&request.signed_transaction);

    Ok(ConstructionHashResponse {
        transaction_identifier: TransactionIdentifier {
            hash: signed_transaction.transaction().id().to_string(),
        },
    })
}
