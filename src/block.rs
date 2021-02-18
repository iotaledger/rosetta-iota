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
use bee_message::prelude::{UTXOInput, Ed25519Address};
use bee_rest_api::types::{AddressDto, OutputDto};
use iota;
use log::debug;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};

use warp::Filter;
use bee_rest_api::handlers::output::OutputResponse;

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

    if options.mode != consts::ONLINE_MODE {
        return Err(ApiError::UnavailableOffline);
    }

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
        Err(_) => return Err(ApiError::UnableToGetMilestone(milestone_index)),
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
            Err(_) => return Err(ApiError::UnableToGetMilestone(milestone_index - 1)),
        };

        parent_block_identifier = BlockIdentifier {
            index: parent_milestone.index as u64,
            hash: parent_milestone.message_id.to_string(),
        };
    }

    let timestamp = milestone.timestamp * 1000;

    let utxo_changes = match iota_client.get_milestone_utxo_changes(milestone_index).await {
        Ok(utxo_changes) => utxo_changes,
        Err(_) => return Err(ApiError::UnableToGetMilestoneUTXOChanges),
    };

    // all_outputs has both created and consumed, but we still keep track of which is which
    // by checking if output_counter > n_consumed_outputs
    // very unelegant! todo: refactor
    let mut output_counter = 0;
    let n_consumed_outputs = utxo_changes.consumed_outputs.len();
    let mut all_outputs = utxo_changes.consumed_outputs;
    all_outputs.extend(utxo_changes.created_outputs);

    let mut transaction_hashset = HashSet::new();
    let mut output_hashmap: HashMap<String, (Vec<OutputResponse>, bool)> = HashMap::new();

    for output_id_str in all_outputs {
        let output_id = UTXOInput::from_str(&output_id_str[..]).unwrap();

        let output = match iota_client.get_output(&output_id).await {
            Ok(output) => output,
            Err(_) => return Err(ApiError::UnableToGetOutput),
        };

        let transaction_id = output.clone().transaction_id;
        transaction_hashset.insert(transaction_id.clone());

        match output_hashmap.get(&transaction_id[..]) {
            None => {
                //                                                                todo: refactor
                output_hashmap.insert(transaction_id, (vec![output.clone()], output_counter > n_consumed_outputs));
                ()
            },
            Some((output_vec, _)) => {
                let mut output_vec_clone = output_vec.clone();
                output_vec_clone.push(output.clone());

                // update output_vec_value with output_vec_clone
                //                                                                                                                           todo: refactor
                let output_vec_value = output_hashmap.entry(transaction_id).or_insert((vec![output], output_counter > n_consumed_outputs));

                // this is ok                          todo: refactor
                *output_vec_value = (output_vec_clone, output_counter > n_consumed_outputs);
            }
        }

        output_counter = output_counter + 1;
    }

    let mut transactions = vec![];

    for transaction in transaction_hashset {
        let transaction_identifier = TransactionIdentifier { hash: transaction.clone() };

        let mut operations = vec![];
        
        match output_hashmap.get(&transaction[..]) {
            Some((output_vec, consumed)) => {
                let mut operation_counter = 0;

                for output in output_vec {
                    let is_spent = output.is_spent;

                    let (amount, ed25519_address) = match output.clone().output {
                        OutputDto::Treasury(_) => panic!("Can't be used as input"),
                        OutputDto::SignatureLockedSingle(r) => match r.address {
                            AddressDto::Ed25519(ed25519) => (r.amount, ed25519.address),
                        },
                        OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                            AddressDto::Ed25519(ed25519) => (r.amount, ed25519.address),
                        },
                    };

                    let bech32_hrp = iota_client.get_bech32_hrp().await.unwrap();
                    let bech32_address = Ed25519Address::from_str(&ed25519_address).unwrap().to_bech32(&bech32_hrp[..]);

                    // todo: refactor
                    match consumed {
                        true => operations.push(consumed_utxo_operation(is_spent, bech32_address, amount, output.output_index, operation_counter)),
                        false => operations.push(created_utxo_operation(is_spent, bech32_address, amount, output.output_index, operation_counter)),
                    }
                    operation_counter = operation_counter + 1;
                }
            },
            None => panic!("no output_vec found on hashmap")
        }

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