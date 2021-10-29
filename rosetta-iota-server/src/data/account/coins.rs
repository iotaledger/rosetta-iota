// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::RosettaConfig,
    consts::iota_currency,
    error::ApiError,
    is_offline_mode_enabled, is_wrong_network,
    types::{AccountIdentifier, NetworkIdentifier, *},
};
use crate::client::{build_client, get_unspent_outputs_of_address, get_output};

use bee_message::milestone::MilestoneIndex;
use bee_message::output::OutputId;
use bee_rest_api::types::dtos::{AddressDto, OutputDto};
use bee_rest_api::types::responses::{OutputResponse, OutputsAddressResponse};

use iota_client::Client;

use log::debug;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccountCoinsRequest {
    pub network_identifier: NetworkIdentifier,
    pub account_identifier: AccountIdentifier,
    pub include_mempool: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccountCoinsResponse {
    pub block_identifier: BlockIdentifier,
    pub coins: Vec<Coin>,
}

pub async fn account_coins(request: AccountCoinsRequest, rosetta_config: RosettaConfig) -> Result<AccountCoinsResponse, ApiError> {
    debug!("/account/coins");

    if is_wrong_network(&rosetta_config, &request.network_identifier) {
        return Err(ApiError::NonRetriable("request was made for wrong network".to_string()));
    }

    if is_offline_mode_enabled(&rosetta_config) {
        return Err(ApiError::NonRetriable(
            "endpoint is not available in offline mode".to_string(),
        ));
    }

    if request.include_mempool {
        return Err(ApiError::NonRetriable(
            "mempool coins are not supported".to_string(),
        ));
    }

    let (outputs, ledger_index) = address_outputs_with_ledger_index(&request.account_identifier.address, &rosetta_config).await?;

    let mut coins = Vec::new();
    for (output_id, output_response) in outputs {
        let amount = match output_response.output {
            OutputDto::SignatureLockedSingle(r) => match r.address {
                AddressDto::Ed25519(_) => r.amount,
            },
            OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                AddressDto::Ed25519(_) => r.amount,
            },
            _ => unimplemented!()
        };

        coins.push(Coin {
            coin_identifier: CoinIdentifier { identifier: output_id.to_string() },
            amount: Amount {
                value: amount.to_string(),
                currency: iota_currency(),
            },
        });
    }

    Ok(AccountCoinsResponse {
        block_identifier: BlockIdentifier {
            index: *ledger_index,
            hash: (*ledger_index).to_string(),
        },
        coins,
    })
}

async fn address_outputs_with_ledger_index(
    address: &str,
    options: &RosettaConfig,
) -> Result<(HashMap<OutputId, OutputResponse>, MilestoneIndex), ApiError> {
    let client: Client = build_client(options).await?;

    loop {

        let outputs_address_response: OutputsAddressResponse = get_unspent_outputs_of_address(&address, &client).await?;
        let mut output_responses = HashMap::new();

        let mut try_again = false;
        for id in outputs_address_response.output_ids {
            let output_id = id
                .parse::<OutputId>()
                .map_err(|e| ApiError::NonRetriable(format!("can not parse output id: {}", e)))?;

            let output_response = get_output(output_id, &client).await?;

            // if the output was spent in the meantime, retry
            if output_response.is_spent {
                try_again = true;
                break;
            } else {
                output_responses.insert(output_id, output_response);
            }
        }

        if try_again {
            continue
        } else {
            break Ok((output_responses, MilestoneIndex(outputs_address_response.ledger_index)))
        }
    }

}
