// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, is_wrong_network, operations::*, config::Config, types::{Block, BlockIdentifier, NetworkIdentifier, PartialBlockIdentifier, Transaction, TransactionIdentifier}, is_offline_mode_enabled};

use bee_message::{
    payload::transaction::Essence,
    prelude::{Output, *},
    Message,
};
use bee_rest_api::types::responses::OutputResponse;

use iota::Client;

use log::debug;
use serde::{Deserialize, Serialize};

use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryFrom,
};
use crate::client::{build_client, get_milestone};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockRequest {
    pub network_identifier: NetworkIdentifier,
    pub block_identifier: PartialBlockIdentifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockResponse {
    pub block: Block,
}

pub async fn block(request: BlockRequest, options: Config) -> Result<BlockResponse, ApiError> {
    debug!("/block");

    if is_wrong_network(&options, &request.network_identifier) {
        return Err(ApiError::NonRetriable("wrong network".to_string()))
    }

    if is_offline_mode_enabled(&options) {
        return Err(ApiError::NonRetriable("endpoint does not support offline mode".to_string()))
    }

    let client = build_client(&options).await?;

    let milestone_index = request
        .block_identifier
        .index
        .ok_or(ApiError::NonRetriable("block index not set".to_string()))?;

    let milestone = get_milestone(milestone_index, &client).await?;

    if let Some(hash) = request.block_identifier.hash {
        if hash != milestone.message_id.to_string() {
            return Err(ApiError::NonRetriable("provided block hash does not relate to the provided block index".to_string()));
        }
    }

    let block_identifier = BlockIdentifier {
        index: milestone_index,
        hash: milestone.message_id.to_string(),
    };

    let parent_block_identifier = {
        let (index, hash) = if milestone_index == 1 {
            (milestone_index, milestone.message_id.to_string())
        } else {
            let parent_milestone = get_milestone(milestone_index - 1, &client).await?;
            (parent_milestone.index, parent_milestone.message_id.to_string())
        };
        BlockIdentifier { index, hash }
    };

    let timestamp = milestone.timestamp * 1000;

    let messages = messages_of_created_outputs(milestone_index, &client).await?;
    let transactions = build_rosetta_transactions(messages, &client, &options).await?;

    let block = Block {
        block_identifier,
        parent_block_identifier,
        timestamp,
        transactions,
        metadata: None,
    };

    let response = BlockResponse { block };

    Ok(response)
}

struct MessageInfo {
    pub message: Message,
    pub created_outputs: Vec<CreatedOutput>,
}

struct CreatedOutput {
    pub output_id: OutputId,
    pub output_response: OutputResponse,
}

async fn messages_of_created_outputs(
    milestone_index: u32,
    iota_client: &Client,
) -> Result<HashMap<MessageId, MessageInfo>, ApiError> {
    let mut messages_of_created_outputs = HashMap::new();

    let created_outputs = match iota_client.get_milestone_utxo_changes(milestone_index).await {
        Ok(utxo_changes) => utxo_changes.created_outputs,
        Err(e) => return Err(ApiError::NonRetriable(format!("can not get uxto-changes: {}", e))),
    };

    for output_id_string in created_outputs {
        let output_id = output_id_string
            .parse::<OutputId>()
            .map_err(|e| ApiError::NonRetriable(format!("can not parse output id: {}", e)))?;

        let output_response = iota_client
            .get_output(&output_id.into())
            .await
            .map_err(|e| ApiError::NonRetriable(format!("can not get output information: {}", e)))?;

        let message_id = output_response
            .message_id
            .parse::<MessageId>()
            .map_err(|e| ApiError::NonRetriable(format!("can not parse message id: {}", e)))?;

        let created_output = CreatedOutput {
            output_id,
            output_response,
        };

        match messages_of_created_outputs.entry(message_id) {
            Entry::Occupied(mut entry) => {
                let message_info: &mut MessageInfo = entry.get_mut();
                message_info.created_outputs.push(created_output);
            }
            Entry::Vacant(entry) => {
                let message = iota_client
                    .get_message()
                    .data(&message_id)
                    .await
                    .map_err(|e| ApiError::NonRetriable(format!("can not get message id: {}", e)))?;
                let created_outputs = vec![created_output];
                let message_info = MessageInfo {
                    message,
                    created_outputs,
                };
                entry.insert(message_info);
            }
        }
    }

    Ok(messages_of_created_outputs)
}

async fn build_rosetta_transactions(
    messages: HashMap<MessageId, MessageInfo>,
    iota_client: &Client,
    options: &Config,
) -> Result<Vec<Transaction>, ApiError> {
    let mut built_transactions = Vec::new();

    for (_message_id, message_info) in messages {
        let transaction = match message_info.message.payload() {
            Some(Payload::Transaction(t)) => from_transaction(t, iota_client, options).await?,
            Some(Payload::Milestone(m)) => from_milestone(m, &message_info.created_outputs, options).await?,
            _ => return Err(ApiError::NonRetriable("payload type not supported".to_string())), // NOT SUPPORTED
        };
        built_transactions.push(transaction);
    }

    Ok(built_transactions)
}

async fn from_transaction(
    transaction_payload: &TransactionPayload,
    iota_client: &Client,
    options: &Config,
) -> Result<Transaction, ApiError> {
    let regular_essence = match transaction_payload.essence() {
        Essence::Regular(r) => r,
        _ => return Err(ApiError::NonRetriable("essence type not supported".to_string())), // NOT SUPPORTED
    };

    let mut operations = Vec::new();

    for input in regular_essence.inputs() {
        let utxo_input = match input {
            Input::Utxo(i) => i,
            _ => return Err(ApiError::NonRetriable("input type not supported".to_string())), // NOT SUPPORTED
        };

        let output_info = iota_client.get_output(&utxo_input).await.map_err(|e| ApiError::NonRetriable(format!("can not get input information: {}", e)))?;

        let output = Output::try_from(&output_info.output).map_err(|e| ApiError::NonRetriable(format!("can not parse output from output information: {}", e)))?;

        let (amount, ed25519_address) = address_and_balance_of_output(&output).await;

        operations.push(utxo_input_operation(
            output_info.transaction_id,
            Address::Ed25519(ed25519_address).to_bech32(&options.bech32_hrp),
            amount,
            output_info.output_index,
            operations.len(),
            true,
            true,
        ));
    }

    for output in regular_essence.outputs() {
        let output: Output = output.clone();

        let (amount, ed25519_address) = address_and_balance_of_output(&output).await;

        operations.push(utxo_output_operation(
            Address::Ed25519(ed25519_address).to_bech32(&options.bech32_hrp),
            amount,
            operations.len(),
            true,
        ));
    }

    let transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: transaction_payload.id().to_string(),
        },
        operations,
        metadata: None,
    };

    Ok(transaction)
}

