// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, operations::*, options::Options, types::{Block, BlockIdentifier, Transaction, TransactionIdentifier}, build_iota_client, require_online_mode, is_bad_network};
use bee_message::prelude::{Ed25519Address, OutputId, Payload, Input, Address};
use iota::{AddressDto, OutputDto, Client};
use log::debug;
use crate::types::{NetworkIdentifier, PartialBlockIdentifier};
use serde::{Deserialize, Serialize};
use iota::MessageId;
use bee_message::payload::transaction::{ Essence};
use bee_message::Message;
use bee_message::prelude::Output;

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
        .ok_or(ApiError::BadMilestoneRequest)?;

    let milestone = match iota_client.get_milestone(milestone_index).await {
        Ok(milestone) => milestone,
        Err(_) => return Err(ApiError::UnableToGetMilestone(milestone_index)),
    };

    // TODO: Do we really need this check?
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

    let messages_from_created_outputs = messages_from_created_outputs(milestone_index, &iota_client).await?;
    let rosetta_transactions = build_rosetta_transactions(messages_from_created_outputs, &iota_client, &options).await?;

    let block = Block {
        block_identifier,
        parent_block_identifier,
        timestamp,
        transactions: rosetta_transactions,
        metadata: None
    };

    let response = BlockResponse { block };

    Ok(response)
}




async fn messages_from_created_outputs(milestone_index: u32, iota_client: &Client) -> Result<Vec<Message>, ApiError> {
    let created_outputs = match iota_client.get_milestone_utxo_changes(milestone_index).await {
        Ok(utxo_changes) => utxo_changes.created_outputs,
        Err(_) => return Err(ApiError::UnableToGetMilestoneUTXOChanges),
    };

    let mut ret = Vec::new();
    for created_output_id_string in created_outputs {
        let created_output_id = created_output_id_string.parse::<OutputId>().map_err(|e| ApiError::BeeMessageError(e))?;
        let output_response = iota_client.get_output(&created_output_id.into()).await.map_err(|e| ApiError::IotaClientError(e))?;
        let message = {
            let message_id = output_response.message_id.parse::<MessageId>().map_err(|e| ApiError::BeeMessageError(e))?;
            iota_client.get_message().data(&message_id).await.map_err(|e| ApiError::IotaClientError(e))?
        };
        ret.push(message);
    }

    Ok(ret)
}

async fn build_rosetta_transactions(messages: Vec<Message>, iota_client: &Client, options: &Options) -> Result<Vec<Transaction>, ApiError> {
    let mut ret = Vec::new();

    for message in messages {
        let message: Message = message;

        let transaction_payload = match message.payload() {
            Some(Payload::Transaction(t)) => t,
            _ => return Err(ApiError::NotImplemented) // NOT SUPPORTED
        };

        let regular_essence = match transaction_payload.essence() {
            Essence::Regular(r) => r,
            _ => return Err(ApiError::NotImplemented), // NOT SUPPORTED
        };

        let mut operation_counter = 0;

        let mut utxo_input_operations = {
            let mut ret = Vec::new();

            for input in regular_essence.inputs() {

                let utxo_input = match input {
                    Input::UTXO(i) => i,
                    _ => return Err(ApiError::NotImplemented), // NOT SUPPORTED
                };

                let output_info = iota_client.get_output(&utxo_input).await?;

                let (amount, ed25519_address) = match output_info.output {
                    OutputDto::Treasury(_) => panic!("Can't be used as input"),
                    OutputDto::SignatureLockedSingle(r) => match r.address {
                        AddressDto::Ed25519(addr) => (r.amount, addr.address)
                    },
                    OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                        AddressDto::Ed25519(addr) => (r.amount, addr.address)
                    },
                };

                ret.push(utxo_input_operation(
                    output_info.transaction_id,
                    Address::Ed25519(ed25519_address.parse::<Ed25519Address>()?).to_bech32(&options.bech32_hrp),
                    amount,
                    output_info.output_index,
                    operation_counter,
                    true,
                    true,
                ));

                operation_counter += 1;
            }

            ret
        };

        let mut utxo_output_operations = {
            let mut ret = Vec::new();

            for output in regular_essence.outputs() {

                let output: Output = output.clone();

                let (amount, ed25519_address) = match output {
                    Output::Treasury(_) => panic!("Can't be used as input"),
                    Output::SignatureLockedSingle(r) => match r.address() {
                        Address::Ed25519(addr) => (r.amount(), *addr),
                        _ =>  panic!("Can't be used as address"),
                    },
                    Output::SignatureLockedDustAllowance(r) => match r.address() {
                        Address::Ed25519(addr) => (r.amount(), *addr),
                        _ => panic!("Can't be used as address"),
                    },
                    _ => panic!("Can't be used as output"),
                };

                ret.push(utxo_output_operation(
                    Address::Ed25519(ed25519_address).to_bech32(&options.bech32_hrp),
                    amount,
                    operation_counter,
                    true
                ));

                operation_counter += 1;
            }

            ret
        };

        utxo_input_operations.append(&mut utxo_output_operations);

        let transaction = Transaction {
            transaction_identifier: TransactionIdentifier { hash: transaction_payload.id().to_string() },
            operations: utxo_input_operations,
            metadata: None
        };

        ret.push(transaction);
    }

    Ok(ret)
}