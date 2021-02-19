// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    currency::iota_currency,
    types::{AccountIdentifier, Amount, Operation, OperationIdentifier, OperationStatus},
};

// operation types
pub const UTXO_CONSUMED: &str = "UTXO_CONSUMED";
pub const UTXO_CREATED: &str = "UTXO_CREATED";

// operation status
pub const UTXO_SPENT: &str = "UTXO_SPENT";
pub const UTXO_UNSPENT: &str = "UTXO_UNSPENT";

pub fn operation_type_list() -> Vec<String> {
    let mut ret = vec![];
    ret.push(UTXO_CONSUMED.into());
    ret.push(UTXO_CREATED.into());
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

pub fn consumed_utxo_operation(is_spent: bool, address: String, amnt: u64, output_index: u16, operation_counter: u32, n_operations: u32) -> Operation {
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

    Operation {
        operation_identifier: OperationIdentifier {
            index: operation_counter as u64,
            network_index: Some(output_index as u64), // no sharding in IOTA yet :(
        },
        related_operations: Some(related_operations),
        type_: UTXO_CONSUMED.into(),
        status: Some(status.into()),
        account: Some(account),
        amount: Some(amount),
    }
}

pub fn created_utxo_operation(is_spent: bool, address: String, amnt: u64, output_index: u16, operation_counter: u32, n_operations: u32) -> Operation {
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

    Operation {
        operation_identifier: OperationIdentifier {
            index: operation_counter as u64,
            network_index: Some(output_index as u64), // no sharding in IOTA yet :(
        },
        related_operations: Some(related_operations),
        type_: UTXO_CREATED.into(),
        status: Some(status.into()),
        account: Some(account),
        amount: Some(amount),
    }
}