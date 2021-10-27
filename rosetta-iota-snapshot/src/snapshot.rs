// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Config;

use rosetta_iota_server::types::{AccountIdentifier, Currency};
use rosetta_iota_server::consts::iota_currency;

use bee_common::packable::{Packable, Read};
use bee_ledger::types::{snapshot::*, BalanceDiffs};
use bee_message::prelude::*;
use bee_tangle::solid_entry_point::SolidEntryPoint;

use serde::{Deserialize, Serialize};

use std::{
    fs,
    fs::OpenOptions,
    io::BufReader,
    path::Path,
};

pub async fn balances_from_snapshot(config: &Config) {
    let full_path = Path::new("full_snapshot.bin");
    let delta_path = Path::new("delta_snapshot.bin");

    if !full_path.exists() {
        panic!("Can not find full_snapshot.bin file. Please re-setup rosetta-iota.")
    }

    let (sep_index, balance_diffs) = {
        let (sep_index, balance_diffs) = read_full_snapshot(full_path).await;
        if !delta_path.exists() {
            println!("Can not find delta_snapshot.bin file, continue nevertheless...");
            (sep_index, balance_diffs)
        } else {
            read_delta_snapshot(delta_path, balance_diffs).await
        }
    };

    save_sep_index(sep_index).await;
    save_balance_diffs(balance_diffs, &config).await;
}

async fn import_milestone_diffs<R: Read>(
    reader: &mut R,
    mut ledger_index: MilestoneIndex,
    milestone_diff_count: u64,
    balance_diffs: &mut BalanceDiffs,
) {
    for _ in 0..milestone_diff_count {
        let diff = MilestoneDiff::unpack(reader).expect("cannot unpack milestone diff");
        let index = diff.milestone().essence().index();
        let mut tmp_balance_diffs = BalanceDiffs::new();

        for (_, output) in diff.created().iter() {
            match output.inner() {
                Output::SignatureLockedSingle(output) => {
                    tmp_balance_diffs
                        .amount_add(*output.address(), output.amount())
                        .expect("can not add amount");
                    // DUST_THRESHOLD
                    if output.amount() < 1_000_000 {
                        tmp_balance_diffs
                            .dust_outputs_inc(*output.address())
                            .expect("can not increment dust outputs");
                    }
                }
                Output::SignatureLockedDustAllowance(output) => {
                    tmp_balance_diffs
                        .amount_add(*output.address(), output.amount())
                        .expect("can not add amount");
                    tmp_balance_diffs
                        .dust_allowance_add(*output.address(), output.amount())
                        .expect("can not dust allowance");
                }
                _ => panic!("unsupported output type"),
            }
        }

        for (_output_id, (created_output, _consumed_output)) in diff.consumed().iter() {
            match created_output.inner() {
                Output::SignatureLockedSingle(output) => {
                    tmp_balance_diffs
                        .amount_sub(*output.address(), output.amount())
                        .expect("can not sub amount");
                    // DUST_THRESHOLD
                    if output.amount() < 1_000_000 {
                        tmp_balance_diffs
                            .dust_outputs_dec(*output.address())
                            .expect("can not decrement dust outputs");
                    }
                }
                Output::SignatureLockedDustAllowance(output) => {
                    tmp_balance_diffs
                        .amount_sub(*output.address(), output.amount())
                        .expect("can not sub amount");
                    tmp_balance_diffs
                        .dust_allowance_sub(*output.address(), output.amount())
                        .expect("can not dust allowance");
                }
                _ => panic!("unsupported output type"),
            }
        }

        match index {
            index if index == MilestoneIndex(*ledger_index + 1) => {
                balance_diffs
                    .merge(tmp_balance_diffs)
                    .expect("can not merge balance diffs");
                ledger_index = MilestoneIndex(*ledger_index + 1)
            }
            index if index == MilestoneIndex(*ledger_index) => {
                tmp_balance_diffs.negate();
                balance_diffs
                    .merge(tmp_balance_diffs)
                    .expect("can not merge balance diffs");
                ledger_index = MilestoneIndex(*ledger_index - 1)
            }
            _ => panic!("unexpected diff index"),
        }
    }
}