async fn from_milestone(
    milestone_payload: &MilestonePayload,
    created_outputs: &Vec<CreatedOutput>,
    options: &Config,
) -> Result<Transaction, ApiError> {
    let mut operations = Vec::new();

    for created_output in created_outputs {
        let output = Output::try_from(&created_output.output_response.output).map_err(|_| ApiError::NonRetriable("can not convert output".to_string()))?;
        let (amount, ed25519_address) = address_and_balance_of_output(&output).await;
        let transaction_id = created_output
            .output_response
            .transaction_id
            .parse::<TransactionId>()
            .map_err(|e| ApiError::NonRetriable(format!("can not parse transaction id: {}", e)))?;
        let output_index = created_output.output_response.output_index;

        let mint_operation = utxo_input_operation(
            transaction_id.to_string(),
            Address::Ed25519(ed25519_address).to_bech32(&options.bech32_hrp),
            amount,
            output_index,
            operations.len(),
            false,
            true,
        );

        operations.push(mint_operation);
    }

    let transaction = Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: milestone_payload.id().to_string(),
        },
        operations,
        metadata: None,
    };

    Ok(transaction)
}

async fn address_and_balance_of_output(output: &Output) -> (u64, Ed25519Address) {
    let (amount, ed25519_address) = match output {
        Output::Treasury(_) => panic!("Can't be used as input"),
        Output::SignatureLockedSingle(r) => match r.address() {
            Address::Ed25519(addr) => (r.amount(), *addr),
            _ => panic!("Can't be used as address"),
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
    use crate::config::RosettaMode;

    #[tokio::test]
    async fn test_block() {
        let request = BlockRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet7".to_string(),
                sub_network_identifier: None,
            },
            block_identifier: PartialBlockIdentifier {
                index: Some(252),
                hash: None,
            },
        };

        let server_options = Config {
            node: "https://api.hornet-rosetta.testnet.chrysalis2.com".to_string(),
            network: "testnet7".to_string(),
            tx_indexation: "rosetta".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: RosettaMode::Online,
            bind_addr: "0.0.0.0:3030".to_string(),
        };
        let response = block(request, server_options).await.unwrap();
        assert_eq!(252, response.block.block_identifier.index);
        assert_eq!(
            "81cb76571142610b3773f247dc6d915673d4b7e4f73bad2ab82d94a10e3ed5bd",
            response.block.block_identifier.hash
        );
        assert_eq!(251, response.block.parent_block_identifier.index);
        assert_eq!(
            "beec71a316e761cabc66e82ff67d451e49f2941f160c38b4296af9de05bba638",
            response.block.parent_block_identifier.hash
        );
        assert_eq!(1617799822 * 1000, response.block.timestamp);
        assert_eq!(false, response.block.metadata.is_some());
    }
}
