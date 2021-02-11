// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    currency::iota_currency,
    types::{AccountIdentifier, Amount, Operation, OperationIdentifier, OperationStatus},
};

pub enum OperationIndex {
    UTXOConsumed,
    UTXOCreated,
}

pub fn operation_type_list() -> Vec<String> {
    let mut ret = vec![];
    ret.push(String::from("UTXO_CONSUMED"));
    ret.push(String::from("UTXO_CREATED"));
    ret
}

pub const CONSUMED_UTXO_OPERATION_IDENTIFIER: OperationIdentifier = OperationIdentifier {
    index: OperationIndex::UTXOConsumed as u64,
    network_index: None, // no sharding in IOTA yet :(
};

pub const CREATED_UTXO_OPERATION_IDENTIFIER: OperationIdentifier = OperationIdentifier {
    index: OperationIndex::UTXOCreated as u64,
    network_index: None, // no sharding in IOTA yet :(
};

pub fn operation_status_spent() -> OperationStatus {
    OperationStatus {
        status: String::from("SPENT"),
        successful: true,
    }
}

pub fn operation_status_unspent() -> OperationStatus {
    OperationStatus {
        status: String::from("UNSPENT"),
        successful: false,
    }
}

pub fn consumed_utxo_operation(is_spent: bool, address: String, amnt: u64) -> Operation {
    let related_operations = vec![CREATED_UTXO_OPERATION_IDENTIFIER];
    let status = match is_spent {
        true => String::from("spent"),
        false => String::from("unspent"),
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
        operation_identifier: CONSUMED_UTXO_OPERATION_IDENTIFIER,
        related_operations: Some(related_operations),
        type_: String::from("consumed UXTO"),
        status: Some(status),
        account: Some(account),
        amount: Some(amount),
    }
}

pub fn created_utxo_operation(is_spent: bool, address: String, amnt: u64) -> Operation {
    let related_operations = vec![CONSUMED_UTXO_OPERATION_IDENTIFIER];
    let status = match is_spent {
        true => String::from("spent"),
        false => String::from("unspent"),
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
        operation_identifier: CREATED_UTXO_OPERATION_IDENTIFIER,
        related_operations: Some(related_operations),
        type_: String::from("created UXTO"),
        status: Some(status),
        account: Some(account),
        amount: Some(amount),
    }
}
