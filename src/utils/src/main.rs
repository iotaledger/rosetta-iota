// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{Client, MessageId};
use iota_wallet::Result;
use bee_message::prelude::Ed25519Address;
use std::str::FromStr;
use crypto::{
    ed25519::SecretKey,
    hashes::{blake2b::Blake2b256, Digest}
};
use serde::{Serialize, Deserialize};
use serde_json;

use std::time::Duration;
use tokio::time::sleep;
use std::convert::TryInto;

#[derive(Deserialize)]
struct FaucetMessageResponse {
    id: String,
}

#[derive(Deserialize)]
struct FaucetResponse {
    data: FaucetMessageResponse,
}

#[derive(Serialize)]
struct PrefundedAccount {
    sk: String,
    pk: String,
    pk_hash: String,
    bech32_addr: String,
    balance: u64
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

    // ask for funds twice
    let message_id = get_funds(&bech32_address).await.expect("error: could not ask for funds!");

    sleep(Duration::from_secs(5)).await;
    reattach_promote_until_confirmed(&message_id, &iota).await;

    sleep(Duration::from_secs(5)).await;
    let message_id = get_funds(&bech32_address).await.expect("error: could not ask for funds!");

    sleep(Duration::from_secs(5)).await;
    reattach_promote_until_confirmed(&message_id, &iota).await;

    let balance_response = iota.get_address().balance(&bech32_address.clone().into()).await.unwrap();

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

    Ok(faucet_message_id)
}

async fn reattach_promote_until_confirmed(message_id: &MessageId, iota: &Client) {
    while let Ok(metadata) = iota.get_message().metadata(&message_id).await {
        if metadata.referenced_by_milestone_index.is_some() {
            break;
        } else if let Ok(msg_id) = iota.reattach(&message_id).await {
            println!("Reattached or promoted {}", msg_id.0);
        }
        sleep(Duration::from_secs(2)).await;
    }
}