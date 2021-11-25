// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{build_client, get_milestone, get_output, get_utxo_changes},
    config::RosettaConfig,
    error::ApiError,
    is_offline_mode_enabled, is_wrong_network,
    operations::*,
    types::{
        Block, BlockIdentifier, BlockTransaction, NetworkIdentifier, PartialBlockIdentifier, TransactionIdentifier,
    },
};

use bee_message::{
    payload::transaction::Essence,
    prelude::{Output, *},
    Message,
};
use bee_rest_api::types::responses::OutputResponse;

use iota_client::Client;

use log::debug;
use serde::{Deserialize, Serialize};

use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryFrom,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockRequest {
    pub network_identifier: NetworkIdentifier,
    pub block_identifier: PartialBlockIdentifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockResponse {
    pub block: Block,
}

pub async fn block(request: BlockRequest, rosetta_config: RosettaConfig) -> Result<BlockResponse, ApiError> {
    debug!("/block");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()));
    }

    if is_offline_mode_enabled(&rosetta_config) {
        return Err(ApiError::NonRetriable(
            "endpoint does not support offline mode".to_string(),
        ));
    }

    let milestone_index = match (request.block_identifier.index, request.block_identifier.hash) {
        (Some(index), Some(hash)) => {
            let hash = hash.parse::<u32>().map_err(|_| {
                ApiError::NonRetriable("invalid block hash: can not parse milestone index from string".to_string())
            })?;
            if index != hash {
                return Err(ApiError::NonRetriable(
                    "block index does not relate to block hash".to_string(),
                ));
            } else {
                index
            }
        }
        (Some(index), None) => index,
        (None, Some(hash)) => hash.parse::<u32>().map_err(|_| {
            ApiError::NonRetriable("invalid block hash: can not parse milestone index from string".to_string())
        })?,
        (None, None) => {
            return Err(ApiError::NonRetriable(
                "either block index or block hash must be set".to_string(),
            ));
        }
    };

    let client = build_client(&rosetta_config).await?;

    let block = Block {
        block_identifier: BlockIdentifier {
            index: milestone_index,
            hash: milestone_index.to_string(),
        },
        parent_block_identifier: BlockIdentifier {
            index: milestone_index - 1,
            hash: (milestone_index - 1).to_string(),
        },
        timestamp: get_milestone(milestone_index, &client).await?.timestamp * 1000,
        transactions: build_block_transactions(milestone_index, &client, &rosetta_config).await?,
    };

    Ok(BlockResponse { block })
}

async fn build_block_transactions(
    milestone_index: u32,
    client: &Client,
    rosetta_config: &RosettaConfig,
) -> Result<Vec<BlockTransaction>, ApiError> {
    let messages = messages_with_utxo_changes(milestone_index, client).await?;

    let mut transactions = Vec::new();

    for (_message_id, message_info) in messages {
        let transaction = match message_info.message.payload() {
            Some(Payload::Transaction(t)) => from_transaction(t, client, rosetta_config).await?,
            Some(Payload::Milestone(_)) => from_milestone(&message_info.created_outputs, rosetta_config).await?,
            _ => return Err(ApiError::NonRetriable("payload type not supported".to_string())),
        };
        transactions.push(transaction);
    }

    Ok(transactions)
}

struct MessageInfo {
    pub message: Message,
    pub created_outputs: Vec<CreatedOutput>,
}

struct CreatedOutput {
    pub output_id: OutputId,
    pub output_response: OutputResponse,
}

async fn messages_with_utxo_changes(
    milestone_index: u32,
    iota_client: &Client,
) -> Result<HashMap<MessageId, MessageInfo>, ApiError> {
    let mut message_map = HashMap::new();

    let created_outputs = get_utxo_changes(milestone_index, iota_client).await?.created_outputs;

    for output_id_string in created_outputs {
        let output_id = output_id_string
            .parse::<OutputId>()
            .map_err(|e| ApiError::NonRetriable(format!("can not parse output id: {}", e)))?;

        let output_response = get_output(output_id, iota_client).await?;

        let message_id = output_response
            .message_id
            .parse::<MessageId>()
            .map_err(|e| ApiError::NonRetriable(format!("can not parse message id: {}", e)))?;

        let created_output = CreatedOutput {
            output_id,
            output_response,
        };

        match message_map.entry(message_id) {
            Entry::Occupied(mut entry) => {
                let message_info: &mut MessageInfo = entry.get_mut();
                message_info.created_outputs.push(created_output);
            }
            Entry::Vacant(entry) => {
                let message = iota_client
                    .get_message()
                    .data(&message_id)
                    .await
                    .map_err(|e| ApiError::NonRetriable(format!("can not get message: {}", e)))?;
                let message_info = MessageInfo {
                    message,
                    created_outputs: vec![created_output],
                };
                entry.insert(message_info);
            }
        }
    }

    Ok(message_map)
}

