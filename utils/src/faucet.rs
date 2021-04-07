// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{Client};
use bee_message::prelude::*;
use crypto::{
    ed25519::SecretKey,
    hashes::{blake2b::Blake2b256, Digest}
};
use serde::{Serialize, Deserialize};
use serde_json;

use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Deserialize)]
struct FaucetMessageResponse {
    pub id: String,
}

#[derive(Deserialize)]
struct FaucetResponse {
    pub data: FaucetMessageResponse,
}

#[derive(Serialize)]
struct PrefundedAccount {
    sk: String,
    pk: String,
    pk_hash: String,
    bech32_addr: String,
    balance: u64
}

pub async fn ask_faucet() {

    // Create iota client
    let iota = Client::builder() // Crate a client instance builder
        .with_node("https://api.lb-0.testnet.chrysalis2.com/") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // Generate a keypair
    let sk = SecretKey::generate().expect("error: could not generate SecretKey!");
    let pk = sk.public_key();

    // Generate address
    let pk_bytes = pk.to_compressed_bytes().to_vec();
    let hash = Blake2b256::digest(&pk_bytes);

    let ed25519_address = Ed25519Address::new(hash.try_into().unwrap());

    // Get bech32 representation
    let bech32_hrp = iota.get_bech32_hrp().await.unwrap();
    let bech32_address = Address::Ed25519(ed25519_address).to_bech32(&bech32_hrp);

    // ask for 10000000i twice (uncomment for IF or TangleKit faucet)
    //ask_if_faucet_twice(&bech32_address, &iota);
    ask_tanglekit_faucet_twice(&bech32_address).await;

    // wait for consensus on the ledger
    let mut balance_response;
    loop {
        balance_response = iota.get_address().balance(&bech32_address).await.unwrap();
        if balance_response.balance == 20000000 {
            break;
        }
    }

    // Construct JSON
    let prefunded_account = PrefundedAccount {
        sk: hex::encode(sk.to_le_bytes()),
        pk: hex::encode(pk.to_compressed_bytes()),
        pk_hash: hex::encode(hash),
        bech32_addr: bech32_address,
        balance: balance_response.balance,
    };

    let prefunded_account_pretty = serde_json::to_string_pretty(&prefunded_account).expect("error: could not pretty-print prefunded_account");

    println!("{}", prefunded_account_pretty);
}

async fn ask_tanglekit_faucet_twice(bech32_address: &String) {
    get_funds_tanglekit_faucet(&bech32_address).await;
    get_funds_tanglekit_faucet(&bech32_address).await;
}

async fn get_funds_tanglekit_faucet(address: &String) {
    let mut map = HashMap::new();
    map.insert("address", address.to_string());

    let client = reqwest::Client::new();
    let _ = client.post("https://faucet.tanglekit.de/api/enqueue")
        .json(&map)
        .send()
        .await
        .expect("could not send POST");
}