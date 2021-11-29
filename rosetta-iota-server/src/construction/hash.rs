// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{construction::deserialize_signed_transaction, error::ApiError, is_wrong_network, types::*, RosettaConfig};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionHashRequest {
    pub network_identifier: NetworkIdentifier,
    pub signed_transaction: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionHashResponse {
    pub transaction_identifier: TransactionIdentifier,
}

pub async fn hash(
    request: ConstructionHashRequest,
    rosetta_config: RosettaConfig,
) -> Result<ConstructionHashResponse, ApiError> {
    debug!("/construction/hash");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()));
    }

    let signed_transaction = deserialize_signed_transaction(&request.signed_transaction);

    Ok(ConstructionHashResponse {
        transaction_identifier: TransactionIdentifier {
            hash: signed_transaction.transaction().id().to_string(),
        },
    })
}
