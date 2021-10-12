use crate::dummy_node::dummy_node::{start_dummy_node};

use rosetta_iota_server::RosettaConfig;
use rosetta_iota_server::config::RosettaMode;
use rosetta_iota_server::data::network::status::{NetworkStatusRequest, network_status};
use rosetta_iota_server::types::NetworkIdentifier;

use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_status() {

    let dummy_node = start_dummy_node("127.0.0.1:3029".to_string()).await;

    let request = NetworkStatusRequest {
        network_identifier: NetworkIdentifier {
            blockchain: "iota".to_string(),
            network: "testnet7".to_string(),
            sub_network_identifier: None,
        },
    };

    let server_options = RosettaConfig {
        node_url: "http://127.0.0.1:3029".to_string(),
        network: "testnet7".to_string(),
        tx_tag: "rosetta".to_string(),
        bech32_hrp: "atoi".to_string(),
        mode: RosettaMode::Online,
        bind_addr: "0.0.0.0:3030".to_string(),
    };

    let response = network_status(request, server_options).await.unwrap();

    assert_eq!(1438448, response.current_block_identifier.index);
    assert_eq!(
        "1438448",
        response.current_block_identifier.hash
    );
    assert_eq!(1634052071000, response.current_block_timestamp);

    dummy_node.shutdown().await;
}