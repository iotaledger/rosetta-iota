// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    currency::iota_currency,
    types::{AccountIdentifier, Amount, Operation, OperationIdentifier, OperationStatus},
};
use crate::types::{CoinChange, CoinIdentifier};

// operation types
pub const UTXO: &str = "UTXO";

// operation status
pub const UTXO_SPENT: &str = "UTXO_SPENT"; //  has already been spent
pub const UTXO_UNSPENT: &str = "UTXO_UNSPENT"; // UTXO has not yet been spent

// operation coin actions
pub const UTXO_CONSUMED: &str = "UTXO_CONSUMED"; // UTXO Input, where coins are coming from into the Transaction
pub const UTXO_CREATED: &str = "UTXO_CREATED"; // UTXO Output, where coins are going out from the Transaction

pub fn operation_type_list() -> Vec<String> {
    let mut ret = vec![];
    ret.push(UTXO.into());
    ret
}

pub fn operation_status_spent() -> OperationStatus {
    OperationStatus {
        status: UTXO_SPENT.into(),
        successful: true,
    }
}

pub fn operation_status_unspent() -> OperationStatus {
    OperationStatus {
        status: UTXO_UNSPENT.into(),
        successful: false,
    }
}

pub fn utxo_operation(transaction_id: String, address: String, amnt: u64, output_index: u16, operation_counter: u32, n_operations: u32, consumed: &bool, is_spent: bool) -> Operation {
    let status = match is_spent {
        true => UTXO_SPENT,
        false => UTXO_UNSPENT,
    };

    let account = AccountIdentifier {
        address,
        sub_account: None,
    };
    let amount = Amount {
        value: amnt.to_string(),
        currency: iota_currency(),
    };

    let mut related_operations = vec![];
    for i in 0..n_operations {
        if i != operation_counter {
            related_operations.push( OperationIdentifier {
                index: i as u64,
                network_index: None
            });
        }
    }

    let output_id = format!("{}{}", transaction_id, hex::encode(output_index.to_le_bytes()));

    Operation {
        operation_identifier: OperationIdentifier {
            index: operation_counter as u64,
            network_index: Some(output_index as u64), // no sharding in IOTA yet :(
        },
        related_operations: Some(related_operations),
        type_: UTXO.into(),
        status: Some(status.into()),
        account: Some(account),
        amount: Some(amount),
        coin_change: CoinChange {
            coin_identifier: CoinIdentifier {
                identifier: output_id
            },
            coin_action: match consumed {
                true => UTXO_CONSUMED.into(),
                false => UTXO_CREATED.into(),
            }
        }
    }
}