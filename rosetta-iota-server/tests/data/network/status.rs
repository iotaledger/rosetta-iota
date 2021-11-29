// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::{VALID_BLOCKCHAIN, VALID_NETWORK, WRONG_BLOCKCHAIN, WRONG_NETWORK},
    test_request, Request,
};

use rosetta_iota_server::{data::network::status::NetworkStatusRequest, types::NetworkIdentifier};

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    let request = NetworkStatusRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        },
    };

    let response = test_request(Request::NetworkStatus(request))
        .await
        .unwrap_network_status_response()
        .unwrap();

    assert_eq!(1438448, response.current_block_identifier.index);
    assert_eq!("1438448", response.current_block_identifier.hash);
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
        },
    };

    test_request(Request::NetworkStatus(request))
        .await
        .unwrap_network_status_response()
        .unwrap();
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_network() {
    let request = NetworkStatusRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: WRONG_NETWORK.to_string(),
        },
    };

    test_request(Request::NetworkStatus(request))
        .await
        .unwrap_network_status_response()
        .unwrap();
}
