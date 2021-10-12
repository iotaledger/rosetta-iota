// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    construction::{
        combine::construction_combine_request, derive::construction_derive_request, hash::construction_hash_request,
        metadata::construction_metadata_request, parse::construction_parse_request,
        payloads::construction_payloads_request, preprocess::construction_preprocess_request,
        submit::construction_submit_request,
    },
    filters::{handle, with_rosetta_config},
    types::{SignedTransaction, UnsignedTransaction},
    RosettaConfig,
};

use warp::Filter;

pub mod combine;
pub mod derive;
pub mod hash;
pub mod metadata;
pub mod parse;
pub mod payloads;
pub mod preprocess;
pub mod submit;

pub fn routes(options: RosettaConfig) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(
            warp::path!("construction" / "derive")
                .and(warp::body::json())
                .and(with_rosetta_config(options.clone()))
                .and_then(handle(construction_derive_request)),
        )
        .or(warp::path!("construction" / "preprocess")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(construction_preprocess_request)))
        .or(warp::path!("construction" / "metadata")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(construction_metadata_request)))
        .or(warp::path!("construction" / "payloads")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(construction_payloads_request)))
        .or(warp::path!("construction" / "parse")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(construction_parse_request)))
        .or(warp::path!("construction" / "combine")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(construction_combine_request)))
        .or(warp::path!("construction" / "hash")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(construction_hash_request)))
        .or(warp::path!("construction" / "submit")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(construction_submit_request)))
}

fn serialize_unsigned_transaction(unsigned_transaction: &UnsignedTransaction) -> String {
    hex::encode(serde_json::to_string(unsigned_transaction).unwrap())
}

fn deserialize_unsigned_transaction(string: &String) -> UnsignedTransaction {
    serde_json::from_slice(&hex::decode(string).unwrap()).unwrap()
}

fn serialize_signed_transaction(signed_transaction: &SignedTransaction) -> String {
    hex::encode(serde_json::to_string(signed_transaction).unwrap())
}

fn deserialize_signed_transaction(string: &String) -> SignedTransaction {
    serde_json::from_slice(&hex::decode(string).unwrap()).unwrap()
}
