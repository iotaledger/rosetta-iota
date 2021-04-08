// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rosetta_iota_server::{consts, run_server, Options};

use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let options = Options::from_args();

    if (options.mode != consts::ONLINE_MODE) && (options.mode != consts::OFFLINE_MODE) {
        panic!("mode needs to be either online or offline");
    }

    let shutdown = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    run_server(
        options,
        shutdown,
    )
    .await;
}
