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
    iota_client: &Client,
    rosetta_config: &RosettaConfig,
) -> Result<Vec<BlockTransaction>, ApiError> {
    let messages = messages_of_created_outputs(milestone_index, iota_client).await?;

    let mut transactions = Vec::new();

    for (_message_id, message_info) in messages {
        let transaction = match message_info.message.payload() {
            Some(Payload::Transaction(t)) => from_transaction(t, iota_client, rosetta_config).await?,
            Some(Payload::Milestone(_)) => {
                from_milestone(&message_info.created_outputs, iota_client, rosetta_config).await?
            }
            _ => return Err(ApiError::NonRetriable("unknown payload type in message".to_string())),
        };
        transactions.push(transaction);
    }

    Ok(transactions)
}

struct MessageInfo {
    pub message: Message,
    pub created_outputs: Vec<OutputId>,
}

async fn messages_of_created_outputs(
    milestone_index: u32,
    iota_client: &Client,
) -> Result<HashMap<MessageId, MessageInfo>, ApiError> {
    let mut message_map = HashMap::new();

    let created_outputs = get_utxo_changes(milestone_index, iota_client).await?.created_outputs;

    for output_id_string in created_outputs {
        let output_id = output_id_string
            .parse::<OutputId>()
            .map_err(|e| ApiError::NonRetriable(format!("can not parse output id: {}", e)))?;

        let message_id = get_output(output_id, iota_client)
            .await?
            .message_id
            .parse::<MessageId>()
            .map_err(|e| ApiError::NonRetriable(format!("can not parse message id: {}", e)))?;

        match message_map.entry(message_id) {
            Entry::Occupied(mut entry) => {
                let message_info: &mut MessageInfo = entry.get_mut();
                message_info.created_outputs.push(output_id);
            }
            Entry::Vacant(entry) => {
                let message = iota_client
                    .get_message()
                    .data(&message_id)
                    .await
                    .map_err(|e| ApiError::NonRetriable(format!("can not get message: {}", e)))?;
                let message_info = MessageInfo {
                    message,
                    created_outputs: vec![output_id],
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
            _ => return Err(ApiError::NonRetriable("unknown UTXO type".to_string())),
        };

        let output = Output::try_from(
            &iota_client
                .get_output(utxo_input)
                .await
                .map_err(|e| ApiError::NonRetriable(format!("can not get output: {}", e)))?
                .output,
        )
        .map_err(|e| ApiError::NonRetriable(format!("can not deserialize output: {}", e)))?;

        operations.push(build_utxo_input_operation(
            utxo_input.output_id(),
            &output,
            operations.len(),
            true,
            rosetta_config,
        )?);
    }

    for (output_index, output) in regular_essence.outputs().iter().enumerate() {
        let output_id = OutputId::new(transaction_payload.id(), output_index as u16)
            .map_err(|e| ApiError::NonRetriable(format!("can not parse output id: {}", e)))?;

        let output_operation = match output {
            Output::SignatureLockedSingle(sig_locked_single_output) => build_sig_locked_single_output_operation(
                Some(output_id),
                sig_locked_single_output,
                operations.len(),
                false,
                rosetta_config,
            )?,
            Output::SignatureLockedDustAllowance(sig_locked_dust_allowance_output) => {
                build_dust_allowance_output_operation(
                    Some(output_id),
                    sig_locked_dust_allowance_output,
                    operations.len(),
                    false,
                    rosetta_config,
                )?
            }
            _ => return Err(ApiError::NonRetriable("unknown output type".to_string())),
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
    created_outputs: &[OutputId],
    iota_client: &Client,
    rosetta_config: &RosettaConfig,
) -> Result<BlockTransaction, ApiError> {
    let mut operations = Vec::new();

    for output_id in created_outputs {
        let output = Output::try_from(
            &iota_client
                .get_output(&UtxoInput::from(*output_id))
                .await
                .map_err(|e| ApiError::NonRetriable(format!("can not get output: {}", e)))?
                .output,
        )
        .map_err(|e| ApiError::NonRetriable(format!("can not deserialize output: {}", e)))?;

        let mint_operation = match output {
            Output::SignatureLockedSingle(sig_locked_single_output) => build_sig_locked_single_output_operation(
                Some(*output_id),
                &sig_locked_single_output,
                operations.len(),
                true,
                rosetta_config,
            ),
            Output::SignatureLockedDustAllowance(sig_locked_dust_allowance_output) => {
                build_dust_allowance_output_operation(
                    Some(*output_id),
                    &sig_locked_dust_allowance_output,
                    operations.len(),
                    true,
                    rosetta_config,
                )
            }
            _ => return Err(ApiError::NonRetriable("unknown output type".to_string())),
        }?;

        operations.push(mint_operation);
    }

    let transaction = BlockTransaction {
        transaction_identifier: TransactionIdentifier {
            hash: created_outputs.first().unwrap().transaction_id().to_string(),
        },
        operations,
    };

    Ok(transaction)
}
