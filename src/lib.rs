// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::ApiError;
use core::future::Future;
use log::{error, info};
pub use options::Options;
use std::{convert::Infallible, net::SocketAddr};
use warp::{http::StatusCode, Filter};
use crate::types::NetworkIdentifier;
use iota::Client;

pub mod account;
pub mod block;
pub mod construction;
pub mod consts;
pub mod currency;
pub mod error;
pub mod filters;
pub mod network;
pub mod operations;
pub mod options;
pub mod types;

pub async fn run_server(
    binding_addr: SocketAddr,
    options: Options,
    shutdown: impl Future<Output = ()> + Send + 'static,
) {
    env_logger::init();

    info!("Listening on {}.", binding_addr.to_string());

    let routes = network::routes(options.clone())
        .or(block::routes(options.clone()))
        .or(account::routes(options.clone()))
        .or(construction::routes(options.clone()))
        .recover(handle_rejection);

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(binding_addr, shutdown);

    server.await;

    info!("Stopped.");
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let message;
    let status;
    let code;
    let retriable;
    let details;

    if err.is_not_found() {
        message = "resource not found".to_string();
        status = StatusCode::NOT_FOUND;
        code = 0;
        retriable = false;
        details = None;
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        message = e.to_string();
        retriable = false;
        status = StatusCode::BAD_REQUEST;
        code = 0;
        details = None;
    } else if let Some(api_error) = err.find::<ApiError>() {
        message = api_error.message();
        retriable = api_error.retriable();
        status = api_error.status_code();
        code = api_error.code();
        details = Some(api_error.details());
    } else {
        error!("unexpected internal error: {:?}", err);
        message = "internal server error".to_string();
        code = 1;
        retriable = true;
        status = StatusCode::INTERNAL_SERVER_ERROR;
        details = None;
    }

    let error = types::Error {
        code,
        message,
        retriable,
        details,
    };
    let json = warp::reply::json(&error);

    Ok(warp::reply::with_status(json, status))
}

pub async fn build_iota_client(
    options: &Options,
    is_node_sync_required: bool,
) -> Result<Client, ApiError> {
    let mut builder = iota::Client::builder()
        .with_network(&options.network)
        .with_node(&options.iota_endpoint)
        .map_err(|_| ApiError::UnableToBuildClient)?;
    if is_node_sync_required {
        builder = builder.with_node_sync_disabled();
    }
    Ok(builder.finish()
        .await
        .map_err(|_| ApiError::UnableToBuildClient)?)
}

pub fn is_bad_network(
    options: &Options,
    network_identifier: &NetworkIdentifier,
) -> Result<(), ApiError> {
    if network_identifier.blockchain != consts::BLOCKCHAIN
        || network_identifier.network != options.network
    {
        return Err(ApiError::BadNetwork);
    }
    Ok(())
}

pub fn require_online_mode(
    options: &Options
) -> Result<(), ApiError> {
    if options.mode == consts::ONLINE_MODE {
        Ok(())
    } else {
        return Err(ApiError::UnavailableOffline);
    }
}

pub fn require_offline_mode(
    options: &Options
) -> Result<(), ApiError> {
    if options.mode == consts::OFFLINE_MODE {
        Ok(())
    } else {
        return Err(ApiError::UnavailableOnline);
    }
}