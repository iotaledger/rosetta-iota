// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example address --release
use iota::{api::GetAddressesBuilder, Client, Seed, MessageId};
use iota_wallet::{
    account_manager::{AccountManager, ManagerStorage},
    address::Address,
    client::ClientOptionsBuilder,
    signing::SignerType,
    Result,
};
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
use serde::Deserialize;

use std::convert::TryInto;

#[derive(Deserialize)]
struct FaucetMessageResponse {
    id: String,
}

#[derive(Deserialize)]
struct FaucetResponse {
    data: FaucetMessageResponse,
}

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

    let ed25519_address = Ed25519Address::new(hash.try_into().unwrap());

    let bech32_hrp = iota.get_bech32_hrp().await.unwrap();
    let bech32_address = ed25519_address.to_bech32(&bech32_hrp);

    println!("sk: {}", hex::encode(sk.to_le_bytes()));
    println!("pk: {}", hex::encode(pk.to_compressed_bytes()));
    println!("hash: {}", hex::encode(hash));
    println!("bech32: {}", bech32_address);

    println!("asking for funds on faucet, please wait...");
    get_funds(&bech32_address).await.expect("error: could not ask for funds!");

    let balance_response = iota.get_address().balance(&bech32_address.into()).await.unwrap();
    println!("balance: {}", balance_response.balance);

}

async fn get_funds(address: &String) -> Result<MessageId> {
    // use the faucet to get funds on the address
    let response = reqwest::get(&format!(
        "https://faucet.testnet.chrysalis2.com/api?address={}",
        address.to_string()
    ))
        .await
        .unwrap()
        .json::<FaucetResponse>()
        .await
        .unwrap();
    let faucet_message_id = MessageId::from_str(&response.data.id).expect("error: cannot talk to faucet!");

    println!("Got funds from faucet, message id: {:?}", faucet_message_id);

    Ok(faucet_message_id)
}

