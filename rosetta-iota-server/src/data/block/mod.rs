// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::RosettaConfig,
    data::block::block_handler::block,
    filters::{handle, with_rosetta_config},
};

use warp::Filter;

pub mod block_handler;

pub fn routes(options: RosettaConfig) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post().and(
        warp::path!("block")
            .and(warp::body::json())
            .and(with_rosetta_config(options))
            .and_then(handle(block)),
    )
}
