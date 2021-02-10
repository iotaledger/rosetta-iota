// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    error::ApiError,
    filters::{handle, with_options},
    options::Options,
    types::{ConstructionHashRequest, ConstructionHashResponse, TransactionIdentifier},
};
use bee_common::packable::Packable;
use iota::Message;
use log::debug;
use warp::Filter;

pub fn routes(options: Options) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post().and(
        warp::path!("construction" / "hash")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(construction_hash_request)),
    )
}

async fn construction_hash_request(
    construction_hash_request: ConstructionHashRequest,
    options: Options,
) -> Result<ConstructionHashResponse, ApiError> {
    debug!("/construction/hash");
    match hex::decode(construction_hash_request.signed_transaction) {
        Ok(bytes) => {
            let message = Message::unpack(&mut bytes.as_slice()).map_err(|_| {
                ApiError::BadConstructionRequest("can not build message from signed_transaction".to_string())
            })?;
            Ok(ConstructionHashResponse {
                transaction_identifier: TransactionIdentifier {
                    hash: message.id().0.to_string(),
                },
            })
        }
        Err(e) => Err(ApiError::BadConstructionRequest(
            "can not decode signed_transaction, invalid hex string".to_string(),
        )),
    }
}
