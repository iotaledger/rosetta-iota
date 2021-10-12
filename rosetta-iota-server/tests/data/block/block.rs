use crate::dummy_node::dummy_node::{start_dummy_node};

use rosetta_iota_server::types::{NetworkIdentifier, PartialBlockIdentifier};
use rosetta_iota_server::RosettaConfig;
use rosetta_iota_server::config::RosettaMode;

use serial_test::serial;
use rosetta_iota_server::data::block::block::{BlockRequest, block};

#[tokio::test]
#[serial]
async fn test_block() {

    let dummy_node = start_dummy_node("127.0.0.1:3029".to_string()).await;

    let request = BlockRequest { network_identifier: NetworkIdentifier {
            blockchain: "iota".to_string(),
            network: "testnet7".to_string(),
            sub_network_identifier: None,
        }, block_identifier: PartialBlockIdentifier {
            index: Some(1438448),
            hash: None,
        }, };

    let server_options = RosettaConfig {
        node_url: "http://127.0.0.1:3029".to_string(),
        network: "testnet7".to_string(),
        tx_tag: "rosetta".to_string(),
        bech32_hrp: "atoi".to_string(),
        mode: RosettaMode::Online,
        bind_addr: "0.0.0.0:3030".to_string(),
    };

    let response = block(request, server_options).await.unwrap();

    dummy_node.shutdown().await;
}