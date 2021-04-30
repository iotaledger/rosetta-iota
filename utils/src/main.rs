// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rosetta_iota_utils::{snapshot::bootstrap_balances_from_snapshot, Config};
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let config = Config::from_args();
    bootstrap_balances_from_snapshot(&config).await;
}
