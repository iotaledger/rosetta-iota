// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{consts, error::ApiError, filters::EmptyRequest, options::Options, types::NetworkIdentifier};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}

pub async fn network_list(_empty: EmptyRequest, options: Options) -> Result<NetworkListResponse, ApiError> {
    debug!("/network/list");

    let response = NetworkListResponse {
        network_identifiers: vec![NetworkIdentifier {
            blockchain: consts::BLOCKCHAIN.to_string(),
            network: options.network.clone(),
            sub_network_identifier: None,
        }],
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::RosettaMode;

    #[tokio::test]
    async fn test_network_list() {
        let server_options = Options {
            node: "https://api.hornet-rosetta.testnet.chrysalis2.com".to_string(),
            network: "testnet6".to_string(),
            indexation: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };
        let response = network_list(EmptyRequest, server_options).await.unwrap();

        assert_eq!("iota", response.network_identifiers[0].blockchain);
        assert_eq!("testnet6", response.network_identifiers[0].network);
        assert_eq!(false, response.network_identifiers[0].sub_network_identifier.is_some())
    }
}
