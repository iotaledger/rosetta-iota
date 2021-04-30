// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use structopt::StructOpt;

use std::str::FromStr;

#[derive(Clone, Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub network: Network,
    #[structopt(long)]
    pub node_url: String,
}

#[derive(Clone, Debug, StructOpt, PartialEq)]
pub enum Network {
    Mainnet,
    Testnet,
}

impl FromStr for Network {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "mainnet" => Ok(Network::Mainnet),
            "testnet" => Ok(Network::Testnet),
            _ => Err("invalid network".to_string()),
        }
    }
}
