// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, filters::{handle, with_options}, options::Options, types::{
    ConstructionHashRequest, ConstructionHashResponse, ConstructionSubmitRequest, ConstructionSubmitResponse,
    TransactionIdentifier,
}, consts};
use bee_common::packable::Packable;
use iota::Message;
use log::debug;
use warp::Filter;
use crate::types::{ConstructionDeriveRequest, ConstructionDeriveResponse, AccountIdentifier, CurveType, Transaction};
use bee_message::prelude::{Ed25519Address, Address, TransactionPayload};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

use std::convert::TryInto;

pub fn routes(options: Options) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(
            warp::path!("construction" / "derive")
                .and(warp::body::json())
                .and(with_options(options.clone()))
                .and_then(handle(construction_derive_request)),
        )
        .or(
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

async fn construction_derive_request(
    construction_derive_request: ConstructionDeriveRequest,
    options: Options,
) -> Result<ConstructionDeriveResponse, ApiError> {
    debug!("/construction/derive");

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

    let network_identifier = construction_derive_request.network_identifier;
    if network_identifier.blockchain != consts::BLOCKCHAIN || network_identifier.network != options.network {
        return Err(ApiError::BadNetwork);
    }

    if construction_derive_request.public_key.curve_type != CurveType::Edwards25519 {
        return Err(ApiError::UnsupportedCurve);
    };

    let public_key_bytes = hex::decode(construction_derive_request.public_key.hex_bytes)?;

    // Hash the public key to get the address
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(public_key_bytes);
    let mut result = vec![];
    hasher.finalize_variable(|res| {
        result = res.to_vec();
    });

    let ed25519_address = Ed25519Address::new(result.try_into().unwrap());
    let address = Address::Ed25519(ed25519_address);

    let bech32_hrp = iota_client.get_bech32_hrp().await.unwrap();

    Ok(ConstructionDeriveResponse {
        account_identifier: AccountIdentifier { address: address.to_bech32(&bech32_hrp), sub_account: None }
    })
}

async fn construction_hash_request(
    construction_hash_request: ConstructionHashRequest,
    options: Options,
) -> Result<ConstructionHashResponse, ApiError> {
    debug!("/construction/hash");

    let signed_transaction_hex_string = construction_hash_request.signed_transaction;
    let signed_transaction_hex_bytes = hex::decode(signed_transaction_hex_string)?;

    let unpacked_transaction = TransactionPayload::unpack(&mut signed_transaction_hex_bytes.as_slice())?;

    Ok(ConstructionHashResponse {
        transaction_identifier: TransactionIdentifier {
            hash: unpacked_transaction.id().to_string(),
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
