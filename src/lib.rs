// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::ApiError;
use core::future::Future;
use log::{error, info};
pub use options::Options;
use std::{convert::Infallible, net::SocketAddr};
use warp::{http::StatusCode, Filter};

mod account;
mod block;
mod construction;
mod consts;
mod currency;
mod error;
mod filters;
mod network;
mod operations;
mod options;
mod types;

pub async fn run_server(
    binding_addr: SocketAddr,
    options: Options,
    shutdown: impl Future<Output = ()> + Send + 'static,
) {
    env_logger::init();

    info!("Listening on {}.", binding_addr.to_string());

    let routes = network::routes(options.clone())
        .or(block::routes(options.clone()))
        // .or(account::routes(options.clone()))
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
