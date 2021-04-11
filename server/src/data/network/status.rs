// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{ error::ApiError, is_wrong_network, options::Options, types::{NetworkIdentifier, *}, is_offline_mode_enabled};
use crate::client::{build_client, get_latest_milestone, get_peers, get_genesis_milestone};

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

pub async fn network_status(
    request: NetworkStatusRequest,
    options: Options,
) -> Result<NetworkStatusResponse, ApiError> {
    debug!("/network/status");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()))
    }

    if is_offline_mode_enabled(&options) {
        return Err(ApiError::NonRetriable("endpoint does not support offline mode".to_string()))
    }

    let client = build_client(&options).await?;

    let genesis_milestone = get_genesis_milestone( &client).await?;

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
        index: genesis_milestone.index,
        hash: genesis_milestone.message_id.to_string(),
    };

    let current_block_identifier = BlockIdentifier {
        index: latest_milestone.index,
        hash: latest_milestone.message_id.to_string(),
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
    use crate::options::RosettaMode;

    #[tokio::test]
    async fn test_network_status() {
        let request = NetworkStatusRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet7".to_string(),
                sub_network_identifier: None,
            },
        };

        let server_options = Options {
            node: "https://api.hornet-rosetta.testnet.chrysalis2.com".to_string(),
            network: "testnet7".to_string(),
            indexation: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };
        let _response = network_status(request, server_options).await.unwrap();

        // todo: assertions
    }
}
