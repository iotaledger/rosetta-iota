// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(long)]
    pub iota_endpoint: String,
    #[structopt(long)]
    pub network: String,
    #[structopt(long)]
    pub mode: String,
    #[structopt(long)]
    pub port: u16,
}
