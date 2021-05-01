// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use structopt::StructOpt;

use std::str::FromStr;

#[derive(Clone, Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub network: Network,
    #[structopt(long)]
    pub bech32_hrp: String,
}

#[derive(Clone, Debug, StructOpt, PartialEq)]
pub enum Network {
    ChrysalisMainnet,
    Testnet7,
}

impl FromStr for Network {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "chrysalis-mainnet" => Ok(Network::ChrysalisMainnet),
            "testnet7" => Ok(Network::Testnet7),
            _ => Err("invalid network".to_string()),
        }
    }
}
