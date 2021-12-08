// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::{VALID_BLOCKCHAIN, VALID_NETWORK, WRONG_BLOCKCHAIN, WRONG_NETWORK},
    test_request, Request,
};

use rosetta_iota_server::{data::network::options::NetworkOptionsRequest, types::NetworkIdentifier};

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    let request = NetworkOptionsRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        },
    };

    let response = test_request(Request::NetworkOptions(request))
        .await
        .unwrap_network_options_response()
        .unwrap();

    assert_eq!("1.4.10", response.version.rosetta_version);
    assert_eq!("1.0.5", response.version.node_version);

    assert_eq!("Success", response.allow.operation_statuses[0].status);
    assert!(response.allow.operation_statuses[0].successful);

    assert_eq!("INPUT", response.allow.operation_types[0]);
    assert_eq!("SIG_LOCKED_SINGLE_OUTPUT", response.allow.operation_types[1]);
    assert_eq!("SIG_LOCKED_DUST_ALLOWANCE_OUTPUT", response.allow.operation_types[2]);

    assert_eq!(1, response.allow.errors[0].code);
    assert_eq!("non retriable error", response.allow.errors[0].message);
    assert!(!response.allow.errors[0].retriable);
    assert!(response.allow.errors[0].details.is_none());
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_blockchain() {
    let request = NetworkOptionsRequest {
        network_identifier: NetworkIdentifier {
            blockchain: WRONG_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        },
    };

    test_request(Request::NetworkOptions(request))
        .await
        .unwrap_network_options_response()
        .unwrap();
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_network() {
    let request = NetworkOptionsRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: WRONG_NETWORK.to_string(),
        },
    };

    test_request(Request::NetworkOptions(request))
        .await
        .unwrap_network_options_response()
        .unwrap();
}
