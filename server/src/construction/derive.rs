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
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()))
    }

    if request.public_key.curve_type != CurveType::Edwards25519 {
        return Err(ApiError::NonRetriable("invalid curve type: must be edwards25519".to_string()))
    }

    let public_key_bytes = hex::decode(request.public_key.hex_bytes).map_err(|e| ApiError::NonRetriable(format!("invalid public key provided: {}", e)))?;
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
    async fn test_address_from_public_key() {
        let request = ConstructionDeriveRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet6".to_string(),
                sub_network_identifier: None,
            },
            public_key: PublicKey {
                hex_bytes: "29bdea325f58cb4ad7493ba7bc12c36bafb381350f5fbea0357ad2b869793e95".to_string(),
                curve_type: CurveType::Edwards25519,
            },
        };

        let server_options = Config {
            node: "https://api.lb-0.testnet.chrysalis2.com".to_string(),
            network: "testnet6".to_string(),
            tx_indexation: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Offline,
            bind_addr: "0.0.0.0:3030".to_string(),
        };

        let response = construction_derive_request(request, server_options).await.unwrap();
        assert_eq!(
            "atoi1qqdxdqak4x96hzw6rkt48t4x5t84e209jmmlkp6t6lcx8aj6unr7zzv994h",
            response.account_identifier.address
        )
    }

    #[tokio::test]
    async fn test_bad_network() {
        let request = ConstructionDeriveRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet4".to_string(),
                sub_network_identifier: None,
            },
            public_key: PublicKey {
                hex_bytes: "29bdea325f58cb4ad7493ba7bc12c36bafb381350f5fbea0357ad2b869793e95".to_string(),
                curve_type: CurveType::Edwards25519,
            },
        };

        let server_options = Config {
            node: "https://api.lb-0.testnet.chrysalis2.com".to_string(),
            network: "testnet6".to_string(),
            tx_indexation: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Offline,
            bind_addr: "0.0.0.0:3030".to_string(),
        };

        if let Err(e) = construction_derive_request(request, server_options).await {
            match e {
                ApiError::WrongNetwork => (),
                _ => panic!("expected bad network error"),
            }
        } else {
            panic!("expected an error")
        }
    }
}
