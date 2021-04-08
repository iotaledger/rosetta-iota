// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    data::network::{list::network_list, options::network_options, status::network_status},
    filters::{handle, with_empty_request, with_options},
    Options,
};

use warp::Filter;

mod list;
mod options;
mod status;

pub fn routes(options: Options) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(
            warp::path!("network" / "list")
                .and(with_empty_request())
                .and(with_options(options.clone()))
                .and_then(handle(network_list)),
        )
        .or(warp::path!("network" / "options")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(network_options)))
        .or(warp::path!("network" / "status")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(network_status)))
}
