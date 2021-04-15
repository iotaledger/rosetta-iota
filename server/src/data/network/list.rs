// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{consts, error::ApiError, filters::EmptyRequest, config::Config, types::NetworkIdentifier};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}

pub async fn network_list(_empty: EmptyRequest, options: Config) -> Result<NetworkListResponse, ApiError> {
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
    use crate::config::RosettaMode;
    use crate::mocknet::start_mocknet_node;

    #[tokio::test]
    async fn test_network_list() {
        tokio::task::spawn(start_mocknet_node());

        let server_options = Config {
            node_url: "http://127.0.0.1:3029".to_string(),
            network: "testnet7".to_string(),
            tx_tag: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };
        let response = network_list(EmptyRequest, server_options).await.unwrap();

        assert_eq!("iota", response.network_identifiers[0].blockchain);
        assert_eq!("testnet7", response.network_identifiers[0].network);
        assert_eq!(false, response.network_identifiers[0].sub_network_identifier.is_some())
    }
}
