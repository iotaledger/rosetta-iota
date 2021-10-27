// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    construction::{
        combine::combine, derive::derive, hash::hash,
        metadata::metadata, parse::parse,
        payloads::payloads, preprocess::preprocess,
        submit::submit,
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
                .and_then(handle(derive)),
        )
        .or(warp::path!("construction" / "preprocess")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(preprocess)))
        .or(warp::path!("construction" / "metadata")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(metadata)))
        .or(warp::path!("construction" / "payloads")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(payloads)))
        .or(warp::path!("construction" / "parse")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(parse)))
        .or(warp::path!("construction" / "combine")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(combine)))
        .or(warp::path!("construction" / "hash")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(hash)))
        .or(warp::path!("construction" / "submit")
            .and(warp::body::json())
            .and(with_rosetta_config(options.clone()))
            .and_then(handle(submit)))
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
