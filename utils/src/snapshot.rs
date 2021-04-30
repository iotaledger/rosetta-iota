// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Config;
use rosetta_iota_server::types::{AccountIdentifier, Currency};

use bee_common::packable::Packable;
use bee_ledger::types::BalanceDiffs;
use bee_message::{prelude::*};
use bee_tangle::solid_entry_point::SolidEntryPoint;

use serde::{Deserialize, Serialize};

use std::{
    fs,
    fs::{File, OpenOptions},
    io::{copy, BufReader},
    path::Path,
};

use iota::Client;
use bee_ledger::types::snapshot::*;

pub async fn bootstrap_balances_from_snapshot(config: &Config) {
    let download_url = "https://dbfiles.testnet.chrysalis2.com/";
    let full_path = Path::new("full_snapshot.bin");
    let delta_path = Path::new("delta_snapshot.bin");

    if !full_path.exists() {
        println!("downloading full snapshot...");
        download_snapshot_file(full_path, &[String::from(download_url)]).await;
    }

    if !delta_path.exists() {
        println!("downloading delta snapshot...");
        download_snapshot_file(delta_path, &[String::from(download_url)]).await;
    }

    let balance_diffs = read_full_snapshot(full_path).await;
    let (sep_index, json_string) = read_delta_snapshot(delta_path, balance_diffs, config).await;

    fs::write("bootstrap_balances.json", json_string).expect("cannot write bootstrap_balances.json file");
    fs::write("sep_index", sep_index.to_string()).expect("cannot write to sep_index file");
}

#[derive(Serialize, Deserialize, Debug)]
struct BootstrapBalanceEntry {
    account_identifier: AccountIdentifier,
    currency: Currency,
    value: String,
}

async fn read_delta_snapshot(
    delta_path: &Path,
    mut balance_diffs: BalanceDiffs,
    config: &Config,
) -> (MilestoneIndex, String) {
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
                    balance_diffs.amount_add(*output.address(), output.amount());
                    // DUST_THRESHOLD
                    if output.amount() < 1_000_000 {
                        balance_diffs.dust_outputs_inc(*output.address());
                    }
                }
                Output::SignatureLockedDustAllowance(output) => {
                    balance_diffs.amount_add(*output.address(), output.amount());
                    balance_diffs.dust_allowance_add(*output.address(), output.amount());
                }
                _ => panic!("unsupported output type"),
            }
        }

        for (_output_id, (created_output, _consumed_output)) in diff.consumed().iter() {
            match created_output.inner() {
                Output::SignatureLockedSingle(output) => {
                    balance_diffs.amount_sub(*output.address(), output.amount());
                    // DUST_THRESHOLD
                    if output.amount() < 1_000_000 {
                        balance_diffs.dust_outputs_dec(*output.address());
                    }
                }
                Output::SignatureLockedDustAllowance(output) => {
                    balance_diffs.amount_sub(*output.address(), output.amount());
                    balance_diffs.dust_allowance_sub(*output.address(), output.amount());
                }
                _ => panic!("unsupported output type"),
            }
        }
    }

    let mut json_entries = Vec::new();

    // Create iota client
    let iota = Client::builder() // Crate a client instance builder
        .with_node(&config.node_url) // Insert the node here
        .unwrap()
        .finish()
        .await
        .expect("can not build client");

    let bech32_hrp = iota.get_bech32_hrp().await.expect("can not get bech32 HRP");

    for (addr, balance_diff) in balance_diffs {
        let addr = addr.to_bech32(&bech32_hrp);

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

    let json_string = serde_json::to_string_pretty(&json_entries).unwrap();

    println!("delta snapshot successfully read...");

    (sep_index, json_string)
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
                balance_diffs.amount_add(*output.address(), output.amount());
                // DUST_THRESHOLD
                if output.amount() < 1_000_000 {
                    balance_diffs.dust_outputs_inc(*output.address());
                }
            }
            Output::SignatureLockedDustAllowance(output) => {
                balance_diffs.amount_add(*output.address(), output.amount());
                balance_diffs.dust_allowance_add(*output.address(), output.amount());
            }
            _ => panic!("unsupported output type"),
        }
    }

    println!("full snapshot successfully read...");

    balance_diffs
}

async fn download_snapshot_file(file_path: &Path, download_urls: &[String]) {
    let file_name = file_path.file_name().expect(&format!(
        "invalid file path {}",
        file_path.to_string_lossy().to_string()
    ));

    std::fs::create_dir_all(file_path.parent().expect(&format!(
        "invalid file path {}",
        file_path.to_string_lossy().to_string()
    )))
    .expect(&format!(
        "invalid file path {}",
        file_path.to_string_lossy().to_string()
    ));

    for url in download_urls {
        let url = url.to_owned() + &file_name.to_string_lossy();

        println!("downloading snapshot file {}...", url);
        match reqwest::get(&url).await {
            Ok(res) => match File::create(file_path) {
                // TODO unwrap
                Ok(mut file) => match copy(&mut res.bytes().await.unwrap().as_ref(), &mut file) {
                    Ok(_) => break,
                    Err(e) => panic!("copying snapshot file failed: {:?}", e),
                },
                Err(e) => panic!("creating snapshot file failed: {:?}", e),
            },
            Err(e) => panic!("downloading snapshot file failed: {:?}", e),
        }
    }

    if !file_path.exists() {
        panic!("no working download source available");
    }
}
