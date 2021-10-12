use crate::dummy_node::dummy_node::{start_dummy_node};

use rosetta_iota_server::types::{NetworkIdentifier, AccountIdentifier};
use rosetta_iota_server::RosettaConfig;
use rosetta_iota_server::config::RosettaMode;
use rosetta_iota_server::data::account::coins::*;

use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_coins() {

    let dummy_node = start_dummy_node("127.0.0.1:3029".to_string()).await;

    let request = AccountCoinsRequest {
        network_identifier: NetworkIdentifier {
            blockchain: "iota".to_string(),
            network: "chrysalis-mainnet".to_string(),
            sub_network_identifier: None,
        },
        account_identifier: AccountIdentifier {
            address: String::from("iota1qp6gwwy7rruk0d3j9fqzcxnfrstfedk2m65jst2tx7xmkad4agjc5r7ptjz"),
            sub_account: None,
        }
    };

    let server_options = RosettaConfig {
        node_url: "http://127.0.0.1:3029".to_string(),
        network: "chrysalis-mainnet".to_string(),
        tx_tag: "rosetta".to_string(),
        bech32_hrp: "iota".to_string(),
        mode: RosettaMode::Online,
        bind_addr: "0.0.0.0:3030".to_string(),
    };

    let response = account_coins(request, server_options).await.unwrap();

    assert_eq!(1438495, response.block_identifier.index);
    assert_eq!(
        "1438495",
        response.block_identifier.hash
    );
    assert_eq!(1, response.coins.len());
    assert_eq!("20651169480", response.coins[0].amount.value);
    assert_eq!(
        "d2e2faaf394a5d22045668a55df27c9abe2057c1f2ce319999bed373269b50190000",
        response.coins[0].coin_identifier.identifier
    );

    dummy_node.shutdown().await;
}