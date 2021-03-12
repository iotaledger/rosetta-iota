// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options, is_bad_network, build_iota_client, require_online_mode};
use crate::error::ApiError;


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
    construction_submit_request: ConstructionSubmitRequest,
    options: Options,
) -> Result<ConstructionSubmitResponse, ApiError> {
    debug!("/construction/submit");

    let _ = require_online_mode(&options)?;

    is_bad_network(&options, &construction_submit_request.network_identifier)?;

    let iota_client = build_iota_client(&options).await?;

    let signed_transaction_decoded = hex::decode(construction_submit_request.signed_transaction)?;
    let signed_transaction: SignedTransaction = serde_json::from_slice(&signed_transaction_decoded).unwrap();
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
            metadata: ConstructionSubmitResponseMetadata { message_id: message_id.to_string() }
        }),
        Err(_) => Err(ApiError::BadConstructionRequest("can not submit message".to_string())),
    }
}