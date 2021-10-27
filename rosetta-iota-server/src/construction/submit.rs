// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    construction::deserialize_signed_transaction, error::ApiError, is_offline_mode_enabled, is_wrong_network, types::*,
    RosettaConfig,
};

use bee_message::prelude::*;

use crate::client::build_client;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionSubmitRequest {
    pub network_identifier: NetworkIdentifier,
    pub signed_transaction: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionSubmitResponse {
    pub transaction_identifier: TransactionIdentifier,
    pub metadata: ConstructionSubmitResponseMetadata,
}

pub async fn submit(
    request: ConstructionSubmitRequest,
    rosetta_config: RosettaConfig,
) -> Result<ConstructionSubmitResponse, ApiError> {
    debug!("/construction/submit");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()));
    }

    if is_offline_mode_enabled(&rosetta_config) {
        return Err(ApiError::NonRetriable(
            "endpoint is not available in offline mode".to_string(),
        ));
    }

    let client = build_client(&rosetta_config).await?;

    let signed_transaction = deserialize_signed_transaction(&request.signed_transaction);
    let transaction = signed_transaction.transaction();

    let message = client
        .message()
        .finish_message(Some(Payload::Transaction(Box::new(transaction.clone()))))
        .await
        .map_err(|e| ApiError::NonRetriable(format!("can not build message: {}", e)))?;

    match client.post_message(&message).await {
        Ok(message_id) => Ok(ConstructionSubmitResponse {
            transaction_identifier: TransactionIdentifier {
                hash: transaction.id().to_string(),
            },
            metadata: ConstructionSubmitResponseMetadata {
                message_id: message_id.to_string(),
            },
        }),

        Err(e) => Err(ApiError::NonRetriable(format!("can not submit message: {}", e))),
    }
}
