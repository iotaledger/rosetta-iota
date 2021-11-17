// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{construction::serialize_unsigned_transaction, error::ApiError, is_wrong_network, types::*, RosettaConfig};

use bee_common::packable::Packable;
use bee_message::prelude::*;

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionPayloadsRequest {
    pub network_identifier: NetworkIdentifier,
    pub operations: Vec<Operation>,
    pub metadata: ConstructionMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionPayloadsResponse {
    pub unsigned_transaction: String,
    pub payloads: Vec<SigningPayload>,
}

pub async fn payloads(
    request: ConstructionPayloadsRequest,
    rosetta_config: RosettaConfig,
) -> Result<ConstructionPayloadsResponse, ApiError> {
    debug!("/construction/payloads");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()));
    }

    let mut inputs = vec![];
    let mut outputs = vec![];
    let mut signing_payloads = vec![];

    for operation in request.operations {
        let address = operation
            .account
            .ok_or(ApiError::NonRetriable("account not populated".to_string()))?
            .address;

        match &operation.type_[..] {
            "INPUT" => {
                let output_id = operation
                    .coin_change
                    .ok_or(ApiError::NonRetriable("coin change not populated".to_string()))?
                    .coin_identifier
                    .identifier;

                let utxo_input = output_id
                    .parse::<UtxoInput>()
                    .map_err(|e| ApiError::NonRetriable(e.to_string()))?;

                inputs.push((Input::Utxo(utxo_input), address));
            }

            "SIG_LOCKED_SINGLE_OUTPUT" => {
                let address = Address::try_from_bech32(&address).unwrap();

                let amount = operation
                    .amount
                    .ok_or(ApiError::NonRetriable("amount not populated".to_string()))?
                    .value
                    .parse::<u64>()
                    .unwrap();

                outputs.push(Output::SignatureLockedSingle(
                    SignatureLockedSingleOutput::new(address, amount).unwrap().into(),
                ));
            }

            "SIG_LOCKED_DUST_ALLOWANCE_OUTPUT" => {
                let address = Address::try_from_bech32(&address).unwrap();

                let amount = operation
                    .amount
                    .ok_or(ApiError::NonRetriable("amount not populated".to_string()))?
                    .value
                    .parse::<u64>()
                    .unwrap();

                outputs.push(Output::SignatureLockedDustAllowance(
                    SignatureLockedDustAllowanceOutput::new(address, amount).unwrap().into(),
                ));
            }

            _ => return Err(ApiError::NonRetriable("invalid operation type".to_string())),
        }
    }

    // sort inputs and outputs
    inputs.sort_unstable_by_key(|i| i.0.pack_new());
    outputs.sort_unstable_by_key(|o| o.pack_new());

    let mut transaction_payload_essence = RegularEssenceBuilder::new();

    for (i, _) in inputs.clone() {
        transaction_payload_essence = transaction_payload_essence.add_input(i);
    }

    for o in outputs {
        transaction_payload_essence = transaction_payload_essence.add_output(o);
    }

    let essence = Essence::Regular(transaction_payload_essence.finish().unwrap());
    let hash_to_sign = essence.hash();
    let unsigned_transaction = UnsignedTransaction::new(essence, request.metadata.utxo_inputs_metadata);

    for (_, address) in inputs {
        signing_payloads.push(SigningPayload {
            address: None,
            account_identifier: AccountIdentifier {
                address,
            },
            hex_bytes: hex::encode(&hash_to_sign),
            signature_type: SignatureType::Edwards25519,
        });
    }

    Ok(ConstructionPayloadsResponse {
        unsigned_transaction: serialize_unsigned_transaction(&unsigned_transaction),
        payloads: signing_payloads,
    })
}
