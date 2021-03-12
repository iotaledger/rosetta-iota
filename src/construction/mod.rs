// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::construction::derive::construction_derive_request;
use crate::construction::preprocess::construction_preprocess_request;
use crate::construction::metadata::construction_metadata_request;
use crate::construction::payloads::construction_payloads_request;
use crate::construction::combine::construction_combine_request;
use crate::construction::hash::construction_hash_request;
use crate::construction::submit::construction_submit_request;
use crate::construction::parse::construction_parse_request;
use crate::Options;
use crate::filters::{with_options, handle};

use warp::Filter;
use bee_message::prelude::*;
use crate::error::ApiError;

use bee_common::packable::Packable;


use crypto::hashes::blake2b::Blake2b256;
use crypto::hashes::Digest;
use std::convert::TryInto;

pub mod combine;
pub mod derive;
pub mod hash;
pub mod metadata;
pub mod parse;
pub mod payloads;
pub mod preprocess;
pub mod submit;

pub fn routes(options: Options) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(
            warp::path!("construction" / "derive")
                .and(warp::body::json())
                .and(with_options(options.clone()))
                .and_then(handle(construction_derive_request)),
        )
        .or(
            warp::path!("construction" / "preprocess")
                .and(warp::body::json())
                .and(with_options(options.clone()))
                .and_then(handle(construction_preprocess_request)),
        )
        .or(
            warp::path!("construction" / "metadata")
                .and(warp::body::json())
                .and(with_options(options.clone()))
                .and_then(handle(construction_metadata_request)),
        )
        .or(
            warp::path!("construction" / "payloads")
                .and(warp::body::json())
                .and(with_options(options.clone()))
                .and_then(handle(construction_payloads_request)),
        )
        .or(warp::path!("construction" / "parse")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(construction_parse_request)))
        .or(
            warp::path!("construction" / "combine")
                .and(warp::body::json())
                .and(with_options(options.clone()))
                .and_then(handle(construction_combine_request)),
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
                .and_then(handle(construction_submit_request)),
        )
}

fn transaction_from_hex_string(hex_str: &str) -> Result<TransactionPayload, ApiError> {
    let signed_transaction_hex_bytes = hex::decode(hex_str)?;
    Ok(TransactionPayload::unpack(&mut signed_transaction_hex_bytes.as_slice()).unwrap())
}

fn address_from_public_key(hex_string: &str) -> Result<Address, ApiError> {
    let public_key_bytes = hex::decode(hex_string)?;
    let hash = Blake2b256::digest(&public_key_bytes);
    let ed25519_address = Ed25519Address::new(hash.try_into().unwrap());
    let address = Address::Ed25519(ed25519_address);

    Ok(address)
}