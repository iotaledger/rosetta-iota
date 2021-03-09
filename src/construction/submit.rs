// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options, is_bad_network, build_iota_client, require_online_mode};
use crate::error::ApiError;
use crate::construction::transaction_from_hex_string;

use bee_message::prelude::*;

use log::debug;

pub(crate) async fn construction_submit_request(
    construction_submit_request: ConstructionSubmitRequest,
    options: Options,
) -> Result<ConstructionSubmitResponse, ApiError> {
    debug!("/construction/submit");

    let _ = require_online_mode(&options)?;

    is_bad_network(&options, &construction_submit_request.network_identifier)?;

    let iota_client = build_iota_client(&options, true).await?;

    let transaction = transaction_from_hex_string(&construction_submit_request.signed_transaction)?;
    let transaction_id = transaction.id();

    let message = iota_client
        .message()
        .finish_message(Some(Payload::Transaction(Box::new(transaction))))
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