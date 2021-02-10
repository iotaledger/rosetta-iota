// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    error::ApiError,
    filters::{handle, with_options},
    options::Options,
    types::{
        ConstructionHashRequest, ConstructionHashResponse, ConstructionSubmitRequest, ConstructionSubmitResponse,
        TransactionIdentifier,
    },
};
use bee_common::packable::Packable;
use iota::Message;
use log::debug;
use warp::Filter;

pub fn routes(options: Options) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(
            warp::path!("construction" / "hash")
                .and(warp::body::json())
                .and(with_options(options.clone()))
                .and_then(handle(construction_hash_request)),
        )
        .or(warp::path!("construction" / "submit")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(construction_submit_request)))
}

async fn construction_hash_request(
    construction_hash_request: ConstructionHashRequest,
    options: Options,
) -> Result<ConstructionHashResponse, ApiError> {
    debug!("/construction/hash");
    let message = message_from_hex(&construction_hash_request.signed_transaction)?;
    Ok(ConstructionHashResponse {
        transaction_identifier: TransactionIdentifier {
            hash: message.id().0.to_string(),
        },
    })
}

async fn construction_submit_request(
    construction_submit_request: ConstructionSubmitRequest,
    options: Options,
) -> Result<ConstructionSubmitResponse, ApiError> {
    debug!("/construction/submit");
    let message = message_from_hex(&construction_submit_request.signed_transaction)?;

    let iota_client = match iota::Client::builder()
        .with_network(&options.network)
        .with_node(&options.iota_endpoint)
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
    {
        Ok(iota_client) => iota_client,
        Err(_) => return Err(ApiError::UnableToBuildClient),
    };

    match iota_client.post_message(&message).await {
        Ok(message_id) => Ok(ConstructionSubmitResponse {
            transaction_identifier: TransactionIdentifier {
                hash: message_id.to_string(),
            },
        }),
        Err(_) => Err(ApiError::BadConstructionRequest("can not submit message".to_string())),
    }
}

fn message_from_hex(hex_str: &str) -> Result<Message, ApiError> {
    match hex::decode(hex_str) {
        Ok(bytes) => Ok(Message::unpack(&mut bytes.as_slice())
            .map_err(|_| ApiError::BadConstructionRequest("can not build message from hex string".to_string()))?),
        Err(e) => Err(ApiError::BadConstructionRequest(
            "can not build message from hex string".to_string(),
        )),
    }
}
