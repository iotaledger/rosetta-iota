// Copyright 2020 IOTA Stiftung
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

    if request.public_key.curve_type != CurveType::Edwards25519 {
        return Err(ApiError::NonRetriable(
            "invalid curve type: must be edwards25519".to_string(),
        ));
    }

    let public_key_bytes = hex::decode(request.public_key.hex_bytes)
        .map_err(|e| ApiError::NonRetriable(format!("invalid public key provided: {}", e)))?;
    let public_key_hash = Blake2b256::digest(&public_key_bytes);
    
    let bech32_address = Address::Ed25519(Ed25519Address::new(public_key_hash.into())).to_bech32(&rosetta_config.bech32_hrp);

    Ok(ConstructionDeriveResponse {
        account_identifier: AccountIdentifier {
            address: bech32_address,
        },
    })
}