// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example address --release
use iota::{api::GetAddressesBuilder, Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use bee_message::prelude::Ed25519Address;
use std::str::FromStr;
use rand::thread_rng;
use crypto::{
    ed25519::{SecretKey, PublicKey},
    hashes::{blake2b::Blake2b256, Digest}
};

/// In this example we create addresses from a seed defined in .env
#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://honeycombos.iota.cafe:14265") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // Generate a signing key
    let sk = SecretKey::generate().expect("error: could not generate SecretKey!");
    let pk = sk.public_key();

    let pk_bytes = pk.to_compressed_bytes().to_vec();
    let hash = Blake2b256::digest(&pk_bytes);

    // todo
    // let bech32_hrp = iota.get_bech32_hrp().await.unwrap();
    // let bech32_address = Ed25519Address::new(hash.try_into().unwrap());

    println!("sk: {}", hex::encode(sk.to_le_bytes()));
    println!("pk: {}", hex::encode(pk.to_compressed_bytes()));
    println!("hash: {}", hex::encode(hash));

    // todo
    // println!("bech32{}", bech32_address);
}