async fn from_transaction(
    transaction_payload: &TransactionPayload,
    iota_client: &Client,
    rosetta_config: &RosettaConfig,
) -> Result<BlockTransaction, ApiError> {
    let Essence::Regular(regular_essence) = transaction_payload.essence();

    let mut operations = Vec::new();

    for input in regular_essence.inputs() {
        let utxo_input = match input {
            Input::Utxo(i) => i,
            _ => return Err(ApiError::NonRetriable("input type not supported".to_string())),
        };

        let output_info = iota_client
            .get_output(utxo_input)
            .await
            .map_err(|e| ApiError::NonRetriable(format!("can not get input information: {}", e)))?;

        let output = Output::try_from(&output_info.output)
            .map_err(|e| ApiError::NonRetriable(format!("can not parse output from output information: {}", e)))?;

        let (amount, ed25519_address) = address_and_balance_of_output(&output).await?;

        operations.push(utxo_input_operation(
            output_info.transaction_id,
            Address::Ed25519(ed25519_address).to_bech32(&rosetta_config.bech32_hrp),
            amount,
            output_info.output_index,
            operations.len(),
            true,
            true,
        ));
    }

    for (output_index, output) in regular_essence.outputs().iter().enumerate() {
        let output_id = OutputId::new(transaction_payload.id(), output_index as u16)
            .map_err(|e| ApiError::NonRetriable(format!("can not parse output id: {}", e)))?;

        let output_operation = match output {
            Output::SignatureLockedSingle(o) => match o.address() {
                Address::Ed25519(addr) => {
                    let bech32_address = Address::Ed25519(*addr).to_bech32(&rosetta_config.bech32_hrp);
                    utxo_output_operation(bech32_address, o.amount(), operations.len(), true, Some(output_id))
                }
            },
            Output::SignatureLockedDustAllowance(o) => match o.address() {
                Address::Ed25519(addr) => {
                    let bech32_address = Address::Ed25519(*addr).to_bech32(&rosetta_config.bech32_hrp);
                    dust_allowance_output_operation(bech32_address, o.amount(), operations.len(), true, Some(output_id))
                }
            },
            _ => unimplemented!(),
        };

        operations.push(output_operation);
    }

    let transaction = BlockTransaction {
        transaction_identifier: TransactionIdentifier {
            hash: transaction_payload.id().to_string(),
        },
        operations,
    };

    Ok(transaction)
}

async fn from_milestone(
    created_outputs: &[CreatedOutput],
    options: &RosettaConfig,
) -> Result<BlockTransaction, ApiError> {
    let mut operations = Vec::new();

    for created_output in created_outputs {
        let output = Output::try_from(&created_output.output_response.output)
            .map_err(|_| ApiError::NonRetriable("can not deserialize output".to_string()))?;

        let (amount, ed25519_address) = address_and_balance_of_output(&output).await?;

        let mint_operation = utxo_output_operation(
            Address::Ed25519(ed25519_address).to_bech32(&options.bech32_hrp),
            amount,
            operations.len(),
            true,
            Some(created_output.output_id),
        );

        operations.push(mint_operation);
    }

    let transaction = BlockTransaction {
        transaction_identifier: TransactionIdentifier {
            hash: created_outputs.first().unwrap().output_id.transaction_id().to_string(),
        },
        operations,
    };

    Ok(transaction)
}

async fn address_and_balance_of_output(output: &Output) -> Result<(u64, Ed25519Address), ApiError> {
    let (amount, ed25519_address) = match output {
        Output::SignatureLockedSingle(r) => match r.address() {
            Address::Ed25519(addr) => (r.amount(), *addr),
        },
        Output::SignatureLockedDustAllowance(r) => match r.address() {
            Address::Ed25519(addr) => (r.amount(), *addr),
        },
        _ => return Err(ApiError::NonRetriable("output type not supported".to_string())),
    };
    Ok((amount, ed25519_address))
}
