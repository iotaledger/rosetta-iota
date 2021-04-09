// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::ApiError, options::RosettaMode, types::NetworkIdentifier};

pub use options::Options;

use iota::Client;

use core::future::Future;
use log::{error, info};
use warp::{http::StatusCode, Filter};

use std::{convert::Infallible, net::SocketAddr};

pub mod construction;
pub mod consts;
pub mod currency;
pub mod data;
pub mod error;
pub mod filters;
pub mod operations;
pub mod options;
pub mod types;

pub async fn run_server(options: Options, shutdown: impl Future<Output = ()> + Send + 'static) {
    env_logger::init();

    let bind_addr = options
        .bind_addr
        .parse::<SocketAddr>()
        .expect("unable to parse socket address");

    info!("Listening on {}.", bind_addr.to_string());

    let routes = data::network::routes(options.clone())
        .or(data::block::routes(options.clone()))
        .or(data::account::routes(options.clone()))
        .or(construction::routes(options.clone()))
        .recover(handle_rejection);

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(bind_addr, shutdown);

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

pub async fn build_iota_client(options: &Options) -> Result<Client, ApiError> {
    let builder = iota::Client::builder()
        .with_network(&options.network)
        .with_node(&options.node)
        .map_err(|_| ApiError::UnableToBuildClient)?;
    Ok(builder.finish().await.map_err(|_| ApiError::UnableToBuildClient)?)
}

pub fn is_wrong_network(options: &Options, network_identifier: &NetworkIdentifier) -> bool {
    if network_identifier.blockchain != consts::BLOCKCHAIN || network_identifier.network != options.network {
        true
    } else {
        false
    }
}

pub fn is_offline_mode_enabled(options: &Options) -> bool {
    if options.mode == RosettaMode::Offline {
        true
    } else {
        false
    }
}
