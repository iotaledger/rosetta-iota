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
use crate::types::*;
use bee_common::packable::Packable;
use std::str::FromStr;
use bee_rest_api::types::{OutputDto, AddressDto};
use crate::operations::{utxo_operation, UTXO_UNSPENT, UTXO_OUTPUT};
use crate::currency::iota_currency;
use crypto::hashes::blake2b::Blake2b256;
use crypto::hashes::Digest;
use std::convert::TryInto;
use iota::Client;

mod combine;
mod derive;
mod hash;
mod metadata;
mod parse;
mod payloads;
mod preprocess;
mod submit;

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


async fn regular_essence_to_operations(regular_essence: &RegularEssence, iota_client: Client) -> Result<Vec<Operation>, ApiError>{

    let mut operations = vec![];
    let mut operation_counter = 0;

    for input in regular_essence.inputs() {
        if let Input::UTXO(i) = input {
            let input_metadata = iota_client.get_output(&i).await.unwrap();
            let transaction_id = input_metadata.transaction_id;
            let output_index = input_metadata.output_index;
            let is_spent = input_metadata.is_spent;

            let (amount, ed25519_address) = match input_metadata.output {
                OutputDto::Treasury(_) => panic!("Can't be used as input"),
                OutputDto::SignatureLockedSingle(x) => match x.address {
                    AddressDto::Ed25519(ed25519) => (x.amount, ed25519.address)
                },
                OutputDto::SignatureLockedDustAllowance(_) => panic!("not implemented!"),
            };

            // todo: treat timeout on this unrwap
            let bech32_hrp = iota_client.get_bech32_hrp().await.unwrap();
            let bech32_address = Ed25519Address::from_str(&ed25519_address).unwrap().to_bech32(&bech32_hrp[..]);

            operations.push(utxo_operation(transaction_id, bech32_address, amount, output_index, operation_counter, &true, is_spent));
        }
        operation_counter = operation_counter + 1;
    }

    let mut output_index = 0;
    for output in regular_essence.outputs() {
        let (amount, ed25519_address) = match output {
            Output::SignatureLockedSingle(x) => match x.address() {
                Address::Ed25519(ed25519) => (x.amount(), ed25519.clone().to_string()),
                _ => panic!("not implemented!")
            },
            _ => panic!("not implemented!")
        };

        // todo: treat timeout on this unrwap
        let bech32_hrp = iota_client.get_bech32_hrp().await.unwrap();
        let bech32_address = Ed25519Address::from_str(&ed25519_address).unwrap().to_bech32(&bech32_hrp[..]);

        operations.push(Operation {
            operation_identifier: OperationIdentifier {
                index: operation_counter as u64,
                network_index: Some(output_index as u64),
            },
            related_operations: None,
            type_: UTXO_OUTPUT.into(),
            status: None,
            account: AccountIdentifier {
                address: bech32_address,
                sub_account: None
            },
            amount: Amount {
                value: amount.to_string(),
                currency: iota_currency(),
            },
            coin_change: None,
            metadata: OperationMetadata {
                is_spent: UTXO_UNSPENT.into()
            }
        });
        output_index = output_index + 1;
        operation_counter = operation_counter + 1;
    }

    Ok(operations)

}

fn transaction_from_hex_string(hex_str: &str) -> Result<TransactionPayload, ApiError> {
    let signed_transaction_hex_bytes = hex::decode(hex_str)?;
    Ok(TransactionPayload::unpack(&mut signed_transaction_hex_bytes.as_slice()).unwrap())
}

fn essence_from_hex_string(hex_str: &str) -> Result<Essence, ApiError> {
    let essence_bytes = hex::decode(hex_str)?;
    Ok(Essence::unpack(&mut essence_bytes.as_slice()).unwrap())
}

fn address_from_public_key(hex_string: &str) -> Result<Address, ApiError> {
    let public_key_bytes = hex::decode(hex_string)?;
    let hash = Blake2b256::digest(&public_key_bytes);
    let ed25519_address = Ed25519Address::new(hash.try_into().unwrap());
    let address = Address::Ed25519(ed25519_address);

    Ok(address)
}