// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, is_wrong_network, types::*, Config};

use bee_message::prelude::{Address, Ed25519Address};

use crypto::hashes::{blake2b::Blake2b256, Digest};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionDeriveRequest {
    pub network_identifier: NetworkIdentifier,
    pub public_key: PublicKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionDeriveResponse {
    pub account_identifier: AccountIdentifier,
}

pub async fn construction_derive_request(
    request: ConstructionDeriveRequest,
    options: Config,
) -> Result<ConstructionDeriveResponse, ApiError> {
    debug!("/construction/derive");

    if is_wrong_network(&options, &request.network_identifier) {
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

    let bech32_address = Address::Ed25519(Ed25519Address::new(public_key_hash.into())).to_bech32(&options.bech32_hrp);

    Ok(ConstructionDeriveResponse {
        account_identifier: AccountIdentifier {
            address: bech32_address,
            sub_account: None,
        },
    })
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::config::RosettaMode;

    #[tokio::test]
    async fn test_derive() {
        let data = r#"{"network_identifier":{"blockchain":"iota","network":"testnet7"},"public_key":{"hex_bytes":"6f8f4d77e94bce3900078b89319e6e25b341d47669a76ae4bf26677d377533f0","curve_type":"edwards25519"}}"#;
        let request: ConstructionDeriveRequest = serde_json::from_str(data).unwrap();

        let server_options = Config {
            node_url: "http://127.0.0.1:3029".to_string(),
            network: "testnet7".to_string(),
            tx_tag: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };

        let response = construction_derive_request(request, server_options).await.unwrap();
        assert_eq!(
            "atoi1qpv2nr99fkjykh5ga3x62lqlztrg6t67k750v93lculsna3z7knnu9wdz4h",
            response.account_identifier.address
        )
    }
}
