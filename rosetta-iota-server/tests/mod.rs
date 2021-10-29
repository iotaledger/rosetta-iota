use crate::dummy_node::dummy_node::start_dummy_node;

use config::default_rosetta_config;

use rosetta_iota_server::data::account::balance::{AccountBalanceRequest, AccountBalanceResponse, account_balance};
use rosetta_iota_server::data::account::coins::{AccountCoinsRequest, AccountCoinsResponse, account_coins};
use rosetta_iota_server::error::ApiError;
use rosetta_iota_server::filters::EmptyRequest;
use rosetta_iota_server::data::network::list::{NetworkListResponse, network_list};
use rosetta_iota_server::data::network::options::{NetworkOptionsRequest, NetworkOptionsResponse, network_options};
use rosetta_iota_server::data::network::status::{NetworkStatusRequest, NetworkStatusResponse, network_status};
use rosetta_iota_server::data::block::block::{BlockRequest, BlockResponse, block};
use rosetta_iota_server::construction::derive::{ConstructionDeriveRequest, ConstructionDeriveResponse, derive};
use rosetta_iota_server::construction::parse::{ConstructionParseRequest, ConstructionParseResponse, parse};
use rosetta_iota_server::construction::preprocess::{ConstructionPreprocessRequest, ConstructionPreprocessResponse, preprocess};
use rosetta_iota_server::construction::metadata::{ConstructionMetadataRequest, ConstructionMetadataResponse, metadata};
use rosetta_iota_server::construction::payloads::{ConstructionPayloadsRequest, ConstructionPayloadsResponse, payloads};
use rosetta_iota_server::construction::combine::{ConstructionCombineRequest, ConstructionCombineResponse, combine};
use rosetta_iota_server::construction::hash::{ConstructionHashRequest, ConstructionHashResponse, hash};
use rosetta_iota_server::construction::submit::{ConstructionSubmitResponse, ConstructionSubmitRequest, submit};

mod data;
mod construction;
mod dummy_node;
mod config;

pub enum Request {
    AccountBalance(AccountBalanceRequest),
    AccountCoins(AccountCoinsRequest),
    Block(BlockRequest),
    NetworkList(EmptyRequest),
    NetworkOptions(NetworkOptionsRequest),
    NetworkStatus(NetworkStatusRequest),
    ConstructionDerive(ConstructionDeriveRequest),
    ConstructionPreprocess(ConstructionPreprocessRequest),
    ConstructionMetadata(ConstructionMetadataRequest),
    ConstructionParse(ConstructionParseRequest),
    ConstructionPayloads(ConstructionPayloadsRequest),
    ConstructionCombine(ConstructionCombineRequest),
    ConstructionHash(ConstructionHashRequest),
    ConstructionSubmit(ConstructionSubmitRequest),
}

pub enum Response {
    AccountBalance(Result<AccountBalanceResponse, ApiError>),
    AccountCoins(Result<AccountCoinsResponse, ApiError>),
    Block(Result<BlockResponse, ApiError>),
    NetworkList(Result<NetworkListResponse, ApiError>),
    NetworkOptions(Result<NetworkOptionsResponse, ApiError>),
    NetworkStatus(Result<NetworkStatusResponse, ApiError>),
    ConstructionDerive(Result<ConstructionDeriveResponse, ApiError>),
    ConstructionPreprocess(Result<ConstructionPreprocessResponse, ApiError>),
    ConstructionMetadata(Result<ConstructionMetadataResponse, ApiError>),
    ConstructionParse(Result<ConstructionParseResponse, ApiError>),
    ConstructionPayloads(Result<ConstructionPayloadsResponse, ApiError>),
    ConstructionCombine(Result<ConstructionCombineResponse, ApiError>),
    ConstructionHash(Result<ConstructionHashResponse, ApiError>),
    ConstructionSubmit(Result<ConstructionSubmitResponse, ApiError>),
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
    fn unwrap_construction_derive_response(self) -> Result<ConstructionDeriveResponse, ApiError> {
        if let Response::ConstructionDerive(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_construction_preprocess_response(self) -> Result<ConstructionPreprocessResponse, ApiError> {
        if let Response::ConstructionPreprocess(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_construction_metadata(self) -> Result<ConstructionMetadataResponse, ApiError> {
        if let Response::ConstructionMetadata(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_construction_parse_response(self) -> Result<ConstructionParseResponse, ApiError> {
        if let Response::ConstructionParse(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_construction_payloads_response(self) -> Result<ConstructionPayloadsResponse, ApiError> {
        if let Response::ConstructionPayloads(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_construction_combine_response(self) -> Result<ConstructionCombineResponse, ApiError> {
        if let Response::ConstructionCombine(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_construction_hash_response(self) -> Result<ConstructionHashResponse, ApiError> {
        if let Response::ConstructionHash(r) = self { r } else { panic!("can not cast type") }
    }
    fn unwrap_construction_submit_response(self) -> Result<ConstructionSubmitResponse, ApiError> {
        if let Response::ConstructionSubmit(r) = self { r } else { panic!("can not cast type") }
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
        Request::ConstructionDerive(r) => Response::ConstructionDerive(derive(r, rosetta_config).await),
        Request::ConstructionPreprocess(r) => Response::ConstructionPreprocess(preprocess(r, rosetta_config).await),
        Request::ConstructionMetadata(r) => Response::ConstructionMetadata(metadata(r, rosetta_config).await),
        Request::ConstructionParse(r) => Response::ConstructionParse(parse(r, rosetta_config).await),
        Request::ConstructionPayloads(r) => Response::ConstructionPayloads(payloads(r, rosetta_config).await),
        Request::ConstructionCombine(r) => Response::ConstructionCombine(combine(r, rosetta_config).await),
        Request::ConstructionHash(r) => Response::ConstructionHash(hash(r, rosetta_config).await),
        Request::ConstructionSubmit(r) => Response::ConstructionSubmit(submit(r, rosetta_config).await),
    };

    dummy_node.shutdown().await;

    response
}