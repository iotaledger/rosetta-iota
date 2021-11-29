// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    construction::{deserialize_signed_transaction, deserialize_unsigned_transaction},
    error::ApiError,
    is_wrong_network,
    operations::{build_utxo_input_operation, build_utxo_output_operation},
    types::*,
    RosettaConfig,
};

use bee_message::prelude::*;
use bee_rest_api::types::responses::OutputResponse;

use crypto::hashes::{blake2b::Blake2b256, Digest};

use log::debug;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionParseRequest {
    pub network_identifier: NetworkIdentifier,
    pub signed: bool,
    pub transaction: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionParseResponse {
    pub operations: Vec<Operation>,
    pub account_identifier_signers: Option<Vec<AccountIdentifier>>,
}

pub async fn parse(
    request: ConstructionParseRequest,
    rosetta_config: RosettaConfig,
) -> Result<ConstructionParseResponse, ApiError> {
    debug!("/construction/parse");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()));
    }

    if request.signed {
        parse_signed_transaction(request, &rosetta_config).await
    } else {
        parse_unsigned_transaction(request, &rosetta_config).await
    }
}

async fn parse_unsigned_transaction(
    construction_parse_request: ConstructionParseRequest,
    options: &RosettaConfig,
) -> Result<ConstructionParseResponse, ApiError> {
    let unsigned_transaction = deserialize_unsigned_transaction(&construction_parse_request.transaction);

    let operations = essence_to_operations(
        unsigned_transaction.essence(),
        unsigned_transaction.inputs_metadata(),
        options,
    )
    .await?;

    Ok(ConstructionParseResponse {
        operations,
        account_identifier_signers: None,
    })
}

async fn parse_signed_transaction(
    construction_parse_request: ConstructionParseRequest,
    options: &RosettaConfig,
) -> Result<ConstructionParseResponse, ApiError> {
    let signed_transaction = deserialize_signed_transaction(&construction_parse_request.transaction);

    let transaction = signed_transaction.transaction();

    let operations =
        essence_to_operations(transaction.essence(), signed_transaction.inputs_metadata(), options).await?;

    let account_identifier_signers = {
        let mut accounts_identifiers = Vec::new();
        for unlock_block in transaction.unlock_blocks().iter() {
            if let UnlockBlock::Signature(s) = unlock_block {
                let SignatureUnlock::Ed25519(signature) = s;
                let bech32_addr =
                    address_from_public_key(&hex::encode(signature.public_key()))?.to_bech32(&options.bech32_hrp);
                accounts_identifiers.push(AccountIdentifier { address: bech32_addr });
            }
        }
        accounts_identifiers
    };

    Ok(ConstructionParseResponse {
        operations,
        account_identifier_signers: Some(account_identifier_signers),
    })
}

async fn essence_to_operations(
    essence: &Essence,
    inputs_metadata: &HashMap<String, OutputResponse>,
    rosetta_config: &RosettaConfig,
) -> Result<Vec<Operation>, ApiError> {
    let Essence::Regular(regular_essence) = essence;

    let mut operations = Vec::new();

    for input in regular_essence.inputs() {
        let utxo_input = match input {
            Input::Utxo(i) => i,
            _ => return Err(ApiError::NonRetriable("unknown input type".to_string())),
        };

        let input_metadata = match inputs_metadata.get(&utxo_input.to_string()) {
            Some(metadata) => metadata,
            None => return Err(ApiError::NonRetriable("missing metadata for input".to_string())),
        };

        let output = Output::try_from(&input_metadata.output)
            .map_err(|e| ApiError::NonRetriable(format!("can not deserialize output: {}", e)))?;

        operations.push(build_utxo_input_operation(
            utxo_input.output_id(),
            &output,
            operations.len(),
            true,
            rosetta_config,
        )?);
    }

    for output in regular_essence.outputs() {
        let output_operation = build_utxo_output_operation(None, output, operations.len(), false, rosetta_config)?;

        operations.push(output_operation);
    }

    Ok(operations)
}

fn address_from_public_key(hex_string: &str) -> Result<Address, ApiError> {
    let public_key_bytes = hex::decode(hex_string)
        .map_err(|e| ApiError::NonRetriable(format!("can not derive address from public key: {}", e)))?;
    let hash = Blake2b256::digest(&public_key_bytes);
    let ed25519_address = Ed25519Address::new(hash.try_into().unwrap());
    let address = Address::Ed25519(ed25519_address);

    Ok(address)
}
