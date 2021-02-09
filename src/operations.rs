use crate::types::{
        AccountIdentifier, Amount, Operation,OperationIdentifier
};
use crate::currency::iota_currency;

pub enum UTXOOperationIndex {
    consumed,
    created,
}

const CONSUMED_UTXOOPERATION_IDENTIFIER: OperationIdentifier = OperationIdentifier {
    index: UTXOOperationIndex::consumed as u64,
    network_index: None, // no sharding in IOTA yet :(
};

const CREATED_UTXO_OPERATION_IDENTIFIER: OperationIdentifier = OperationIdentifier {
    index: UTXOOperationIndex::created as u64,
    network_index: None, // no sharding in IOTA yet :(
};

pub fn consumed_utxo_operation(is_spent: bool, address: String, amnt: u64) -> Operation {
    let related_operations = vec![CREATED_UTXO_OPERATION_IDENTIFIER];
    let status = match is_spent {
        true => String::from("spent"),
        false => String::from("unspent")
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
        operation_identifier: CONSUMED_UTXOOPERATION_IDENTIFIER,
        related_operations: Some(related_operations),
        type_: String::from("consumed UXTO"),
        status: Some(status),
        account: Some(account),
        amount: Some(amount),
    }
}

pub fn created_utxo_operation(is_spent: bool, address: String, amnt: u64) -> Operation {
    let related_operations = vec![CONSUMED_UTXOOPERATION_IDENTIFIER];
    let status = match is_spent {
        true => String::from("spent"),
        false => String::from("unspent")
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