// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub mode: String,
    #[structopt(long)]
    pub bech32_hrp: String,
}
