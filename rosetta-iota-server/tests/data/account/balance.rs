// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::{
        VALID_BECH32_ADDRESS_WITH_BALANCE, VALID_BLOCKCHAIN, VALID_NETWORK, WRONG_ADDRESS_FORMAT, WRONG_BLOCKCHAIN,
        WRONG_NETWORK,
    },
    test_request, Request,
};

use rosetta_iota_server::{
    data::account::balance::*,
    types::{AccountIdentifier, NetworkIdentifier},
};

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    let request = AccountBalanceRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        },
        account_identifier: AccountIdentifier {
            address: VALID_BECH32_ADDRESS_WITH_BALANCE.to_string(),
        },
        currencies: None,
    };

    let response = test_request(Request::AccountBalance(request))
        .await
        .unwrap_account_balance_response()
        .unwrap();

    assert_eq!(1438441, response.block_identifier.index);
    assert_eq!("1438441", response.block_identifier.hash);
    assert_eq!(1, response.balances.len());
    assert_eq!("IOTA", response.balances[0].currency.symbol);
    assert_eq!(0, response.balances[0].currency.decimals);
    assert_eq!("20651169480", response.balances[0].value);
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_blockchain() {
    let request = AccountBalanceRequest {
        network_identifier: NetworkIdentifier {
            blockchain: WRONG_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        },
        account_identifier: AccountIdentifier {
            address: VALID_BECH32_ADDRESS_WITH_BALANCE.to_string(),
        },
        currencies: None,
    };

    test_request(Request::AccountBalance(request))
        .await
        .unwrap_account_balance_response()
        .unwrap();
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_network() {
    let request = AccountBalanceRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: WRONG_NETWORK.to_string(),
        },
        account_identifier: AccountIdentifier {
            address: VALID_BECH32_ADDRESS_WITH_BALANCE.to_string(),
        },
        currencies: None,
    };

    test_request(Request::AccountBalance(request))
        .await
        .unwrap_account_balance_response()
        .unwrap();
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_address_format() {
    let request = AccountBalanceRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        },
        account_identifier: AccountIdentifier {
            address: WRONG_ADDRESS_FORMAT.to_string(),
        },
        currencies: None,
    };

    test_request(Request::AccountBalance(request))
        .await
        .unwrap_account_balance_response()
        .unwrap();
}
