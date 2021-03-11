// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options, is_bad_network, require_offline_mode};
use crate::error::ApiError;

use bee_common::packable::Packable;
use bee_message::prelude::*;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::operations::UTXO_SPENT;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionPayloadsRequest {
    pub network_identifier: NetworkIdentifier,
    pub operations: Vec<Operation>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionPayloadsResponse {
    pub unsigned_transaction: String,
    pub payloads: Vec<SigningPayload>,
}

pub(crate) async fn construction_payloads_request(
    construction_payloads_request: ConstructionPayloadsRequest,
    options: Options,
) -> Result<ConstructionPayloadsResponse, ApiError> {
    debug!("/construction/payloads");

    let _ = require_offline_mode(&options)?;
    is_bad_network(&options, &construction_payloads_request.network_identifier)?;

    let mut inputs = vec![];
    let mut outputs = vec![];
    let mut signing_payloads = vec![];

    for operation in construction_payloads_request.operations {

        let account = operation.account.ok_or(ApiError::BadConstructionRequest("account not populated".to_string()))?;

        match &operation.type_[..] {
            "UTXO_INPUT" => {

                let operation_metadata = operation.metadata.ok_or(ApiError::BadConstructionRequest("metadata not populated".to_string()))?;

                if operation_metadata.is_spent == UTXO_SPENT {
                    return Err(ApiError::UnableToSpend);
                }
                let output_id_str = match operation.coin_change {
                    Some(coin_change) => coin_change.coin_identifier.identifier,
                    None => panic!("no coin_change on UTXO_INPUT!")
                };
                if output_id_str.is_empty() {
                    return Err(ApiError::BadConstructionRequest("coin_change.coin_identifier.identifier is empty".to_string()));
                }
                let output_id = output_id_str.parse::<OutputId>().map_err(|e| ApiError::BadConstructionRequest(e.to_string()))?;
                let input = Input::UTXO(output_id.into());
                
                inputs.push((input, account.address));
            },
            "UTXO_OUTPUT" => {
                let address = Address::try_from_bech32(&account.address).unwrap();
                let amount = operation.amount.ok_or(ApiError::BadConstructionRequest("amount not populated".to_string()))?.value.parse::<u64>().unwrap();
                // todo: tread Dust allowance
                let output: Output = SignatureLockedSingleOutput::new(address, amount).unwrap().into();
                outputs.push(output);
            },
            _ => return Err(ApiError::UnknownOperationType)
        }
    }

    let mut transaction_payload_essence = RegularEssenceBuilder::new();

    // todo: Rosetta indexation payload?
    // builder = builder.with_payload(p);

    for (i, _) in inputs.clone() {
        transaction_payload_essence = transaction_payload_essence.add_input(i);
    }

    for o in outputs {
        transaction_payload_essence = transaction_payload_essence.add_output(o);
    }

    let transaction_payload_essence = Essence::Regular(transaction_payload_essence.finish().unwrap());
    let transaction_payload_essence_hex = hex::encode(transaction_payload_essence.pack_new());

    for (_, address) in inputs {
        signing_payloads.push(SigningPayload {
            account_identifier: Some(AccountIdentifier {
                address,
                sub_account: None
            }),
            hex_bytes: hex::encode(transaction_payload_essence.hash()),
            signature_type: Some(SignatureType::Edwards25519)
        });
    }

    Ok(ConstructionPayloadsResponse {
        unsigned_transaction: transaction_payload_essence_hex,
        payloads: signing_payloads
    })
}