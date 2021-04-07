// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    currency::iota_currency,
    types::{AccountIdentifier, Amount, CoinAction, CoinChange, CoinIdentifier, Operation, OperationIdentifier},
};

// operation types
pub const UTXO_INPUT: &str = "UTXO_INPUT";
pub const UTXO_OUTPUT: &str = "UTXO_OUTPUT";

// operation status
pub const SUCCESS: &str = "Success";
pub const SKIPPED: &str = "Skipped";

// operation coin actions
pub const UTXO_CONSUMED: &str = "coin_spent"; // UTXO Input, where coins are coming from into the Transaction
pub const UTXO_CREATED: &str = "coin_created"; // UTXO Output, where coins are going out from the Transaction

pub fn operation_type_list() -> Vec<String> {
    let mut ret = vec![];
    ret.push(UTXO_INPUT.into());
    ret.push(UTXO_OUTPUT.into());
    ret
}

pub fn operation_status_success() -> String {
    SUCCESS.into()
}

pub fn operation_status_skipped() -> String {
    SKIPPED.into()
}

pub fn utxo_input_operation(
    transaction_id: String,
    address: String,
    amnt: u64,
    output_index: u16,
    operation_counter: usize,
    consumed: bool,
    online: bool,
) -> Operation {
    let account = AccountIdentifier {
        address,
        sub_account: None,
    };

    let amount = Amount {
        value: match consumed {
            true => (amnt as i64 * -1).to_string(),
            false => amnt.to_string(),
        },
        currency: iota_currency(),
        metadata: None,
    };

    let output_id = format!("{}{}", transaction_id, hex::encode(output_index.to_le_bytes()));

    Operation {
        operation_identifier: OperationIdentifier {
            index: operation_counter as u64,
            network_index: Some(output_index as u64), // no sharding in IOTA yet :(
        },
        related_operations: None,
        type_: UTXO_INPUT.into(),
        status: match online {
            true => Some(SUCCESS.into()), // call coming from /data/block
            false => None,                // call coming from /construction/parse
        },
        account: Some(account),
        amount: Some(amount),
        coin_change: Some(CoinChange {
            coin_identifier: CoinIdentifier { identifier: output_id },
            coin_action: match consumed {
                true => CoinAction::CoinSpent,
                false => CoinAction::CoinCreated,
            },
        }),
        metadata: None,
    }
}

pub fn utxo_output_operation(address: String, amnt: u64, operation_counter: usize, online: bool) -> Operation {
    let account = AccountIdentifier {
        address,
        sub_account: None,
    };

    let amount = Amount {
        value: amnt.to_string(),
        currency: iota_currency(),
        metadata: None,
    };

    Operation {
        operation_identifier: OperationIdentifier {
            index: operation_counter as u64,
            network_index: None,
        },
        related_operations: None,
        type_: UTXO_OUTPUT.into(),
        status: match online {
            true => Some(SUCCESS.into()),
            false => None,
        },
        account: Some(account),
        amount: Some(amount),
        coin_change: None,
        metadata: None,
    }
}
