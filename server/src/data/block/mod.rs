// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    data::block::block::block,
    filters::{handle, with_options},
    config::Config,
};

use warp::Filter;

mod block;

pub fn routes(options: Config) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post().and(
        warp::path!("block")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(block)),
    )
}
