use crate::{
    config::{
        VALID_BECH32_ADDRESS_WITH_BALANCE, VALID_BLOCKCHAIN, VALID_NETWORK, WRONG_ADDRESS_FORMAT, WRONG_BLOCKCHAIN,
        WRONG_NETWORK,
    },
    test_request, Request,
};

use rosetta_iota_server::{
    data::account::coins::*,
    types::{AccountIdentifier, NetworkIdentifier},
};

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    let request = AccountCoinsRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        },
        account_identifier: AccountIdentifier {
            address: VALID_BECH32_ADDRESS_WITH_BALANCE.to_string(),
        },
        include_mempool: false,
    };

    let response = test_request(Request::AccountCoins(request))
        .await
        .unwrap_account_coins_response()
        .unwrap();

    assert_eq!(1438495, response.block_identifier.index);
    assert_eq!("1438495", response.block_identifier.hash);
    assert_eq!(1, response.coins.len());
    assert_eq!("20651169480", response.coins[0].amount.value);
    assert_eq!(
        "d2e2faaf394a5d22045668a55df27c9abe2057c1f2ce319999bed373269b50190000",
        response.coins[0].coin_identifier.identifier
    );
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_blockchain() {
    let request = AccountCoinsRequest {
        network_identifier: NetworkIdentifier {
            blockchain: WRONG_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        },
        account_identifier: AccountIdentifier {
            address: VALID_BECH32_ADDRESS_WITH_BALANCE.to_string(),
        },
        include_mempool: false,
    };

    test_request(Request::AccountCoins(request))
        .await
        .unwrap_account_coins_response()
        .unwrap();
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_network() {
    let request = AccountCoinsRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: WRONG_NETWORK.to_string(),
        },
        account_identifier: AccountIdentifier {
            address: VALID_BECH32_ADDRESS_WITH_BALANCE.to_string(),
        },
        include_mempool: false,
    };

    test_request(Request::AccountCoins(request))
        .await
        .unwrap_account_coins_response()
        .unwrap();
}

#[tokio::test]
#[should_panic]
#[serial]
async fn wrong_address_format() {
    let request = AccountCoinsRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        },
        account_identifier: AccountIdentifier {
            address: WRONG_ADDRESS_FORMAT.to_string(),
        },
        include_mempool: false,
    };

    test_request(Request::AccountCoins(request))
        .await
        .unwrap_account_coins_response()
        .unwrap();
}
