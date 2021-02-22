// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{consts, operations, error::ApiError, filters::{handle, with_options}, options::Options, types::{
    ConstructionHashRequest, ConstructionHashResponse, ConstructionSubmitRequest, ConstructionSubmitResponse,
    TransactionIdentifier,
}, is_bad_network};
use bee_common::packable::Packable;
use log::debug;
use warp::Filter;
use crate::types::{ConstructionDeriveRequest, ConstructionDeriveResponse, AccountIdentifier, CurveType, ConstructionSubmitResponseMetadata, ConstructionPreprocessRequest, ConstructionPreprocessResponse, ConstructionPayloadsRequest, ConstructionPayloadsResponse, Operation, SigningPayload, SignatureType, ConstructionMetadataRequest, ConstructionMetadataResponse, ConstructionMetadata};
use bee_message::prelude::{Ed25519Address, Address, TransactionId, Input, Output, SignatureLockedSingleOutput, UTXOInput, RegularEssenceBuilder};
use iota::{Client, Payload, TransactionPayload, RegularEssence};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

use std::convert::TryInto;
use std::str;
use crate::operations::UTXO_SPENT;
use serde::Serialize;

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

    is_bad_network(&options, &construction_derive_request.network_identifier)?;

    if construction_derive_request.public_key.curve_type != CurveType::Edwards25519 {
        return Err(ApiError::UnsupportedCurve);
    };

    let public_key_bytes = hex::decode(construction_derive_request.public_key.hex_bytes)?;

    // Hash the public key to get the address as in https://github.com/iotaledger/wallet.rs/blob/develop/src/stronghold.rs#L531
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

async fn construction_preprocess_request(
    construction_preprocess_request: ConstructionPreprocessRequest,
    options: Options,
) -> Result<ConstructionPreprocessResponse, ApiError> {
    debug!("/construction/preprocess");

    is_bad_network(&options, &construction_preprocess_request.network_identifier)?;

    Ok(ConstructionPreprocessResponse {
        options: None
    })
}

async fn construction_metadata_request(
    construction_metadata_request: ConstructionMetadataRequest,
    options: Options,
) -> Result<ConstructionMetadataResponse, ApiError> {
    debug!("/construction/metadata");

    is_bad_network(&options, &construction_metadata_request.network_identifier)?;

    if options.mode != consts::ONLINE_MODE {
        return Err(ApiError::UnavailableOffline);
    }

    Ok(ConstructionMetadataResponse {
        metadata: None
    })
}

async fn construction_payloads_request(
    construction_payloads_request: ConstructionPayloadsRequest,
    options: Options,
) -> Result<ConstructionPayloadsResponse, ApiError> {
    debug!("/construction/payloads");

    is_bad_network(&options, &construction_payloads_request.network_identifier)?;

    let mut inputs = vec![];
    let mut outputs = vec![];

    let mut signing_payloads = vec![];

    for operation in construction_payloads_request.operations {
        match &operation.type_[..] {
            "UTXO_INPUT" => {
                if operation.metadata.is_spent == UTXO_SPENT {
                    return Err(ApiError::UnableToSpend);
                }
                let output_id_str = operation.coin_change.coin_identifier.identifier;
                let output_id_bytes = hex::decode(output_id_str).unwrap();
                let (transaction_id, index) = output_id_bytes.split_at(32);
                let output_index = u16::from_le_bytes(index.try_into().unwrap());
                let utxo_input = UTXOInput::new(TransactionId::new(From::<[u8; 32]>::from(transaction_id.try_into().unwrap())), output_index).unwrap();
                let input: Input = Input::UTXO(utxo_input.clone());
                inputs.push(input);

                signing_payloads.push( SigningPayload {
                    address: operation.account.address,
                    hex_bytes: hex::encode(utxo_input.to_string()),
                    signature_type: Some(SignatureType::Edwards25519)
                });
            },
            "UTXO_OUTPUT" => {
                let address = Address::try_from_bech32(&operation.account.address).unwrap();
                let amount = operation.amount.value.parse::<u64>().unwrap();
                // todo: tread Dust allowance
                let output: Output = SignatureLockedSingleOutput::new(address, amount).unwrap().into();
                outputs.push(output);
            },
            _ => return Err(ApiError::UnknownOperationType)
        }
    }

    let mut transaction_payload_essence = RegularEssenceBuilder::new();

    // todo: Rosetta indexation payload?
    // builder = builder.with_payload(p);

    for i in inputs {
        transaction_payload_essence = transaction_payload_essence.add_input(i);
    }

    for o in outputs {
        transaction_payload_essence = transaction_payload_essence.add_output(o);
    }

    let transaction_payload_essence = transaction_payload_essence.finish().unwrap();
    let transaction_payload_essence_hex = hex::encode(transaction_payload_essence.pack_new());

    Ok(ConstructionPayloadsResponse {
        unsigned_transaction: transaction_payload_essence_hex,
        payloads: signing_payloads
    })
}

async fn construction_hash_request(
    construction_hash_request: ConstructionHashRequest,
    options: Options,
) -> Result<ConstructionHashResponse, ApiError> {
    debug!("/construction/hash");

    is_bad_network(&options, &construction_hash_request.network_identifier)?;

    let transaction = transaction_from_hex_string(&construction_hash_request.signed_transaction)?;

    Ok(ConstructionHashResponse {
        transaction_identifier: TransactionIdentifier {
            hash: transaction.id().to_string(),
        },
    })
}

async fn construction_submit_request(
    construction_submit_request: ConstructionSubmitRequest,
    options: Options,
) -> Result<ConstructionSubmitResponse, ApiError> {
    debug!("/construction/submit");

    if options.mode != consts::ONLINE_MODE {
        return Err(ApiError::UnavailableOffline);
    }

    is_bad_network(&options, &construction_submit_request.network_identifier)?;

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

fn transaction_from_hex_string(hex_str: &str) -> Result<TransactionPayload, ApiError> {
    let signed_transaction_hex_bytes = hex::decode(hex_str)?;
    Ok(TransactionPayload::unpack(&mut signed_transaction_hex_bytes.as_slice()).unwrap())
}