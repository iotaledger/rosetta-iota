// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rosetta_iota_utils::{Options, consts, faucet::ask_faucet};
use structopt::StructOpt;
use std::process;

#[tokio::main]
async fn main() {
    let options = Options::from_args();
    if (options.mode != consts::FAUCET) && (options.mode != consts::SNAPSHOT) {
        println!("utils mode needs to be either faucet or snapshot!");
        process::exit(0);
    } else if options.mode == consts::FAUCET {
        ask_faucet().await;
    } else if options.mode == consts::SNAPSHOT {

    }

}