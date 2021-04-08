// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    build_iota_client,
    error::ApiError,
    is_bad_network,
    options::Options,
    require_online_mode,
    types::{NetworkIdentifier, *},
};

use bee_message::prelude::MESSAGE_ID_LENGTH;

use iota::{self, client::MilestoneResponse, MessageId};

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
    network_request: NetworkStatusRequest,
    options: Options,
) -> Result<NetworkStatusResponse, ApiError> {
    debug!("/network/status");

    let _ = require_online_mode(&options)?;

    is_bad_network(&options, &network_request.network_identifier)?;

    let iota_client = build_iota_client(&options).await?;

    let node_info = match iota_client.get_info().await {
        Ok(node_info) => node_info,
        Err(_) => return Err(ApiError::UnableToGetNodeInfo),
    };

    let genesis_milestone = match iota_client.get_milestone(1).await {
        Ok(genesis_milestone) => genesis_milestone,
        Err(_) => MilestoneResponse {
            index: 1,
            message_id: MessageId::new([0; MESSAGE_ID_LENGTH]),
            timestamp: 0,
        },
    };

    let latest_milestone_index = node_info.latest_milestone_index;
    let latest_milestone = match iota_client.get_milestone(latest_milestone_index).await {
        Ok(latest_milestone) => latest_milestone,
        Err(_) => return Err(ApiError::UnableToGetMilestone(latest_milestone_index)),
    };

    let current_block_timestamp = latest_milestone.timestamp * 1000;
    let peers_bee = match iota_client.get_peers().await {
        Ok(peers) => peers,
        Err(_) => return Err(ApiError::UnableToGetPeers),
    };

    let mut peers = vec![];
    for peer_bee in peers_bee {
        peers.push(Peer {
            peer_id: peer_bee.id,
            metadata: PeerMetadata {
                multi_addresses: peer_bee.multi_addresses,
                alias: peer_bee.alias,
                connected: peer_bee.connected,
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
