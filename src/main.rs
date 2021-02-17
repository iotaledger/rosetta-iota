// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rosetta_iota::{run_server, Options};
use structopt::StructOpt;
use std::process;

#[tokio::main]
async fn main() {
    let options = Options::from_args();

    // todo: use consts::ONLINE and consts::OFFLINE
    if (options.mode != "online") && (options.mode != "offline") {
        println!("mode needs to be either online or offline!");
        process::exit(0);
    }

    let shutdown = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    let mut binding_addr = String::from("0.0.0.0:");
    binding_addr.push_str(&options.port.to_string());

    run_server(binding_addr.parse().expect("Unable to parse socket address"), options, shutdown).await;
}
