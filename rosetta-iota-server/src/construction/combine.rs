// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    construction::{deserialize_unsigned_transaction, serialize_signed_transaction},
    error::ApiError,
    is_wrong_network,
    types::*,
    RosettaConfig,
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
    request: ConstructionCombineRequest,
    options: RosettaConfig,
) -> Result<ConstructionCombineResponse, ApiError> {
    debug!("/construction/combine");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()));
    }

    let unsigned_transaction = deserialize_unsigned_transaction(&request.unsigned_transaction);

    let regular_essence = match &unsigned_transaction.essence() {
        Essence::Regular(r) => r,
    };

    if regular_essence.inputs().len() != request.signatures.len() {
        return Err(ApiError::NonRetriable(
            "for every input a signature must be provided".to_string(),
        ));
    }

    let mut unlock_blocks = Vec::new();
    type SignatureUnlockBlockIndex = u16;
    let mut index_of_signature_unlock_block_with_address: HashMap<String, SignatureUnlockBlockIndex> = HashMap::new();

    for signature in request.signatures {
        // get address for which the signature was produced
        let bech32_addr = signature
            .signing_payload
            .account_identifier
            .ok_or(ApiError::NonRetriable(
                "signing_payload.account_identifier not populated".to_string(),
            ))?
            .address;

        // check if a Signature Unlock Block already was added for the address
        if let Some(index) = index_of_signature_unlock_block_with_address.get(&bech32_addr) {
            // build a Reference Unlock Block
            unlock_blocks.push(UnlockBlock::Reference(ReferenceUnlock::new(*index).unwrap()));
        } else {
            // build a Signature Unlock Block

            let signature = {
                let mut public_key_bytes = [0u8; 32];
                let mut signature_bytes = [0u8; 64];
                hex::decode_to_slice(signature.public_key.hex_bytes.clone(), &mut public_key_bytes)
                    .map_err(|e| ApiError::NonRetriable(format!("invalid public key: {}", e)))?;
                hex::decode_to_slice(signature.hex_bytes.clone(), &mut signature_bytes)
                    .map_err(|e| ApiError::NonRetriable(format!("invalid signature: {}", e)))?;
                Ed25519Signature::new(public_key_bytes, signature_bytes)
            };

            unlock_blocks.push(UnlockBlock::Signature(SignatureUnlock::Ed25519(signature)));

            // memorise the address and index of the Signature Unlock Block
            index_of_signature_unlock_block_with_address.insert(bech32_addr, (unlock_blocks.len() - 1) as u16);
        }
    }

    let transaction = TransactionPayload::builder()
        .with_essence(unsigned_transaction.essence().clone())
        .with_unlock_blocks(UnlockBlocks::new(unlock_blocks).unwrap())
        .finish()
        .map_err(|e| ApiError::NonRetriable(format!("can not build transaction: {}", e)))?;

    let signed_transaction = SignedTransaction::new(transaction, unsigned_transaction.inputs_metadata().clone());

    Ok(ConstructionCombineResponse {
        signed_transaction: serialize_signed_transaction(&signed_transaction),
    })
}
