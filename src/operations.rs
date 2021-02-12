// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    currency::iota_currency,
    types::{AccountIdentifier, Amount, Operation, OperationIdentifier, OperationStatus},
};

// operation types
pub const CONSUMED_UTXO: &str = "CONSUMED_UTXO";

// operation status
pub const SPENT: &str = "SPENT";
pub const UNSPENT: &str = "UNSPENT";

pub fn operation_type_list() -> Vec<String> {
    let mut ret = vec![];
    ret.push(CONSUMED_UTXO.into());
    ret
}

pub fn operation_status_spent() -> OperationStatus {
    OperationStatus {
        status: SPENT.into(),
        successful: true,
    }
}

pub fn operation_status_unspent() -> OperationStatus {
    OperationStatus {
        status: UNSPENT.into(),
        successful: false,
    }
}

pub fn consumed_utxo_operation(is_spent: bool, address: String, amnt: u64, output_index: u16, operation_counter: u32) -> Operation {
    //let related_operations = vec![CREATED_UTXO_OPERATION_IDENTIFIER]; // todo
    let status = match is_spent {
        true => SPENT,
        false => UNSPENT,
    };
    let account = AccountIdentifier {
        address,
        sub_account: None,
    };
    let amount = Amount {
        value: amnt.to_string(),
        currency: iota_currency(),
    };

    Operation {
        operation_identifier: OperationIdentifier {
            index: operation_counter as u64,
            network_index: Some(output_index as u64), // no sharding in IOTA yet :(
        },
        related_operations: None, // todo
        type_: CONSUMED_UTXO.into(),
        status: Some(status.into()),
        account: Some(account),
        amount: Some(amount),
    }
}