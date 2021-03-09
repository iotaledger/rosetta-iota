// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options, is_bad_network};
use crate::error::ApiError;

use bee_message::prelude::{Address, Ed25519Address};
use crypto::hashes::blake2b::Blake2b256;
use crypto::hashes::Digest;
use log::debug;

use std::convert::TryInto;

pub async fn construction_derive_request(
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
    let public_key_hash = Blake2b256::digest(&public_key_bytes);

    let address = Address::Ed25519(Ed25519Address::new(public_key_hash.into()));

    // todo: treat timeout on this unrwap
    let bech32_hrp = iota_client.get_bech32_hrp().await.unwrap();

    Ok(ConstructionDeriveResponse {
        account_identifier: AccountIdentifier { address: address.to_bech32(&bech32_hrp), sub_account: None }
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_address_from_public_key() {
        let request = ConstructionDeriveRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet6".to_string(),
                sub_network_identifier: None
            },
            public_key: PublicKey {
                hex_bytes: "29bdea325f58cb4ad7493ba7bc12c36bafb381350f5fbea0357ad2b869793e95".to_string(),
                curve_type: CurveType::Edwards25519
            }
        };

        let server_options = Options {
            iota_endpoint: "https://api.lb-0.testnet.chrysalis2.com".to_string(),
            network: "testnet6".to_string(),
            mode: "online".to_string(),
            port: 3030
        };

        let response = construction_derive_request(request, server_options).await.unwrap();
        assert_eq!("atoi1qqdxdqak4x96hzw6rkt48t4x5t84e209jmmlkp6t6lcx8aj6unr7zzv994h", response.account_identifier.address)
    }

    #[tokio::test]
    async fn test_bad_network() {
        let request = ConstructionDeriveRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet4".to_string(),
                sub_network_identifier: None
            },
            public_key: PublicKey {
                hex_bytes: "29bdea325f58cb4ad7493ba7bc12c36bafb381350f5fbea0357ad2b869793e95".to_string(),
                curve_type: CurveType::Edwards25519
            }
        };

        let server_options = Options {
            iota_endpoint: "https://api.lb-0.testnet.chrysalis2.com".to_string(),
            network: "testnet6".to_string(),
            mode: "online".to_string(),
            port: 3030
        };

        if let Ok(_) = construction_derive_request(request, server_options).await {
            panic!("expected bad network error")
        }
    }
}