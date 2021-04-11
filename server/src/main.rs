// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rosetta_iota_server::{run_server, Config};

use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let config = Config::from_args();

    let shutdown = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    run_server(config, shutdown).await;
}
