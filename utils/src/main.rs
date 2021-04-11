// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rosetta_iota_utils::{consts, faucet::ask_faucet, snapshot::bootstrap_balances_from_snapshot, Config};
use structopt::StructOpt;

use std::process;

#[tokio::main]
async fn main() {
    let config = Config::from_args();
    if (config.mode != consts::FAUCET) && (config.mode != consts::SNAPSHOT) {
        println!("utils mode needs to be either faucet or snapshot!");
        process::exit(0);
    } else if config.mode == consts::FAUCET {
        ask_faucet().await;
    } else if config.mode == consts::SNAPSHOT {
        bootstrap_balances_from_snapshot(&config).await;
    }
}
