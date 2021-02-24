// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{consts, currency::iota_currency, operations::*, error::ApiError, filters::{handle, with_options}, options::Options, types::{
    ConstructionHashRequest, ConstructionHashResponse, ConstructionSubmitRequest, ConstructionSubmitResponse,
    TransactionIdentifier,
}, is_bad_network};
use bee_common::packable::Packable;
use log::debug;
use warp::Filter;
use crate::types::{ConstructionDeriveRequest, ConstructionDeriveResponse, ConstructionParseRequest, AccountIdentifier, CurveType, ConstructionSubmitResponseMetadata, ConstructionPreprocessRequest, ConstructionPreprocessResponse, ConstructionPayloadsRequest, ConstructionPayloadsResponse, Operation, SigningPayload, SignatureType, ConstructionMetadataRequest, ConstructionMetadataResponse, ConstructionMetadata, ConstructionParseResponse, OperationIdentifier, OperationMetadata, CoinChange, Amount, ConstructionCombineResponse, ConstructionCombineRequest, Signature};
use bee_message::prelude::{Ed25519Address, Address, TransactionId, Input, Output, SignatureLockedSingleOutput, UTXOInput, RegularEssenceBuilder, RegularEssence, Ed25519Signature};
use iota::{Client, Payload, TransactionPayload, OutputDto, AddressDto};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

use std::convert::TryInto;
use std::str::FromStr;
use crate::operations::UTXO_SPENT;
use serde::Serialize;
use bee_message::payload::transaction::{Essence, UnlockBlock, ReferenceUnlock, SignatureUnlock};
use std::collections::HashMap;

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
                let output_id_str = match operation.coin_change {
                    Some(coin_change) => coin_change.coin_identifier.identifier,
                    None => panic!("no coin_change on UTXO_INPUT!")
                };
                let output_id_bytes = hex::decode(output_id_str).unwrap();
                let (transaction_id, index) = output_id_bytes.split_at(32);
                let output_index = u16::from_le_bytes(index.try_into().unwrap());
                let utxo_input = UTXOInput::new(TransactionId::new(From::<[u8; 32]>::from(transaction_id.try_into().unwrap())), output_index).unwrap();
                let input: Input = Input::UTXO(utxo_input.clone());
                let address = operation.account.address;
                inputs.push((input, address));
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

    for (i, _) in inputs.clone() {
        transaction_payload_essence = transaction_payload_essence.add_input(i);
    }

    for o in outputs {
        transaction_payload_essence = transaction_payload_essence.add_output(o);
    }

    let transaction_payload_essence = transaction_payload_essence.finish().unwrap();
    let transaction_payload_essence_hex = hex::encode(transaction_payload_essence.pack_new());

    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(transaction_payload_essence.pack_new());
    let mut hash = vec![];
    hasher.finalize_variable(|res| {
        hash = res.to_vec();
    });

    for (_, address) in inputs {
        signing_payloads.push( SigningPayload {
            account_identifier: AccountIdentifier {
                address,
                sub_account: None
            },
            hex_bytes: hex::encode(hash.clone()),
            signature_type: Some(SignatureType::Edwards25519)
        });
    }

    Ok(ConstructionPayloadsResponse {
        unsigned_transaction: transaction_payload_essence_hex,
        payloads: signing_payloads
    })
}

