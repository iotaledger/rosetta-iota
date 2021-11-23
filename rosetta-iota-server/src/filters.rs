// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{config::RosettaConfig, error::ApiError};

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use warp::Filter;

use std::{convert::Infallible, future::Future};

pub fn with_rosetta_config(
    options: RosettaConfig,
) -> impl Filter<Extract = (RosettaConfig,), Error = Infallible> + Clone {
    warp::any().map(move || options.clone())
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EmptyRequest;

pub fn with_empty_request() -> impl Filter<Extract = (EmptyRequest,), Error = Infallible> + Clone {
    warp::any().map(move || EmptyRequest)
}

pub fn handle<'a, F, R, Req, Resp>(
    handler: F,
) -> impl Fn(Req, RosettaConfig) -> BoxFuture<'static, Result<warp::reply::WithStatus<warp::reply::Json>, Infallible>> + Clone
where
    F: FnOnce(Req, RosettaConfig) -> R + Clone + Copy + Send + 'static,
    R: Future<Output = Result<Resp, ApiError>> + Send,
    Req: Deserialize<'a> + Send + 'static,
    Resp: Serialize,
{
    move |request, options| {
        let fut = async move {
            match handler(request, options).await {
                Ok(response) => Ok(warp::reply::with_status(
                    warp::reply::json(&response),
                    warp::http::StatusCode::OK,
                )),
                Err(api_error) => {
                    let status = api_error.status_code();
                    Ok(warp::reply::with_status(
                        warp::reply::json(&api_error.into_error()),
                        status,
                    ))
                }
            }
        };
        Box::pin(fut)
    }
}
