// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(long)]
    pub mode: String,
}
