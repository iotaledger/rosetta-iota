// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rosetta_iota_server::types::{AccountIdentifier, Currency};
use iota::Client;
use log::{error, info, warn};
use std::{fs, fs::File, io::copy, path::Path};
use thiserror::Error;
use bee_snapshot::{ header::SnapshotHeader, milestone_diff::MilestoneDiff};
use bee_ledger::{types::BalanceDiffs};
use bee_common::packable::Packable;
use std::{io::BufReader, fs::OpenOptions};
use bee_message::prelude::*;
use std::collections::HashMap;
use bee_message::solid_entry_point::SolidEntryPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub enum Error {
    #[error("")]
    InvalidFilePath(String),
    #[error("")]
    NoDownloadSourceAvailable,
    #[error("")]
    UnsupportedOutputKind,
}

pub async fn bootstrap_balances_from_snapshot() {

    let full_path = Path::new("full_snapshot.bin");
    let delta_path = Path::new("delta_snapshot.bin");
    let url = "https://dbfiles.testnet.chrysalis2.com/";

    if !full_path.exists() {
        println!("Downloading full snapshot...");
        download_snapshot_file(full_path,
                               &[String::from(url)]).await.unwrap();
    }

    if !delta_path.exists() {
        println!("Downloading delta snapshot...");
        download_snapshot_file(delta_path,
                               &[String::from(url)]).await.unwrap();
    }

    let mut balance_diffs = read_full_outputs(full_path).await;

    let sep_index = read_sep_index(delta_path).await;
    let json_string = read_delta_diff(delta_path, balance_diffs).await;
    fs::write("bootstrap_balances.json", json_string).expect("cannot write to file");
    fs::write("sep_index", sep_index.to_string()).expect("cannot write to file");
}


#[derive(Serialize, Deserialize, Debug)]
struct BootstrapBalanceEntry {
    account_identifier: AccountIdentifier,
    currency: Currency,
    value: String
}

