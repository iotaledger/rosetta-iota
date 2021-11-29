// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{config::RosettaMode, error::ApiError, types::NetworkIdentifier};

pub use config::RosettaConfig;

use core::future::Future;
use log::{error, info};
use warp::{http::StatusCode, Filter};

use std::{convert::Infallible, net::SocketAddr};

pub mod client;
pub mod config;
pub mod construction;
pub mod consts;
pub mod data;
pub mod error;
pub mod filters;
pub mod operations;
pub mod types;

pub async fn run_server(config: RosettaConfig, shutdown: impl Future<Output = ()> + Send + 'static) {
    env_logger::init();

    let bind_addr = config
        .bind_addr
        .parse::<SocketAddr>()
        .expect("unable to parse socket address");

    info!("Listening on {}.", bind_addr.to_string());
    info!(
        "BIND_ADDRESS {} NETWORK {} BECH32_HRP {} NODE_URL {} MODE {:#?}",
        bind_addr.to_string(),
        config.network,
        config.bech32_hrp,
        config.node_url,
        config.mode
    );

    let routes = data::network::routes(config.clone())
        .or(data::block::routes(config.clone()))
        .or(data::account::routes(config.clone()))
        .or(construction::routes(config.clone()))
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

pub fn is_wrong_network(options: &RosettaConfig, network_identifier: &NetworkIdentifier) -> bool {
    network_identifier.blockchain != consts::BLOCKCHAIN || network_identifier.network != options.network
}

pub fn is_offline_mode_enabled(options: &RosettaConfig) -> bool {
    options.mode == RosettaMode::Offline
}
