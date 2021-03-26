// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, operations::*, options::Options, types::{Block, BlockIdentifier, Transaction, TransactionIdentifier}, build_iota_client, require_online_mode, is_bad_network};
use bee_message::prelude::*;
use iota::{Client};
use log::debug;
use crate::types::{NetworkIdentifier, PartialBlockIdentifier};
use serde::{Deserialize, Serialize};
use iota::MessageId;
use bee_message::payload::transaction::{ Essence};
use bee_message::Message;
use bee_message::prelude::Output;
use std::convert::TryFrom;
use bee_rest_api::types::responses::OutputResponse;
use std::collections::HashSet;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockRequest {
    pub network_identifier: NetworkIdentifier,
    pub block_identifier: PartialBlockIdentifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockResponse {
    pub block: Block
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

    let created_outputs = filter_created_outputs(milestone_index, &iota_client).await?;
    let transactions = process_created_outputs(created_outputs, &iota_client, &options).await?;

    let block = Block {
        block_identifier,
        parent_block_identifier,
        timestamp,
        transactions,
        metadata: None
    };

    let response = BlockResponse { block };

    Ok(response)
}

struct CreatedOutput {
    pub output_id: OutputId,
    pub output_response: OutputResponse,
    pub message: Message,
}

async fn filter_created_outputs(
    milestone_index: u32,
    iota_client: &Client
) -> Result<Vec<CreatedOutput>, ApiError> {
    let created_outputs = match iota_client.get_milestone_utxo_changes(milestone_index).await {
        Ok(utxo_changes) => utxo_changes.created_outputs,
        Err(_) => return Err(ApiError::UnableToGetMilestoneUTXOChanges),
    };

    let mut visited_message_ids = HashSet::new();
    let mut created_outputs_to_process = Vec::new();
    for created_output_id_string in created_outputs {

        let output_id = created_output_id_string.parse::<OutputId>().map_err(|e| ApiError::BeeMessageError(e))?;
        let output_response = iota_client.get_output(&output_id.into()).await.map_err(|e| ApiError::IotaClientError(e))?;

        let message_id_string = output_response.message_id.clone();
        if !visited_message_ids.contains(&message_id_string) {
            visited_message_ids.insert(message_id_string.clone());

            let message = {
                let message_id = message_id_string.parse::<MessageId>().map_err(|e| ApiError::BeeMessageError(e))?;
                iota_client.get_message().data(&message_id).await.map_err(|e| ApiError::IotaClientError(e))?
            };

            let created_output = CreatedOutput { output_id, output_response, message };

            created_outputs_to_process.push(created_output);
        }
    }

    Ok(created_outputs_to_process)
}

async fn process_created_outputs(created_outputs: Vec<CreatedOutput>, iota_client: &Client, options: &Options) -> Result<Vec<Transaction>, ApiError>{
    let mut built_transactions = Vec::new();

    for created_output in created_outputs {
        let created_output: CreatedOutput = created_output;

        let built_transaction = match created_output.message.payload() {
            Some(Payload::Transaction(t)) => from_transaction(t, iota_client, options).await?,
            Some(Payload::Milestone(_)) => from_milestone(&created_output, options).await?,
            _ => return Err(ApiError::NotImplemented) // NOT SUPPORTED
        };

        built_transactions.push(built_transaction);
    }

    Ok(built_transactions)
}

async fn from_transaction(transaction_payload: &TransactionPayload, iota_client: &Client, options: &Options) -> Result<Transaction, ApiError> {
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

            let output = Output::try_from(&output_info.output).map_err(|_| ApiError::NotImplemented)?;

            let (amount, ed25519_address) = address_and_balance_of_output(&output).await;

            ret.push(utxo_input_operation(
                output_info.transaction_id,
                Address::Ed25519(ed25519_address).to_bech32(&options.bech32_hrp),
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

            let (amount, ed25519_address) = address_and_balance_of_output(&output).await;

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

    Ok(transaction)

}

async fn from_milestone(created_output: &CreatedOutput, options: &Options) -> Result<Transaction, ApiError> {

    let output = Output::try_from(&created_output.output_response.output).map_err(|_| ApiError::NotImplemented)?;
    let (amount, ed25519_address) = address_and_balance_of_output(&output).await;

    let mint_operation = utxo_output_operation(
        Address::Ed25519(ed25519_address).to_bech32(&options.bech32_hrp),
        amount,
        0,
        true
    );

    let transaction = Transaction {
        transaction_identifier: TransactionIdentifier { hash: created_output.output_response.transaction_id.clone() },
        operations: vec![mint_operation],
        metadata: None
    };

    Ok(transaction)
}

async fn address_and_balance_of_output(output: &Output) -> (u64, Ed25519Address) {
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
    (amount, ed25519_address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_block() {
        let request = BlockRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet6".to_string(),
                sub_network_identifier: None
            },
            block_identifier: PartialBlockIdentifier {
                index: Some(10),
                hash: None
            }
        };

        let server_options = Options {
            iota_endpoint: "https://api.hornet-rosetta.testnet.chrysalis2.com".to_string(),
            network: "testnet6".to_string(),
            indexation: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: "online".to_string(),
            port: 3030
        };
        let response = block(request, server_options).await.unwrap();
        assert_eq!(10, response.block.block_identifier.index);
        assert_eq!("62dd0dfde19584d250ea34157ee17945996380b792944bbea17b011ddc3225e3", response.block.block_identifier.hash);
        assert_eq!(9, response.block.parent_block_identifier.index);
        assert_eq!("b26f8a43e1e40c62f5c4984e1e778650a93ee53d915559adc032de7bfe30291f", response.block.parent_block_identifier.hash);
        assert_eq!(1614779517000, response.block.timestamp);
        assert_eq!(false, response.block.metadata.is_some());
    }
}