async fn read_delta_diff(delta_path: &Path, mut balance_diffs: BalanceDiffs) -> String {
    let mut reader = BufReader::new(OpenOptions::new().read(true).open(delta_path).expect("cannot read buffer"));

    let header = SnapshotHeader::unpack(&mut reader).expect("cannot unpack snapshot header");
    for _ in 0..header.sep_count() {
        SolidEntryPoint::unpack(&mut reader).expect("cannot unpack sep");
    }

    for _ in 0..header.milestone_diff_count() {
        let diff = MilestoneDiff::unpack(&mut reader).expect("cannot unpack milestone diff");
        for (_, output) in diff.created().iter() {
            match output.inner() {
                Output::SignatureLockedSingle(output) => {
                    balance_diffs.amount_add(*output.address(), output.amount());
                    // DUST_THRESHOLD
                    if output.amount() < 1_000_000 {
                        balance_diffs.dust_output_inc(*output.address());
                    }
                }
                Output::SignatureLockedDustAllowance(output) => {
                    balance_diffs.amount_add(*output.address(), output.amount());
                    balance_diffs.dust_allowance_add(*output.address(), output.amount());
                }
                _ => panic!("unsuported output kind"),
            }
        }

        let mut consumed = HashMap::new();

        for (output_id, (created_output, consumed_output)) in diff.consumed().iter() {
            match created_output.inner() {
                Output::SignatureLockedSingle(output) => {
                    balance_diffs.amount_sub(*output.address(), output.amount());
                    // DUST_THRESHOLD
                    if output.amount() < 1_000_000 {
                        balance_diffs.dust_output_dec(*output.address());
                    }
                }
                Output::SignatureLockedDustAllowance(output) => {
                    balance_diffs.amount_sub(*output.address(), output.amount());
                    balance_diffs.dust_allowance_sub(*output.address(), output.amount());
                }
                _ => panic!("unsuported output kind"),
            }
            consumed.insert(*output_id, (*consumed_output).clone());
        }
    }

    let iota = Client::builder() // Crate a client instance builder
        .with_node("https://api.hornet-rosetta.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();
    let bech32_hrp = iota.get_bech32_hrp().await.unwrap();

    let mut json_entries = Vec::new();

    for (addr, balance_diff) in balance_diffs {
        let addr = addr.to_bech32(&bech32_hrp);

        // is this correct?
        let balance = balance_diff.amount();

        if balance > 0 {
            json_entries.push(BootstrapBalanceEntry {
                account_identifier: AccountIdentifier {
                    address: addr,
                    sub_account: None
                },
                currency: Currency {
                    symbol: "IOTA".to_string(),
                    decimals: 0,
                    metadata: None
                },
                value: balance.to_string()
            });
        }
    }

    let json_string = serde_json::to_string_pretty(&json_entries).unwrap();

    json_string
}

async fn read_sep_index(delta_path: &Path) -> MilestoneIndex {
    println!("Reading delta snapshot...");
    let mut reader = BufReader::new(OpenOptions::new().read(true).open(delta_path).expect("could not open delta snapshot"));
    let header = SnapshotHeader::unpack(&mut reader).unwrap();

    for _ in 0..header.sep_count() {
        let _ = SolidEntryPoint::unpack(&mut reader).expect("Can not read solid entry point.");
    }

    let sep_index = header.sep_index();

    println!("Delta snapshot successfully read.");

    sep_index
}

async fn read_full_outputs(full_path: &Path) -> BalanceDiffs{
    println!("Reading full snapshot...");

    let mut reader = BufReader::new(OpenOptions::new().read(true).open(full_path).expect("Could not open full snapshot."));
    let header = SnapshotHeader::unpack(&mut reader).expect("Can not read snapshot header.");

    for _ in 0..header.sep_count() {
        let _ = SolidEntryPoint::unpack(&mut reader).expect("Can not read solid entry point.");
    }

    let mut count = 0;
    let mut balance_diffs = BalanceDiffs::new();
    for _ in 0..header.output_count() {
        let message_id = MessageId::unpack(&mut reader).expect("Can not read message id of output.");
        let output_id = OutputId::unpack(&mut reader).expect("Can not read output id.");
        let output = Output::unpack(&mut reader).expect("Can not read output.");

        match output {
            Output::SignatureLockedSingle(output) => {
                balance_diffs.amount_add(*output.address(), output.amount());
                // DUST_THRESHOLD
                if output.amount() < 1_000_000 {
                    balance_diffs.dust_output_inc(*output.address());
                }
            }
            Output::SignatureLockedDustAllowance(output) => {
                balance_diffs.amount_add(*output.address(), output.amount());
                balance_diffs.dust_allowance_add(*output.address(), output.amount());
            }
            _ => panic!("unsuported output kind"),
        }
    }

    println!("Full snapshot successfully read.");

    balance_diffs
}

async fn download_snapshot_file(file_path: &Path, download_urls: &[String]) -> Result<(), Error> {
    let file_name = file_path
        .file_name()
        .ok_or_else(|| Error::InvalidFilePath(file_path.to_string_lossy().to_string()))?;

    std::fs::create_dir_all(
        file_path
            .parent()
            .ok_or_else(|| Error::InvalidFilePath(file_path.to_string_lossy().to_string()))?,
    )
        .map_err(|_| Error::InvalidFilePath(file_path.to_string_lossy().to_string()))?;

    for url in download_urls {
        let url = url.to_owned() + &file_name.to_string_lossy();

        info!("Downloading snapshot file {}...", url);
        match reqwest::get(&url).await {
            Ok(res) => match File::create(file_path) {
                // TODO unwrap
                Ok(mut file) => match copy(&mut res.bytes().await.unwrap().as_ref(), &mut file) {
                    Ok(_) => break,
                    Err(e) => warn!("Copying snapshot file failed: {:?}.", e),
                },
                Err(e) => warn!("Creating snapshot file failed: {:?}.", e),
            },
            Err(e) => warn!("Downloading snapshot file failed: {:?}.", e),
        }
    }

    if !file_path.exists() {
        error!("No working download source available.");
        return Err(Error::NoDownloadSourceAvailable);
    }

    Ok(())
}