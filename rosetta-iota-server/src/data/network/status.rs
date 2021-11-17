// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{build_client, get_confirmed_milestone, get_peers},
    config::RosettaConfig,
    error::ApiError,
    is_offline_mode_enabled, is_wrong_network,
    types::{NetworkIdentifier, *},
};
use crate::client::{get_pruning_index, get_latest_milestone_index};

use log::debug;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkStatusRequest {
    pub network_identifier: NetworkIdentifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkStatusResponse {
    pub current_block_identifier: BlockIdentifier,
    pub current_block_timestamp: u64,
    pub genesis_block_identifier: BlockIdentifier,
    pub oldest_block_identifier: BlockIdentifier,
    pub sync_status: SyncStatus,
    pub peers: Vec<Peer>,
}

pub async fn network_status(request: NetworkStatusRequest, rosetta_config: RosettaConfig) -> Result<NetworkStatusResponse, ApiError> {
    debug!("/network/status");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()));
    }

    if is_offline_mode_enabled(&rosetta_config) {
        return Err(ApiError::NonRetriable(
            "endpoint does not support offline mode".to_string(),
        ));
    }

    let client = build_client(&rosetta_config).await?;

    let confirmed_milestone = get_confirmed_milestone(&client).await?;
    let oldest_block = get_pruning_index(&client).await? + 1;
    let latest_milestone_index = get_latest_milestone_index(&client).await?;

    let mut peers = vec![];
    for peer in get_peers(&client).await? {
        peers.push(Peer {
            peer_id: peer.id,
        });
    }

    let response = NetworkStatusResponse {
        current_block_identifier: BlockIdentifier {
            index: confirmed_milestone.index,
            hash: confirmed_milestone.index.to_string(),
        },
        current_block_timestamp: confirmed_milestone.timestamp * 1000,
        genesis_block_identifier: BlockIdentifier {
            index: 1,
            hash: 1.to_string(),
        },
        oldest_block_identifier: BlockIdentifier {
            index: oldest_block,
            hash: oldest_block.to_string(),
        },
        sync_status: SyncStatus {
            current_index: confirmed_milestone.index as u64,
            target_index: latest_milestone_index as u64,
            synced: latest_milestone_index - confirmed_milestone.index == 0
        },
        peers,
    };

    Ok(response)
}