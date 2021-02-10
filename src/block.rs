// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    consts,
    error::ApiError,
    filters::{handle, with_options},
    operations::*,
    options::Options,
    types::{Block, BlockIdentifier, BlockRequest, BlockResponse, Transaction, TransactionIdentifier},
};
use bee_message::prelude::UTXOInput;
use bee_rest_api::types::{AddressDto, OutputDto};
use iota;
use log::debug;
use std::str::FromStr;

use warp::Filter;

pub fn routes(options: Options) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post().and(
        warp::path!("block")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(block)),
    )
}

async fn block(block_request: BlockRequest, options: Options) -> Result<BlockResponse, ApiError> {
    debug!("/block");

    let network_identifier = block_request.network_identifier;
    if network_identifier.blockchain != consts::BLOCKCHAIN || network_identifier.network != options.network {
        return Err(ApiError::BadNetwork);
    }

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

    let milestone_index = block_request
        .block_identifier
        .index
        .ok_or_else(|| ApiError::BadMilestoneRequest)?;

    let milestone = match iota_client.get_milestone(milestone_index).await {
        Ok(milestone) => milestone,
        Err(_) => return Err(ApiError::UnableToGetMilestone),
    };

    if block_request.block_identifier.hash.is_some(){
        let block_request_hash = block_request.block_identifier.hash.unwrap();
        if (block_request_hash != "") && (block_request_hash != milestone.message_id.to_string()) {
            return Err(ApiError::BadMilestoneRequest);
        }
    }

    let block_identifier = BlockIdentifier {
        index: milestone_index,
        hash: milestone.message_id.to_string(),
    };

    let parent_block_identifier;
    if milestone_index == 1 {
        parent_block_identifier = BlockIdentifier {
            index: milestone.index as u64,
            hash: milestone.message_id.to_string(),
        };
    } else {
        let parent_milestone = match iota_client.get_milestone(milestone_index - 1).await {
            Ok(parent_milestone) => parent_milestone,
            Err(_) => return Err(ApiError::UnableToGetMilestone),
        };

        parent_block_identifier = BlockIdentifier {
            index: parent_milestone.index as u64,
            hash: parent_milestone.message_id.to_string(),
        };
    }

    let timestamp = milestone.timestamp;

    let utxo_changes = match iota_client.get_milestone_utxo_changes(milestone_index).await {
        Ok(utxo_changes) => utxo_changes,
        Err(_) => return Err(ApiError::UnableToGetMilestoneUTXOChanges),
    };

    let mut transactions = vec![];

    for output_id_str in utxo_changes.created_outputs {
        let output_id = UTXOInput::from_str(&output_id_str[..]).unwrap();

        let output = match iota_client.get_output(&output_id).await {
            Ok(output) => output,
            Err(_) => return Err(ApiError::UnableToGetOutput),
        };

        let is_spent = output.is_spent;

        let (amount, address) = match output.output {
            OutputDto::Treasury(_) => panic!("Can't be used as input"),
            OutputDto::SignatureLockedSingle(r) => match r.address {
                AddressDto::Ed25519(ed25519) => (r.amount, ed25519.address),
            },
            OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                AddressDto::Ed25519(ed25519) => (r.amount, ed25519.address),
            },
        };

        let transaction_identifier = TransactionIdentifier { hash: output_id_str };

        // todo: related_transactions (?)

        let mut operations = vec![];
        operations.push(created_utxo_operation(is_spent, address, amount));

        transactions.push(Transaction {
            transaction_identifier: transaction_identifier,
            operations: operations,
        });
    }

    for output_id_str in utxo_changes.consumed_outputs {
        let output_id = UTXOInput::from_str(&output_id_str[..]).unwrap();

        let output = match iota_client.get_output(&output_id).await {
            Ok(output) => output,
            Err(_) => return Err(ApiError::UnableToGetOutput),
        };

        let is_spent = output.is_spent;

        let (amount, address) = match output.output {
            OutputDto::Treasury(_) => panic!("Can't be used as input"),
            OutputDto::SignatureLockedSingle(r) => match r.address {
                AddressDto::Ed25519(ed25519) => (r.amount, ed25519.address),
            },
            OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                AddressDto::Ed25519(ed25519) => (r.amount, ed25519.address),
            },
        };

        let transaction_identifier = TransactionIdentifier { hash: output_id_str };

        // todo: related_transactions (?)

        let mut operations = vec![];
        operations.push(consumed_utxo_operation(is_spent, address, amount));

        transactions.push(Transaction {
            transaction_identifier: transaction_identifier,
            operations: operations,
        });
    }

    let block = Block {
        block_identifier: block_identifier,
        parent_block_identifier: parent_block_identifier,
        timestamp: timestamp,
        transactions: transactions,
    };

    let response = BlockResponse { block };

    Ok(response)
}
