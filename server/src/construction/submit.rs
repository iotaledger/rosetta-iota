// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{build_iota_client, construction::deserialize_signed_transaction, error::ApiError, is_wrong_network, types::*, Options, is_offline_mode_enabled};

use bee_message::prelude::*;

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionSubmitRequest {
    pub network_identifier: NetworkIdentifier,
    pub signed_transaction: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionSubmitResponse {
    pub transaction_identifier: TransactionIdentifier,
    pub metadata: ConstructionSubmitResponseMetadata,
}

pub(crate) async fn construction_submit_request(
    request: ConstructionSubmitRequest,
    options: Options,
) -> Result<ConstructionSubmitResponse, ApiError> {
    debug!("/construction/submit");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::BadNetwork)
    }

    if is_offline_mode_enabled(&options) {
        return Err(ApiError::UnavailableOffline)
    }

    let iota_client = build_iota_client(&options).await?;

    let signed_transaction = deserialize_signed_transaction(&request.signed_transaction);
    let transaction = signed_transaction.transaction();
    let transaction_id = transaction.id();

    let message = iota_client
        .message()
        .finish_message(Some(Payload::Transaction(Box::new(transaction.clone()))))
        .await?;

    match iota_client.post_message(&message).await {
        Ok(message_id) => Ok(ConstructionSubmitResponse {
            transaction_identifier: TransactionIdentifier {
                hash: transaction_id.to_string(),
            },
            metadata: ConstructionSubmitResponseMetadata {
                message_id: message_id.to_string(),
            },
        }),
        Err(_) => Err(ApiError::BadConstructionRequest("can not submit message".to_string())),
    }
}
