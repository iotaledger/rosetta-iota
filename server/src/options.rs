// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use structopt::StructOpt;

use std::str::FromStr;

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(long)]
    pub node: String,
    #[structopt(long)]
    pub network: String,
    #[structopt(long)]
    pub bech32_hrp: String,
    #[structopt(long)]
    pub indexation: String,
    #[structopt(long = "mode", default_value = "online")]
    pub mode: RosettaMode,
    #[structopt(long)]
    pub bind_addr: String,
}

#[derive(Clone, Debug, StructOpt, PartialEq)]
pub enum RosettaMode {
    Online,
    Offline,
}

impl FromStr for RosettaMode {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "online" => Ok(RosettaMode::Online),
            "offline" => Ok(RosettaMode::Offline),
            _ => Err("can not parse mode".to_string()),
        }
    }
}