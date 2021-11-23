// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    consts::iota_currency,
    types::{AccountIdentifier, Amount, CoinAction, CoinChange, CoinIdentifier, Operation, OperationIdentifier},
};
use bee_message::prelude::OutputId;

// operation types
pub const INPUT: &str = "INPUT";
pub const SIG_LOCKED_SINGLE_OUTPUT: &str = "SIG_LOCKED_SINGLE_OUTPUT";
pub const SIG_LOCKED_DUST_ALLOWANCE_OUTPUT: &str = "SIG_LOCKED_DUST_ALLOWANCE_OUTPUT";

// operation status
pub const SUCCESS: &str = "Success";
pub const SKIPPED: &str = "Skipped";

// operation coin actions
pub const UTXO_CONSUMED: &str = "coin_spent"; // UTXO Input, where coins are coming from into the Transaction
pub const UTXO_CREATED: &str = "coin_created"; // UTXO Output, where coins are going out from the Transaction

pub fn operation_type_list() -> Vec<String> {
    vec![
        INPUT.into(),
        SIG_LOCKED_SINGLE_OUTPUT.into(),
        SIG_LOCKED_DUST_ALLOWANCE_OUTPUT.into(),
    ]
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
    amount: u64,
    output_index: u16,
    operation_counter: usize,
    consumed: bool,
    online: bool,
) -> Operation {
    let account = AccountIdentifier {
        address,
    };

    let amount = Amount {
        value: match consumed {
            true => (amount as i64 * -1).to_string(),
            false => amount.to_string(),
        },
        currency: iota_currency(),
    };

    let output_id = format!("{}{}", transaction_id, hex::encode(output_index.to_le_bytes()));

    Operation {
        operation_identifier: OperationIdentifier {
            index: operation_counter as u64,
            network_index: Some(output_index as u64),
        },
        type_: INPUT.into(),
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
    }
}

pub fn utxo_output_operation(
    address: String,
    amount: u64,
    operation_counter: usize,
    online: bool,
    output_id: Option<OutputId>,
) -> Operation {
    let account = AccountIdentifier {
        address,
    };

    let amount = Amount {
        value: amount.to_string(),
        currency: iota_currency(),
    };

    Operation {
        operation_identifier: OperationIdentifier {
            index: operation_counter as u64,
            network_index: None,
        },
        type_: SIG_LOCKED_SINGLE_OUTPUT.into(),
        status: match online {
            true => Some(SUCCESS.into()),
            false => None,
        },
        account: Some(account),
        amount: Some(amount),
        coin_change: output_id.map(|output_id| CoinChange {
            coin_identifier: CoinIdentifier {
                identifier: output_id.to_string(),
            },
            coin_action: CoinAction::CoinCreated,
        }),
    }
}

pub fn dust_allowance_output_operation(
    address: String,
    amnt: u64,
    operation_counter: usize,
    online: bool,
    output_id: Option<OutputId>,
) -> Operation {
    let account = AccountIdentifier {
        address,
    };

    let amount = Amount {
        value: amnt.to_string(),
        currency: iota_currency(),
    };

    Operation {
        operation_identifier: OperationIdentifier {
            index: operation_counter as u64,
            network_index: None,
        },
        type_: SIG_LOCKED_DUST_ALLOWANCE_OUTPUT.into(),
        status: match online {
            true => Some(SUCCESS.into()),
            false => None,
        },
        account: Some(account),
        amount: Some(amount),
        coin_change: output_id.map(|output_id| CoinChange {
            coin_identifier: CoinIdentifier {
                identifier: output_id.to_string(),
            },
            coin_action: CoinAction::CoinCreated,
        }),
    }
}


