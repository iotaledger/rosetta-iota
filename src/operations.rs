// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    currency::iota_currency,
    types::{AccountIdentifier, Amount, Operation, OperationIdentifier, OperationStatus},
};
use crate::types::{CoinChange, CoinIdentifier, OperationMetadata};

// operation types
pub const UTXO_INPUT: &str = "UTXO_INPUT";
pub const UTXO_OUTPUT: &str = "UTXO_OUTPUT";

// operation status
pub const SUCCESS: &str = "SUCCESS";
pub const SKIPPED: &str = "SKIPPED";

// operation coin actions
pub const UTXO_CONSUMED: &str = "coin_spent"; // UTXO Input, where coins are coming from into the Transaction
pub const UTXO_CREATED: &str = "coin_created"; // UTXO Output, where coins are going out from the Transaction

// operation metadata
pub const UTXO_SPENT: &str = "UTXO_SPENT"; // UTXO has already been spent (possibly by another Transaction)
pub const UTXO_UNSPENT: &str = "UTXO_UNSPENT"; // UTXO has not yet been spent

pub fn operation_type_list() -> Vec<String> {
    let mut ret = vec![];
    ret.push(UTXO_INPUT.into());
    ret.push(UTXO_OUTPUT.into());
    ret
}

pub fn operation_status_success() -> OperationStatus {
    OperationStatus {
        status: SUCCESS.into(),
        successful: true,
    }
}

pub fn operation_status_skipped() -> OperationStatus {
    OperationStatus {
        status: SKIPPED.into(),
        successful: false,
    }
}

pub fn utxo_operation(transaction_id: String, address: String, amnt: u64, output_index: u16, operation_counter: u32, n_operations: u32, consumed: &bool, is_spent: bool) -> Operation {
    let account = AccountIdentifier {
        address,
        sub_account: None,
    };
    let amount = Amount {
        value: amnt.to_string(),
        currency: iota_currency(),
    };

    let output_id = format!("{}{}", transaction_id, hex::encode(output_index.to_le_bytes()));

    Operation {
        operation_identifier: OperationIdentifier {
            index: operation_counter as u64,
            network_index: Some(output_index as u64), // no sharding in IOTA yet :(
        },
        related_operations: None,
        type_: match consumed {
            true => UTXO_INPUT.into(),
            false => UTXO_OUTPUT.into(),
        },
        status: Some(SUCCESS.into()),
        account: Some(account),
        amount: Some(amount),
        coin_change: CoinChange {
            coin_identifier: CoinIdentifier {
                identifier: output_id
            },
            coin_action: match consumed {
                true => UTXO_CONSUMED.into(),
                false => UTXO_CREATED.into(),
            },
        },
        metadata: Some(OperationMetadata {
            is_spent: match is_spent {
                true => UTXO_SPENT.into(),
                false => UTXO_UNSPENT.into()
            }
        })
    }
}