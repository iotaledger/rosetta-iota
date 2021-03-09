// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options};
use crate::error::ApiError;

use bee_common::packable::Packable;
use bee_message::prelude::*;

use log::debug;
use crate::construction::{address_from_public_key, regular_essence_to_operations};
use iota::Client;

pub(crate) async fn construction_parse_request(
    construction_parse_request: ConstructionParseRequest,
    options: Options,
) -> Result<ConstructionParseResponse, ApiError> {
    debug!("/construction/parse");

    let iota_client = match iota::Client::builder()
        .with_network(&options.network)
        .with_node(&options.iota_endpoint)
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
    {
        Ok(iota_client) => iota_client,
        Err(_) => return Err(ApiError::UnableToBuildClient),
    };

    if construction_parse_request.signed {
        parse_signed_transaction(construction_parse_request, iota_client).await
    } else {
        parse_unsigned_transaction(construction_parse_request, iota_client).await
    }

}

async fn parse_unsigned_transaction(
    construction_parse_request: ConstructionParseRequest,
    client: Client
) -> Result<ConstructionParseResponse, ApiError> {
    let essence_hex_bytes = hex::decode(construction_parse_request.transaction)?;
    let essence = Essence::unpack(&mut essence_hex_bytes.as_slice()).unwrap();

    let regular_essence = match essence {
        Essence::Regular(r) => r,
        _ => return Err(ApiError::BadConstructionRequest("essence type not supported".to_string()))
    };

    let operations = regular_essence_to_operations(&regular_essence, client).await?;

    Ok(ConstructionParseResponse {
        operations,
        account_identifier_signers: None,
    })
}

async fn parse_signed_transaction(
    construction_parse_request: ConstructionParseRequest,
    client: Client
) -> Result<ConstructionParseResponse, ApiError> {
    let transaction_hex_bytes = hex::decode(construction_parse_request.transaction)?;
    let transaction: TransactionPayload = TransactionPayload::unpack(&mut transaction_hex_bytes.as_slice()).unwrap();

    let regular_essence = match transaction.essence() {
        Essence::Regular(r) => r,
        _ => return Err(ApiError::BadConstructionRequest("essence type not supported".to_string()))
    };

    // todo: treat timeout on this unrwap
    let bech32_hrp = client.get_bech32_hrp().await.unwrap();

    let operations = regular_essence_to_operations(&regular_essence, client).await?;

    let account_identifier_signers = {
        let mut accounts_identifiers = Vec::new();
        for unlock_block in transaction.unlock_blocks().into_iter() {
            if let UnlockBlock::Signature(s) = unlock_block {
                let signature = match s {
                    SignatureUnlock::Ed25519(s) => s,
                    _ => return Err(ApiError::BadConstructionRequest("signature type not supported".to_string()))
                };
                let bech32_addr = address_from_public_key(&hex::encode(signature.public_key()))?.to_bech32(&bech32_hrp);
                accounts_identifiers.push(AccountIdentifier {
                    address: bech32_addr,
                    sub_account: None
                });
            }
        }
        accounts_identifiers
    };

    Ok(ConstructionParseResponse {
        operations,
        account_identifier_signers: Some(account_identifier_signers),
    })


}