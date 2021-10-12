use crate::dummy_node::dummy_node::{start_dummy_node};

use rosetta_iota_server::RosettaConfig;
use rosetta_iota_server::config::RosettaMode;
use rosetta_iota_server::filters::EmptyRequest;
use rosetta_iota_server::data::network::list::network_list;

use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_list() {

    let dummy_node = start_dummy_node("127.0.0.1:3029".to_string()).await;

    let server_options = RosettaConfig {
        node_url: "http://127.0.0.1:3029".to_string(),
        network: "chrysalis-mainnet".to_string(),
        tx_tag: "rosetta".to_string(),
        bech32_hrp: "atoi".to_string(),
        mode: RosettaMode::Online,
        bind_addr: "0.0.0.0:3030".to_string(),
    };

    let response = network_list(EmptyRequest, server_options).await.unwrap();

    assert_eq!("iota", response.network_identifiers[0].blockchain);
    assert_eq!("chrysalis-mainnet", response.network_identifiers[0].network);
    assert_eq!(false, response.network_identifiers[0].sub_network_identifier.is_some());

    dummy_node.shutdown().await;
}