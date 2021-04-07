// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    construction::serialize_unsigned_transaction, error::ApiError, is_bad_network, require_offline_mode, types::*,
    Options,
};

use bee_common::packable::Packable;
use bee_message::prelude::*;

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionPayloadsRequest {
    pub network_identifier: NetworkIdentifier,
    pub operations: Vec<Operation>,
    pub metadata: ConstructionMetadata,
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
        let address = operation
            .account
            .ok_or(ApiError::BadConstructionRequest("account not populated".to_string()))?
            .address;

        match &operation.type_[..] {
            "UTXO_INPUT" => {
                let output_id = operation
                    .coin_change
                    .ok_or(ApiError::BadConstructionRequest(
                        "coin_change not populated for UTXO_INPUT".to_string(),
                    ))?
                    .coin_identifier
                    .identifier;
                let utxo_input = output_id
                    .parse::<UtxoInput>()
                    .map_err(|e| ApiError::BadConstructionRequest(e.to_string()))?;
                let input = Input::Utxo(utxo_input);

                inputs.push((input, address));
            }
            "UTXO_OUTPUT" => {
                let address = Address::try_from_bech32(&address).unwrap();
                let amount = operation
                    .amount
                    .ok_or(ApiError::BadConstructionRequest("amount not populated".to_string()))?
                    .value
                    .parse::<u64>()
                    .unwrap();
                // todo: tread Dust allowance
                let output: Output = SignatureLockedSingleOutput::new(address, amount).unwrap().into();
                outputs.push(output);
            }
            _ => return Err(ApiError::UnknownOperationType),
        }
    }

    let index = options.indexation;
    let indexation_payload = IndexationPayload::new(index.as_bytes(), &[])?;

    let mut transaction_payload_essence =
        RegularEssenceBuilder::new().with_payload(Payload::Indexation(Box::new(indexation_payload)));

    // sort inputs and outputs
    inputs.sort_unstable_by_key(|i| i.0.pack_new());
    outputs.sort_unstable_by_key(|o| o.pack_new());

    for (i, _) in inputs.clone() {
        transaction_payload_essence = transaction_payload_essence.add_input(i);
    }

    for o in outputs {
        transaction_payload_essence = transaction_payload_essence.add_output(o);
    }

    let essence = Essence::Regular(transaction_payload_essence.finish().unwrap());
    let hash_to_sign = essence.hash();
    let unsigned_transaction =
        UnsignedTransaction::new(essence, construction_payloads_request.metadata.inputs_metadata);

    for (_, address) in inputs {
        signing_payloads.push(SigningPayload {
            account_identifier: Some(AccountIdentifier {
                address,
                sub_account: None,
            }),
            hex_bytes: hex::encode(&hash_to_sign),
            signature_type: Some(SignatureType::Edwards25519),
        });
    }

    Ok(ConstructionPayloadsResponse {
        unsigned_transaction: serialize_unsigned_transaction(&unsigned_transaction),
        payloads: signing_payloads,
    })
}
