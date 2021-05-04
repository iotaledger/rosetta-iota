// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Config;
use crate::config::Network;

use rosetta_iota_server::types::{AccountIdentifier, Currency};

use bee_common::packable::Packable;
use bee_ledger::types::BalanceDiffs;
use bee_ledger::types::snapshot::*;
use bee_message::{prelude::*};
use bee_tangle::solid_entry_point::SolidEntryPoint;

use serde::{Deserialize, Serialize};

use std::{
    fs,
    fs::{File, OpenOptions},
    io::{copy, BufReader},
    path::Path,
};

pub async fn bootstrap_balances_from_snapshot(config: &Config) {
    let (full_url, delta_url) = match &config.network {
        Network::ChrysalisMainnet => (
            "https://chrysalis-dbfiles.iota.org/snapshots/hornet/latest-full_snapshot.bin",
            "https://chrysalis-dbfiles.iota.org/snapshots/hornet/latest-delta_snapshot.bin",
        ),
        Network::Testnet7 => (
            "https://dbfiles.testnet.chrysalis2.com/full_snapshot.bin",
            "https://dbfiles.testnet.chrysalis2.com/delta_snapshot.bin"
        )
    };

    let full_path = Path::new("full_snapshot.bin");
    let delta_path = Path::new("delta_snapshot.bin");

    if !full_path.exists() {
        download_snapshot_file(full_path, &full_url.to_string()).await;
    }

    if !delta_path.exists() {
        download_snapshot_file(delta_path, &delta_url.to_string()).await;
    }

    let balance_diffs = read_full_snapshot(full_path).await;
    let (sep_index, balance_diffs) = read_delta_snapshot(delta_path, balance_diffs).await;

    save_sep_index(sep_index).await;
    save_balance_diffs(balance_diffs, &config).await;
}

async fn read_full_snapshot(full_path: &Path) -> BalanceDiffs {
    println!("reading full snapshot...");

    let mut reader = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(full_path)
            .expect("could not open full snapshot"),
    );
    let _header = SnapshotHeader::unpack(&mut reader).expect("can not read snapshot header");
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
                balance_diffs.amount_add(*output.address(), output.amount()).expect("can not add amount");
                // DUST_THRESHOLD
                if output.amount() < 1_000_000 {
                    balance_diffs.dust_outputs_inc(*output.address()).expect("can not increment dust outputs");
                }
            }
            Output::SignatureLockedDustAllowance(output) => {
                balance_diffs.amount_add(*output.address(), output.amount()).expect("can not add amount");
                balance_diffs.dust_allowance_add(*output.address(), output.amount()).expect("can not add dust allowance");
            }
            _ => panic!("unsupported output type"),
        }
    }

    println!("full snapshot successfully read");

    balance_diffs
}

async fn read_delta_snapshot(
    delta_path: &Path,
    mut balance_diffs: BalanceDiffs,
) -> (MilestoneIndex, BalanceDiffs) {
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

    for _ in 0..delta_header.milestone_diff_count() {
        let diff = MilestoneDiff::unpack(&mut reader).expect("cannot unpack milestone diff");
        for (_, output) in diff.created().iter() {
            match output.inner() {
                Output::SignatureLockedSingle(output) => {
                    balance_diffs.amount_add(*output.address(), output.amount()).expect("can not add amount");
                    // DUST_THRESHOLD
                    if output.amount() < 1_000_000 {
                        balance_diffs.dust_outputs_inc(*output.address()).expect("can not increment dust outputs");
                    }
                }
                Output::SignatureLockedDustAllowance(output) => {
                    balance_diffs.amount_add(*output.address(), output.amount()).expect("can not add amount");
                    balance_diffs.dust_allowance_add(*output.address(), output.amount()).expect("can not dust allowance");
                }
                _ => panic!("unsupported output type"),
            }
        }

        for (_output_id, (created_output, _consumed_output)) in diff.consumed().iter() {
            match created_output.inner() {
                Output::SignatureLockedSingle(output) => {
                    balance_diffs.amount_sub(*output.address(), output.amount()).expect("can not sub amount");
                    // DUST_THRESHOLD
                    if output.amount() < 1_000_000 {
                        balance_diffs.dust_outputs_dec(*output.address()).expect("can not decrement dust outputs");
                    }
                }
                Output::SignatureLockedDustAllowance(output) => {
                    balance_diffs.amount_sub(*output.address(), output.amount()).expect("can not sub amount");
                    balance_diffs.dust_allowance_sub(*output.address(), output.amount()).expect("can not dust allowance");
                }
                _ => panic!("unsupported output type"),
            }
        }
    }

    println!("delta snapshot successfully read");

    (sep_index, balance_diffs)
}

async fn save_balance_diffs(balance_diffs: BalanceDiffs, config: &Config,) {
    let mut json_entries = Vec::new();

    for (addr, balance_diff) in balance_diffs {
        let addr = addr.to_bech32(&config.bech32_hrp);

        let balance = balance_diff.amount();

        if balance > 0 {
            json_entries.push(BootstrapBalanceEntry {
                account_identifier: AccountIdentifier {
                    address: addr,
                    sub_account: None,
                },
                currency: Currency {
                    symbol: "IOTA".to_string(),
                    decimals: 0,
                    metadata: None,
                },
                value: balance.to_string(),
            });
        }
    }

    fs::write("bootstrap_balances.json", serde_json::to_string_pretty(&json_entries).unwrap()).expect("cannot write bootstrap_balances.json file");
}

async fn save_sep_index(sep_index: MilestoneIndex) {
    fs::write("sep_index", sep_index.to_string()).expect("cannot write to sep_index file");
}

async fn download_snapshot_file(file_path: &Path, url: &String) {
    std::fs::create_dir_all(file_path.parent().expect(&format!(
        "invalid file path {}",
        file_path.to_string_lossy().to_string()
    )))
    .expect(&format!(
        "invalid file path {}",
        file_path.to_string_lossy().to_string()
    ));

    println!("downloading snapshot file {}...", url);

    match reqwest::get(url).await {
        Ok(res) => match File::create(file_path) {
            // TODO unwrap
            Ok(mut file) => match copy(&mut res.bytes().await.unwrap().as_ref(), &mut file) {
                Ok(_) => {},
                Err(e) => panic!("copying snapshot file failed: {:?}", e),
            },
            Err(e) => panic!("creating snapshot file failed: {:?}", e),
        },
        Err(e) => panic!("downloading snapshot file failed: {:?}", e),
    }

    if !file_path.exists() {
        panic!("no working download source available");
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BootstrapBalanceEntry {
    account_identifier: AccountIdentifier,
    currency: Currency,
    value: String,
}