use crate::dummy_node::dummy_node::start_dummy_node;

use rosetta_iota_server::data::account::balance::{AccountBalanceRequest, AccountBalanceResponse, account_balance};
use rosetta_iota_server::data::account::coins::{AccountCoinsRequest, AccountCoinsResponse, account_coins};
use rosetta_iota_server::error::ApiError;
use rosetta_iota_server::filters::EmptyRequest;
use rosetta_iota_server::data::network::list::{NetworkListResponse, network_list};
use rosetta_iota_server::data::network::options::{NetworkOptionsRequest, NetworkOptionsResponse, network_options};
use rosetta_iota_server::data::network::status::{NetworkStatusRequest, NetworkStatusResponse, network_status};
use rosetta_iota_server::data::block::block::{BlockRequest, BlockResponse, block};

use std::net::SocketAddr;
use config::default_rosetta_config;

mod data;
mod dummy_node;
mod config;

pub enum Request {
    AccountBalance(AccountBalanceRequest),
    AccountCoins(AccountCoinsRequest),
    Block(BlockRequest),
    NetworkList(EmptyRequest),
    NetworkOptions(NetworkOptionsRequest),
    NetworkStatus(NetworkStatusRequest),
}

pub enum Response {
    AccountBalance(Result<AccountBalanceResponse, ApiError>),
    AccountCoins(Result<AccountCoinsResponse, ApiError>),
    Block(Result<BlockResponse, ApiError>),
    NetworkList(Result<NetworkListResponse, ApiError>),
    NetworkOptions(Result<NetworkOptionsResponse, ApiError>),
    NetworkStatus(Result<NetworkStatusResponse, ApiError>),
}

impl Response {
    fn unwrap_account_balance_response(self) -> Result<AccountBalanceResponse, ApiError> {
        if let Response::AccountBalance(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_account_coins_response(self) -> Result<AccountCoinsResponse, ApiError> {
        if let Response::AccountCoins(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_block_response(self) -> Result<BlockResponse, ApiError> {
        if let Response::Block(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_network_list_response(self) -> Result<NetworkListResponse, ApiError> {
        if let Response::NetworkList(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_network_options_response(self) -> Result<NetworkOptionsResponse, ApiError> {
        if let Response::NetworkOptions(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_network_status_response(self) -> Result<NetworkStatusResponse, ApiError> {
        if let Response::NetworkStatus(r) = self { r } else { panic!("can not cast type") }
    }
}

pub async fn test_request(
    request: Request,
) -> Response {
    let rosetta_config = default_rosetta_config();

    let dummy_node = start_dummy_node().await;

    let response = match request {
        Request::AccountBalance(r) => Response::AccountBalance(account_balance(r, rosetta_config).await),
        Request::AccountCoins(r) => Response::AccountCoins(account_coins(r, rosetta_config).await),
        Request::Block(r) => Response::Block(block(r, rosetta_config).await),
        Request::NetworkList(r) => Response::NetworkList(network_list(r, rosetta_config).await),
        Request::NetworkOptions(r) => Response::NetworkOptions(network_options(r, rosetta_config).await),
        Request::NetworkStatus(r) => Response::NetworkStatus(network_status(r, rosetta_config).await),
    };

    dummy_node.shutdown().await;

    response
}