async fn read_full_snapshot(full_path: &Path) -> (MilestoneIndex, BalanceDiffs) {
    println!("reading full snapshot...");

    let mut reader = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(full_path)
            .expect("could not open full snapshot"),
    );
    let header = SnapshotHeader::unpack(&mut reader).expect("can not read snapshot header");
    let full_header = FullSnapshotHeader::unpack(&mut reader).expect("can not read full snapshot header");

    for _ in 0..full_header.sep_count() {
        let _ = SolidEntryPoint::unpack(&mut reader).expect("can not read solid entry point");
    }

    let mut balance_diffs = BalanceDiffs::new();
    for _ in 0..full_header.output_count() {
        let _ = MessageId::unpack(&mut reader).expect("can not read message id of output");
        let _ = OutputId::unpack(&mut reader).expect("can not read output id");
        let output = Output::unpack(&mut reader).expect("can not read output");

        match output {
            Output::SignatureLockedSingle(output) => {
                balance_diffs
                    .amount_add(*output.address(), output.amount())
                    .expect("can not add amount");
                // DUST_THRESHOLD
                if output.amount() < 1_000_000 {
                    balance_diffs
                        .dust_outputs_inc(*output.address())
                        .expect("can not increment dust outputs");
                }
            }
            Output::SignatureLockedDustAllowance(output) => {
                balance_diffs
                    .amount_add(*output.address(), output.amount())
                    .expect("can not add amount");
                balance_diffs
                    .dust_allowance_add(*output.address(), output.amount())
                    .expect("can not add dust allowance");
            }
            _ => panic!("unsupported output type"),
        }
    }

    import_milestone_diffs(
        &mut reader,
        header.ledger_index(),
        full_header.milestone_diff_count(),
        &mut balance_diffs,
    )
    .await;

    println!("full snapshot successfully read");

    (header.sep_index(), balance_diffs)
}

async fn read_delta_snapshot(delta_path: &Path, mut balance_diffs: BalanceDiffs) -> (MilestoneIndex, BalanceDiffs) {
    println!("reading delta snapshot...");

    let mut reader = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(delta_path)
            .expect("cannot open delta snapshot"),
    );

    let header = SnapshotHeader::unpack(&mut reader).expect("cannot unpack snapshot header");
    let sep_index = header.sep_index();
    let delta_header = DeltaSnapshotHeader::unpack(&mut reader).expect("can not read delta snapshot header");

    for _ in 0..delta_header.sep_count() {
        SolidEntryPoint::unpack(&mut reader).expect("cannot unpack solid entry point");
    }

    import_milestone_diffs(
        &mut reader,
        header.ledger_index(),
        delta_header.milestone_diff_count(),
        &mut balance_diffs,
    )
    .await;

    println!("delta snapshot successfully read");

    (sep_index, balance_diffs)
}

#[derive(Serialize, Deserialize, Debug)]
struct BootstrapBalanceEntry {
    account_identifier: AccountIdentifier,
    currency: Currency,
    value: String,
}

async fn save_sep_index(sep_index: MilestoneIndex) {
    fs::write("sep_index", sep_index.to_string()).expect("cannot write to sep_index file");
}

async fn save_balance_diffs(balance_diffs: BalanceDiffs, config: &Config) {
    let mut json_entries = Vec::new();

    for (addr, balance_diff) in balance_diffs {
        let addr = addr.to_bech32(&config.bech32_hrp);

        let balance = balance_diff.amount();

        if balance > 0 {
            json_entries.push(BootstrapBalanceEntry {
                account_identifier: AccountIdentifier {
                    address: addr,
                },
                currency: iota_currency(),
                value: balance.to_string(),
            });
        }
    }

    fs::write(
        "bootstrap_balances.json",
        serde_json::to_string_pretty(&json_entries).unwrap(),
    )
    .expect("cannot write bootstrap_balances.json file");
}
