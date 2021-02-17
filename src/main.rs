// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rosetta_iota::{run_server, Options};
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let options = Options::from_args();

    let shutdown = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    let mut binding_addr = String::from("0.0.0.0:");
    binding_addr.push_str(&options.port.to_string());

    run_server(binding_addr.parse().expect("Unable to parse socket address"), options, shutdown).await;
}
