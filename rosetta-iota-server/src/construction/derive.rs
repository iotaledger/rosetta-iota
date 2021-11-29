// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, is_wrong_network, types::*, RosettaConfig};

use bee_message::prelude::{Address, Ed25519Address};

use crypto::hashes::{blake2b::Blake2b256, Digest};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionDeriveRequest {
    pub network_identifier: NetworkIdentifier,
    pub public_key: PublicKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionDeriveResponse {
    pub account_identifier: AccountIdentifier,
}

pub async fn derive(
    request: ConstructionDeriveRequest,
    rosetta_config: RosettaConfig,
) -> Result<ConstructionDeriveResponse, ApiError> {
    debug!("/construction/derive");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()));
    }

    let public_key_bytes = hex::decode(request.public_key.hex_bytes)
        .map_err(|e| ApiError::NonRetriable(format!("invalid public key provided: {}", e)))?;

    // serde only allows Ed25519 curve type; however the Ed25519 key size still needs to be checked
    if public_key_bytes.len() != 32 {
        return Err(ApiError::NonRetriable(format!(
            "invalid Ed25519 key length: expected a length of 32 bytes but received {} bytes",
            public_key_bytes.len()
        )));
    }

    let blake2b_hash = Blake2b256::digest(&public_key_bytes);

    Ok(ConstructionDeriveResponse {
        account_identifier: AccountIdentifier {
            address: Address::Ed25519(Ed25519Address::new(blake2b_hash.into())).to_bech32(&rosetta_config.bech32_hrp),
        },
    })
}
