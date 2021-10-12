use crate::dummy_node::dummy_node::{start_dummy_node};

use rosetta_iota_server::RosettaConfig;
use rosetta_iota_server::config::RosettaMode;
use rosetta_iota_server::data::network::options::{NetworkOptionsRequest, network_options};
use rosetta_iota_server::types::NetworkIdentifier;

use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_options() {

    let dummy_node = start_dummy_node("127.0.0.1:3029".to_string()).await;

    let request = NetworkOptionsRequest {
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

    let response = network_options(request, server_options).await.unwrap();

    assert_eq!("1.4.10", response.version.rosetta_version);
    assert_eq!("0.6.0-alpha", response.version.node_version);
    assert_eq!("0.6.0-alpha", response.version.middleware_version);

    assert_eq!("Success", response.allow.operation_statuses[0].status);
    assert_eq!(true, response.allow.operation_statuses[0].successful);

    assert_eq!("INPUT", response.allow.operation_types[0]);
    assert_eq!("SIG_LOCKED_SINGLE_OUTPUT", response.allow.operation_types[1]);
    assert_eq!("SIG_LOCKED_DUST_ALLOWANCE_OUTPUT", response.allow.operation_types[2]);

    assert_eq!(1, response.allow.errors[0].code);
    assert_eq!("non retriable error", response.allow.errors[0].message);
    assert_eq!(false, response.allow.errors[0].retriable);
    assert_eq!(false, response.allow.errors[0].details.is_some());

    dummy_node.shutdown().await;
}