async fn construction_parse_request(
    construction_parse_request: ConstructionParseRequest,
    options: Options,
) -> Result<ConstructionParseResponse, ApiError> {
    debug!("/construction/parse");

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

    // todo: add logic for pre-signed transactions

    let mut transaction_hex_bytes = hex::decode(construction_parse_request.transaction)?;
    let transaction_essence = RegularEssence::unpack(&mut transaction_hex_bytes.as_slice()).unwrap();

    let mut operations = vec![];
    let mut operation_counter = 0;

    for input in transaction_essence.inputs() {
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
                OutputDto::SignatureLockedDustAllowance(x) => panic!("not implemented!"),
            };

            let bech32_hrp = iota_client.get_bech32_hrp().await.unwrap();
            let bech32_address = Ed25519Address::from_str(&ed25519_address).unwrap().to_bech32(&bech32_hrp[..]);

            operations.push(utxo_operation(transaction_id, bech32_address, amount, output_index, operation_counter, &true, is_spent));
        }
        operation_counter = operation_counter + 1;
    }

    let mut output_index = 0;
    for output in transaction_essence.outputs() {
        let (amount, ed25519_address) = match output {
            Output::SignatureLockedSingle(x) => match x.address() {
                Address::Ed25519(ed25519) => (x.amount(), ed25519.clone().to_string()),
                _ => panic!("not implemented!")
            },
            _ => panic!("not implemented!")
        };

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

    Ok(ConstructionParseResponse {
        operations: operations,
        account_identifier_signers: None,
    })
}

async fn construction_combine_request(
    construction_combine_request: ConstructionCombineRequest,
    options: Options,
) -> Result<ConstructionCombineResponse, ApiError> {
    debug!("/construction/combine");

    is_bad_network(&options, &construction_combine_request.network_identifier)?;

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

    let essence = essence_from_hex_string(&construction_combine_request.unsigned_transaction)?;

    let regular_essence = match &essence {
        Essence::Regular(r) => r,
        _ => return Err(ApiError::BadConstructionRequest("essence type not supported".to_string()))
    };

    let signature_by_address_map = {
        let mut ret = HashMap::new();
        for s in &construction_combine_request.signatures {
            ret.insert(s.signing_payload.account_identifier.address.clone(), s.clone());
        }
        ret
    };

    let mut signature_unlock_block_index_by_address = HashMap::new();
    let mut unlock_blocks = Vec::new();

    for input in regular_essence.inputs() {

        let input = match input {
            Input::UTXO(i) => i,
            _ => return Err(ApiError::BadConstructionRequest("input type not supported".to_string()))
        };

        let input_metadata = iota_client.get_output(&input).await.unwrap(); // TODO: handle unwrap
        let address = match input_metadata.output {
            OutputDto::SignatureLockedSingle(s) => {
                match s.address {
                    AddressDto::Ed25519(e) => e.address,
                    _ => return Err(ApiError::BadConstructionRequest("address type of output not supported".to_string()))
                }
            },
            OutputDto::SignatureLockedDustAllowance(s) => unimplemented!(),
            _ => return Err(ApiError::BadConstructionRequest("output type not supported".to_string()))
        };

        // check if there exists a signature by the address of the input
        if let Some(signature) = signature_by_address_map.get(&address) {

            // check if a signature unlock block was already added for the address
            if let Some(index) = signature_unlock_block_index_by_address.get(&address) {
                // add a reference unlock block which references the index of the signature unlock block
                unlock_blocks.push(UnlockBlock::Reference(ReferenceUnlock::new(*index).unwrap())); // TODO: handle unwrap
            } else {
                let mut public_key = [0u8; 32];
                hex::decode_to_slice(signature.public_key.hex_bytes.clone(), &mut public_key)?;
                let signature = Ed25519Signature::new(
                    public_key,
                    hex::decode(signature.hex_bytes.clone())?.into_boxed_slice()
                );
                unlock_blocks.push(UnlockBlock::Signature(SignatureUnlock::Ed25519(signature)));
                signature_unlock_block_index_by_address.insert(address, signature_unlock_block_index_by_address.len() as u16);
            }
        } else {
            return Err(ApiError::BadConstructionRequest(format!("no signature for address {} provided", &address)))
        }
    }

    let transaction = TransactionPayload::builder()
        .with_essence(essence)
        .with_unlock_blocks(unlock_blocks)
        .finish()?;

    Ok(ConstructionCombineResponse {
        signed_transaction: hex::encode(transaction.pack_new())
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

fn essence_from_hex_string(hex_str: &str) -> Result<Essence, ApiError> {
    let essence_bytes = hex::decode(hex_str)?;
    Ok(Essence::unpack(&mut essence_bytes.as_slice()).unwrap())
}