// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{test_request, Request};

use rosetta_iota_server::construction::derive::ConstructionDeriveRequest;

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    let request: ConstructionDeriveRequest = serde_json::from_str(
        r#"
        {
           "network_identifier":{
              "blockchain":"iota",
              "network":"chrysalis-mainnet"
           },
           "public_key":{
              "hex_bytes":"6f8f4d77e94bce3900078b89319e6e25b341d47669a76ae4bf26677d377533f0",
              "curve_type":"edwards25519"
           }
        }
    "#,
    )
    .unwrap();

    let response = test_request(Request::ConstructionDerive(request))
        .await
        .unwrap_construction_derive_response()
        .unwrap();

    assert_eq!(
        "iota1qpv2nr99fkjykh5ga3x62lqlztrg6t67k750v93lculsna3z7knnuzqur06",
        response.account_identifier.address
    )
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_blockchain() {
    let request: ConstructionDeriveRequest = serde_json::from_str(
        r#"
        {
           "network_identifier":{
              "blockchain":"xyz",
              "network":"chrysalis-mainnet"
           },
           "public_key":{
              "hex_bytes":"6f8f4d77e94bce3900078b89319e6e25b341d47669a76ae4bf26677d377533f0",
              "curve_type":"edwards25519"
           }
        }
    "#,
    )
    .unwrap();

    test_request(Request::ConstructionDerive(request))
        .await
        .unwrap_construction_derive_response()
        .unwrap();
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_network() {
    let request: ConstructionDeriveRequest = serde_json::from_str(
        r#"
        {
           "network_identifier":{
              "blockchain":"iota",
              "network":"xyz"
           },
           "public_key":{
              "hex_bytes":"6f8f4d77e94bce3900078b89319e6e25b341d47669a76ae4bf26677d377533f0",
              "curve_type":"edwards25519"
           }
        }
    "#,
    )
    .unwrap();

    test_request(Request::ConstructionDerive(request))
        .await
        .unwrap_construction_derive_response()
        .unwrap();
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_hex_bytes() {
    let request: ConstructionDeriveRequest = serde_json::from_str(
        r#"
        {
           "network_identifier":{
              "blockchain":"iota",
              "network":"chrysalis-mainnet"
           },
           "public_key":{
              "hex_bytes":"abcd",
              "curve_type":"edwards25519"
           }
        }
    "#,
    )
    .unwrap();

    test_request(Request::ConstructionDerive(request))
        .await
        .unwrap_construction_derive_response()
        .unwrap();
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_curve() {
    let request: ConstructionDeriveRequest = serde_json::from_str(
        r#"
        {
           "network_identifier":{
              "blockchain":"iota",
              "network":"chrysalis-mainnet"
           },
           "public_key":{
              "hex_bytes":"6f8f4d77e94bce3900078b89319e6e25b341d47669a76ae4bf26677d377533f0",
              "curve_type":"xyz"
           }
        }
    "#,
    )
    .unwrap();

    test_request(Request::ConstructionDerive(request))
        .await
        .unwrap_construction_derive_response()
        .unwrap();
}
