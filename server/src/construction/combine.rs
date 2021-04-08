// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    construction::{deserialize_unsigned_transaction, serialize_signed_transaction},
    error::ApiError,
    is_wrong_network, require_offline_mode,
    types::*,
    Options,
};

use bee_message::prelude::*;

use log::debug;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionCombineRequest {
    pub network_identifier: NetworkIdentifier,
    pub unsigned_transaction: String,
    pub signatures: Vec<Signature>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionCombineResponse {
    pub signed_transaction: String,
}

pub(crate) async fn construction_combine_request(
    construction_combine_request: ConstructionCombineRequest,
    options: Options,
) -> Result<ConstructionCombineResponse, ApiError> {
    debug!("/construction/combine");

    let _ = require_offline_mode(&options)?;

    is_wrong_network(&options, &construction_combine_request.network_identifier)?;

    let unsigned_transaction = deserialize_unsigned_transaction(&construction_combine_request.unsigned_transaction);

    let regular_essence = match &unsigned_transaction.essence() {
        Essence::Regular(r) => r,
        _ => {
            return Err(ApiError::BadConstructionRequest(
                "essence type not supported".to_string(),
            ));
        }
    };

    if regular_essence.inputs().len() != construction_combine_request.signatures.len() {
        return Err(ApiError::BadConstructionRequest(
            "for every input a signature must be provided".to_string(),
        ));
    }

    let mut unlock_blocks = Vec::new();
    let mut index_of_signature_unlock_block_with_address: HashMap<String, u16> = HashMap::new();

    for signature in construction_combine_request.signatures {

        // get address for which the signature was produced
        let bech32_addr = signature
            .signing_payload
            .account_identifier
            .ok_or(ApiError::BadConstructionRequest(
                "signing_payload.account_identifier not populated".to_string(),
            ))?.address;

        // check if a Signature Unlock Block already was added for the address
        if let Some(index) = index_of_signature_unlock_block_with_address.get(&bech32_addr) {
            // build a Reference Unlock Block
            unlock_blocks.push(UnlockBlock::Reference(ReferenceUnlock::new(*index).unwrap()));
        } else {
            // build a Signature Unlock Block
            let mut public_key = [0u8; 32];
            hex::decode_to_slice(signature.public_key.hex_bytes.clone(), &mut public_key)?;
            let signature =
                Ed25519Signature::new(public_key, hex::decode(signature.hex_bytes.clone())?.into_boxed_slice());

            unlock_blocks.push(UnlockBlock::Signature(SignatureUnlock::Ed25519(signature)));

            // memorise the address and index of the Signature Unlock Block
            index_of_signature_unlock_block_with_address
                .insert(bech32_addr, unlock_blocks.len() as u16);
        }
    }

    let transaction = TransactionPayload::builder()
        .with_essence(unsigned_transaction.essence().clone())
        .with_unlock_blocks(UnlockBlocks::new(unlock_blocks).unwrap())
        .finish()?;

    let signed_transaction = SignedTransaction::new(transaction, unsigned_transaction.inputs_metadata().clone());

    Ok(ConstructionCombineResponse {
        signed_transaction: serialize_signed_transaction(&signed_transaction),
    })
}
