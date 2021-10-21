use crate::dummy_node::dummy_node::{start_dummy_node};
use crate::config::{VALID_BLOCKCHAIN, VALID_NETWORK, WRONG_BLOCKCHAIN, WRONG_NETWORK};
use crate::{test_request, Request};

use rosetta_iota_server::RosettaConfig;
use rosetta_iota_server::config::RosettaMode;
use rosetta_iota_server::data::network::status::{NetworkStatusRequest, network_status};
use rosetta_iota_server::types::NetworkIdentifier;

use serial_test::serial;
use rosetta_iota_server::data::network::options::NetworkOptionsRequest;

#[tokio::test]
#[serial]
async fn valid_request() {
    let request = NetworkStatusRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
            sub_network_identifier: None,
        },
    };

    let response = test_request(Request::NetworkStatus(request)).await.unwrap_network_status_response().unwrap();

    assert_eq!(1438448, response.current_block_identifier.index);
    assert_eq!(
        "1438448",
        response.current_block_identifier.hash
    );
    assert_eq!(1634052071000, response.current_block_timestamp);
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_blockchain() {
    let request = NetworkStatusRequest {
        network_identifier: NetworkIdentifier {
            blockchain: WRONG_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
            sub_network_identifier: None,
        },
    };

    test_request(Request::NetworkStatus(request)).await.unwrap_network_status_response().unwrap();
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_network() {
    let request = NetworkStatusRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: WRONG_NETWORK.to_string(),
            sub_network_identifier: None,
        },
    };

    test_request(Request::NetworkStatus(request)).await.unwrap_network_status_response().unwrap();
}