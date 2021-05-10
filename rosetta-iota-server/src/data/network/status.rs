// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{build_client, get_latest_milestone, get_peers},
    config::Config,
    error::ApiError,
    is_offline_mode_enabled, is_wrong_network,
    types::{NetworkIdentifier, *},
};

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkStatusRequest {
    pub network_identifier: NetworkIdentifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkStatusResponse {
    pub current_block_identifier: BlockIdentifier,
    pub current_block_timestamp: u64,
    pub genesis_block_identifier: BlockIdentifier,
    pub peers: Vec<Peer>,
}

pub async fn network_status(request: NetworkStatusRequest, options: Config) -> Result<NetworkStatusResponse, ApiError> {
    debug!("/network/status");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()));
    }

    if is_offline_mode_enabled(&options) {
        return Err(ApiError::NonRetriable(
            "endpoint does not support offline mode".to_string(),
        ));
    }

    let client = build_client(&options).await?;

    let latest_milestone = get_latest_milestone(&client).await?;

    let current_block_timestamp = latest_milestone.timestamp * 1000;

    let mut peers = vec![];
    for peer in get_peers(&client).await? {
        peers.push(Peer {
            peer_id: peer.id,
            metadata: PeerMetadata {
                multi_addresses: peer.multi_addresses,
                alias: peer.alias,
                connected: peer.connected,
            },
        });
    }

    let genesis_block_identifier = BlockIdentifier {
        index: 1,
        hash: 1.to_string(),
    };

    let current_block_identifier = BlockIdentifier {
        index: latest_milestone.index,
        hash: latest_milestone.index.to_string(),
    };

    let response = NetworkStatusResponse {
        current_block_identifier,
        current_block_timestamp,
        genesis_block_identifier,
        peers,
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RosettaMode;

    use crate::mocked_node::start_mocked_node;
    use serial_test::serial;
    use tokio::sync::oneshot;

    #[tokio::test]
    #[serial]
    async fn test_network_status() {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        tokio::task::spawn(start_mocked_node(shutdown_rx));

        let request = NetworkStatusRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet7".to_string(),
                sub_network_identifier: None,
            },
        };

        let server_options = Config {
            node_url: "http://127.0.0.1:3029".to_string(),
            network: "testnet7".to_string(),
            tx_tag: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };

        let response = network_status(request, server_options).await.unwrap();

        assert_eq!(68910, response.current_block_identifier.index);
        assert_eq!(
            "68910",
            response.current_block_identifier.hash
        );
        assert_eq!(1618486402000, response.current_block_timestamp);

        let _ = shutdown_tx.send(());
    }
}
