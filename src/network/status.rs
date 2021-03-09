// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{consts, error::ApiError,  options::Options, types::{
    BlockIdentifier, NetworkRequest,
    NetworkStatusResponse, Peer, PeerMetadata,
}, build_iota_client, require_online_mode};
use bee_message::prelude::{MESSAGE_ID_LENGTH};
use iota::{self, client::MilestoneResponse, MessageId};
use log::debug;

pub async fn network_status(network_request: NetworkRequest, options: Options) -> Result<NetworkStatusResponse, ApiError> {
    debug!("/network/status");

    let _ = require_online_mode(&options)?;

    if network_request.network_identifier.blockchain != consts::BLOCKCHAIN
        || network_request.network_identifier.network != options.network
    {
        return Err(ApiError::BadNetwork);
    }

    let iota_client = build_iota_client(&options, true).await?;

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