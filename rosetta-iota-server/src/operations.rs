// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    consts::iota_currency,
    error::ApiError,
    types::{AccountIdentifier, Amount, CoinAction, CoinChange, CoinIdentifier, Operation, OperationIdentifier},
    RosettaConfig,
};
use bee_message::{
    address::Address,
    output::{Output, SignatureLockedSingleOutput},
    prelude::{OutputId, SignatureLockedDustAllowanceOutput},
};

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

pub fn build_utxo_input_operation(
    output_id: &OutputId,
    output: &Output,
    operation_index: usize,
    online: bool,
    rosetta_config: &RosettaConfig,
) -> Result<Operation, ApiError> {
    let (amount, ed25519_address) = match output {
        Output::SignatureLockedSingle(r) => match r.address() {
            Address::Ed25519(addr) => (r.amount(), *addr),
        },
        Output::SignatureLockedDustAllowance(r) => match r.address() {
            Address::Ed25519(addr) => (r.amount(), *addr),
        },
        _ => return Err(ApiError::NonRetriable("output type not supported".to_string())),
    };

    let account = AccountIdentifier {
        address: Address::Ed25519(ed25519_address).to_bech32(&rosetta_config.bech32_hrp),
    };

    let amount = Amount {
        value: (-(amount as i64)).to_string(),
        currency: iota_currency(),
    };

    Ok(Operation {
        operation_identifier: OperationIdentifier {
            index: operation_index as u64,
            network_index: Some(output_id.index() as u64),
        },
        type_: INPUT.into(),
        status: match online {
            true => Some(SUCCESS.into()), // call coming from /data/block
            false => None,                // call coming from /construction/parse
        },
        account: Some(account),
        amount: Some(amount),
        coin_change: Some(CoinChange {
            coin_identifier: CoinIdentifier {
                identifier: output_id.to_string(),
            },
            coin_action: CoinAction::CoinSpent,
        }),
    })
}

pub fn build_sig_locked_single_output_operation(
    output_id: Option<OutputId>,
    output: &SignatureLockedSingleOutput,
    operation_counter: usize,
    online: bool,
    rosetta_config: &RosettaConfig,
) -> Result<Operation, ApiError> {
    let account = AccountIdentifier {
        address: output.address().to_bech32(&rosetta_config.bech32_hrp),
    };

    let amount = Amount {
        value: output.amount().to_string(),
        currency: iota_currency(),
    };

    Ok(Operation {
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
    })
}

pub fn build_dust_allowance_output_operation(
    output_id: Option<OutputId>,
    output: &SignatureLockedDustAllowanceOutput,
    operation_counter: usize,
    online: bool,
    rosetta_config: &RosettaConfig,
) -> Result<Operation, ApiError> {
    let account = AccountIdentifier {
        address: output.address().to_bech32(&rosetta_config.bech32_hrp),
    };

    let amount = Amount {
        value: output.amount().to_string(),
        currency: iota_currency(),
    };

    Ok(Operation {
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
    })
}
