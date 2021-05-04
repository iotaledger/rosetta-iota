// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use structopt::StructOpt;

use std::str::FromStr;

#[derive(Clone, Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub bind_addr: String,
    #[structopt(long)]
    pub network: String,
    #[structopt(long)]
    pub bech32_hrp: String,
    #[structopt(long, default_value = "Rosetta")]
    pub tx_tag: String,
    #[structopt(long)]
    pub node_url: String,
    #[structopt(long, default_value = "online")]
    pub mode: RosettaMode,
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
            _ => Err("invalid mode".to_string()),
        }
    }
}
