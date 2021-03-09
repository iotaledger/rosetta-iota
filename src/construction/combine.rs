// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options, is_bad_network, build_iota_client, require_offline_mode};
use crate::error::ApiError;

use log::debug;
use bee_message::prelude::*;
use bee_common::packable::Packable;

use std::collections::HashMap;
use crate::construction::essence_from_hex_string;
use bee_rest_api::types::{OutputDto, AddressDto};

pub(crate) async fn construction_combine_request(
    construction_combine_request: ConstructionCombineRequest,
    options: Options,
) -> Result<ConstructionCombineResponse, ApiError> {
    debug!("/construction/combine");

    let _ = require_offline_mode(&options)?;

    is_bad_network(&options, &construction_combine_request.network_identifier)?;

    let iota_client = build_iota_client(&options, false).await?;

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
                    AddressDto::Ed25519(e) => e.address
                }
            },
            OutputDto::SignatureLockedDustAllowance(_) => unimplemented!(),
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
        .with_unlock_blocks(UnlockBlocks::new(unlock_blocks).unwrap())
        .finish()?;

    Ok(ConstructionCombineResponse {
        signed_transaction: hex::encode(transaction.pack_new())
    })

}