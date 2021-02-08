use iota::Client;
use crate::{
    consts,
    error::ApiError,
    filters::{handle, with_options},
    options::Options,
    types::{
        AccountIdentifier, Amount, Block, BlockIdentifier, BlockRequest, BlockResponse, Currency,
        Operation, OperationIdentifier, Transaction, TransactionIdentifier,
    },
};
use log::debug;
use warp::Filter;

pub fn routes(
    options: Options,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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
    if network_identifier.blockchain != consts::BLOCKCHAIN
        || network_identifier.network != options.network
    {
        return Err(ApiError::BadNetwork);
    }

    let iota_client = match iota::Client::builder()
        .with_network(&options.network)
        .with_node(&options.iota_endpoint)
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await {
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

    let block_request_hash = block_request.block_identifier.hash.unwrap();
    if (block_request_hash != "") && (block_request_hash != milestone.message_id.to_string()) {
        return Err(ApiError::BadMilestoneRequest);
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

    for output_id in utxo_changes.created_outputs {
        // todo: uncomment below as soon as get_milestone_utxo_changes drops Vec<String> for outputs
        // input = UTXOInput::from_str(output_id);

        let transaction_identifier = TransactionIdentifier {
            hash: output_id
        };

        let operations  = vec![];
        // operations.push(Operation {
        //     // here we might want to consume a constant representation of Created Outputs as OperationIdentifier
        //     operation_identifier: OperationIdentifier {
        //         index: 0,
        //         network_index: 0,
        //     },
        //     related_operations: Option<Vec<OperationIdentifier>>,
        //     type_: String::from(""),
        //     status: Option<String>,
        //     account: Option<AccountIdentifier>,
        //     amount: Option<Amount>,
        // });

        transactions.push(Transaction {
            transaction_identifier: transaction_identifier,
            operations: operations
        });
    }

    for output_id in utxo_changes.consumed_outputs {
        // todo: uncomment below as soon as get_milestone_utxo_changes drops Vec<String> for outputs
        // output = UTXOOutput:from_str(output_id);

        let transaction_identifier = TransactionIdentifier {
            hash: output_id
        };

        let operations  = vec![];

        transactions.push(Transaction {
            transaction_identifier: transaction_identifier,
            operations: operations
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