// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use crate::{Options, is_bad_network};
use crate::error::ApiError;
use crate::construction::transaction_from_hex_string;

use log::debug;


pub(crate) async fn construction_hash_request(
    construction_hash_request: ConstructionHashRequest,
    options: Options,
) -> Result<ConstructionHashResponse, ApiError> {
    debug!("/construction/hash");

    is_bad_network(&options, &construction_hash_request.network_identifier)?;

    let transaction = transaction_from_hex_string(&construction_hash_request.signed_transaction)?;

    Ok(ConstructionHashResponse {
        transaction_identifier: TransactionIdentifier {
            hash: transaction.id().to_string(),
        },
    })
}