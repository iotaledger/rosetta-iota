// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{test_request, Request};

use rosetta_iota_server::filters::EmptyRequest;

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    let response = test_request(Request::NetworkList(EmptyRequest))
        .await
        .unwrap_network_list_response()
        .unwrap();
    assert_eq!("iota", response.network_identifiers[0].blockchain);
    assert_eq!("chrysalis-mainnet", response.network_identifiers[0].network);
}
