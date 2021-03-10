
use bee_message::prelude::*;
use bee_common::packable::Packable;
use bee_rest_api::types::OutputDto;
use iota::Client;
use iota::{ AddressOutputsOptions, OutputType};

use rosetta_iota::operations::*;
use rosetta_iota::Options;
use rosetta_iota::consts::OFFLINE_MODE;
use rosetta_iota::types::*;
use rosetta_iota::construction::derive::{construction_derive_request, ConstructionDeriveResponse, ConstructionDeriveRequest};
use rosetta_iota::types::Operation;
use rosetta_iota::operations::UTXO_INPUT;
use bee_rest_api::endpoints::api::v1::balance_ed25519::BalanceForAddressResponse;
use bee_rest_api::endpoints::api::v1::output::OutputResponse;

const DEFAULT_NODE_URL: &str = "https://api.lb-0.testnet.chrysalis2.com";

#[tokio::test]
async fn test_transfer_funds() {

    /// This function creates a transfer of funds from Alice (the sender) to Bob (the receiver).

    /// Describes how much should be transferred from Alice to Bob.
    let amount_to_transfer = 5;

    /// The public key and secret key of Alice. These keys serve for TESTING PURPOSES ONLY AND CAN BE REPLACED.
    /// The secret key will be used to legitimate/sign the transfer of funds.

    let public_key = "82eeba00688da228b83bbe32d6c2e2d548550ab3c6e30752d9fe2617e89f554d";
    let secret_key = "7e828a3c369f1d963685aae2354ab7f3509bed9e6244a7d4c370daccb37ca606";

    println!("Public key of Alice: {}", public_key);
    println!("Secret key of Alice: {}", secret_key);

    /// 1) Derive the address from the public key
    let derive_response: ConstructionDeriveResponse = {
        let request = ConstructionDeriveRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet6".to_string(),
                sub_network_identifier: None
            },
            public_key: PublicKey { hex_bytes: public_key.to_string(), curve_type: CurveType::Edwards25519 }
        };
        let rosetta_options = Options {
            iota_endpoint: DEFAULT_NODE_URL.to_string(),
            network: "testnet6".to_string(),
            bech32_hrp: "atoi".to_string(),
            mode: OFFLINE_MODE.into(),
            port: 3030
        };
        construction_derive_request(request, rosetta_options).await.expect("derive request failed")
    };

    let bech32_address = derive_response.account_identifier.address;
    println!("Address of Alice: {}", bech32_address);

    // 2) Check if the address of Alice has enough funds to transfer
    let balance = balance_of_address(bech32_address.clone()).await;
    println!("Balance of Alice's address: {}", balance);
    if balance < amount_to_transfer {
        panic!("Not enough funds on Alice's address: expected {} iota but found {} iota on the address. Please fund Alice's address.", amount_to_transfer, balance);
    }

    // 3) get all unspent outputs from the Alice's address.
    let unspent_outputs = unspent_outputs_of_address(bech32_address).await;

    // 4) create operations that consume the unspent outputs









}

async fn balance_of_address(bech32_addr: String) -> u64 {
    let iota_client = iota::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();
    iota_client.get_address().balance(&Bech32Address(bech32_addr)).await.unwrap().balance
}

async fn unspent_outputs_of_address(bech32_addr: String) -> Vec<OutputResponse> {
    let iota_client = iota::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();
    let output_ids = iota_client.get_address().outputs(&Bech32Address(bech32_addr), AddressOutputsOptions { include_spent: false, output_type: Some(OutputType::SignatureLockedSingle) }).await.unwrap();
    iota_client.find_outputs(&output_ids, &[]).await.unwrap()
}
