// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example address --release
use iota::{api::GetAddressesBuilder, Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use bee_message::prelude::Ed25519Address;
use std::str::FromStr;

/// In this example we create addresses from a seed defined in .env
#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://honeycombos.iota.cafe:14265") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let bech32_hrp = iota.get_bech32_hrp().await.unwrap();
    let bech32_address = Ed25519Address::from_str("035450cca8a4ecf6545f2b524e3aa752dbe6338acb3d84ae00f3db8827ec728c").unwrap().to_bech32(&bech32_hrp[..]);

    println!("{}", bech32_address);
}
