// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, operations::*, options::Options, types::{Block, BlockIdentifier, Transaction, TransactionIdentifier}, build_iota_client, require_online_mode, is_bad_network};
use bee_message::prelude::{Ed25519Address};
use iota::{UTXOInput, OutputResponse, AddressDto, OutputDto};
use log::debug;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use crate::types::{NetworkIdentifier, PartialBlockIdentifier};
use serde::{Deserialize, Serialize};
use crate::consts::ONLINE_MODE;
use iota::MessageId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockRequest {
    pub network_identifier: NetworkIdentifier,
    pub block_identifier: PartialBlockIdentifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockResponse {
    pub block: Block,
    // pub other_transactions: Vec<TransactionIdentifier>
}

pub async fn block(block_request: BlockRequest, options: Options) -> Result<BlockResponse, ApiError> {
    debug!("/block");

    let _ = require_online_mode(&options)?;
    is_bad_network(&options, &block_request.network_identifier)?;

    let iota_client = build_iota_client(&options).await?;

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
            index: milestone.index,
            hash: milestone.message_id.to_string(),
        };
    } else {
        let parent_milestone = match iota_client.get_milestone(milestone_index - 1).await {
            Ok(parent_milestone) => parent_milestone,
            Err(_) => return Err(ApiError::UnableToGetMilestone(milestone_index - 1)),
        };

        parent_block_identifier = BlockIdentifier {
            index: parent_milestone.index,
            hash: parent_milestone.message_id.to_string(),
        };
    }

    let timestamp = milestone.timestamp * 1000;

    let utxo_changes = match iota_client.get_milestone_utxo_changes(milestone_index).await {
        Ok(utxo_changes) => utxo_changes,
        Err(_) => return Err(ApiError::UnableToGetMilestoneUTXOChanges),
    };

    let mut transaction_hashset = HashSet::new();
    let mut output_hashmap: HashMap<String, (Vec<OutputResponse>)> = HashMap::new();

    // loop over created_outputs
    for output_id_str in utxo_changes.created_outputs {
        let output_id = UTXOInput::from_str(&output_id_str[..]).unwrap();

        let output = match iota_client.get_output(&output_id).await {
            Ok(output) => output,
            Err(_) => return Err(ApiError::UnableToGetOutput),
        };

        let message_id = output.clone().message_id;
        let message = iota_client.get_message()
            .data(&MessageId::from_str(&message_id[..]).unwrap()).await.unwrap();

        // todo: finish this
        // match message.payload() {
        //     Some(p) => match p {
        //         get list of inputs
        //     },
        //     None => panic!("no payload!")
        // }

        // perhaps we will also need to refactor output_hashmap so it contains
        // HashMap<String, (Vec<OutputResponse>, Vec<Input>)>
        // or something similar, where String is the Transaction ID

        let transaction_id = output.clone().transaction_id;
        transaction_hashset.insert(transaction_id.clone());

        // populate output_hashmap
        match output_hashmap.get(&transaction_id[..]) {
            None => {
                output_hashmap.insert(transaction_id, vec![output.clone()]);
                ()
            },
            Some(output_vec) => {
                let mut output_vec_clone = output_vec.clone();
                output_vec_clone.push(output.clone());

                // update output_vec_value with output_vec_clone
                let output_vec_value = output_hashmap.entry(transaction_id).or_insert(vec![output]);

                *output_vec_value = output_vec_clone;
            }
        }

    }

    let mut transactions = vec![];

    for transaction in transaction_hashset {
        let transaction_identifier = TransactionIdentifier { hash: transaction.clone() };

        let mut operations = vec![];

        match output_hashmap.get(&transaction[..]) {
            Some(output_vec) => {
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

                    // todo: treat timeout on this unrwap
                    let bech32_hrp = iota_client.get_bech32_hrp().await.unwrap();
                    let bech32_address = Ed25519Address::from_str(&ed25519_address).unwrap().to_bech32(&bech32_hrp[..]);

                    let online = options.mode == ONLINE_MODE;
                    operations.push(utxo_output_operation(bech32_address, amount, output.output_index, operation_counter));
                    operation_counter = operation_counter + 1;
                }
            },
            None => panic!("no output_vec found on hashmap")
        }

        transactions.push(Transaction {
            transaction_identifier: transaction_identifier,
            operations: operations,
            metadata: None
        });
    }

    let block = Block {
        block_identifier: block_identifier,
        parent_block_identifier: parent_block_identifier,
        timestamp: timestamp,
        transactions: transactions,
        metadata: None
    };

    let response = BlockResponse { block };

    Ok(response)
}
