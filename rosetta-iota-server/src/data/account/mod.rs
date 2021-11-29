// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::RosettaConfig,
    data::account::{balance::account_balance, coins::account_coins},
    filters::{handle, with_rosetta_config},
};

use warp::Filter;

pub mod balance;
pub mod coins;

pub fn routes(options: RosettaConfig) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(
            warp::path!("account" / "balance")
                .and(warp::body::json())
                .and(with_rosetta_config(options.clone()))
                .and_then(handle(account_balance)),
        )
        .or(warp::path!("account" / "coins")
            .and(warp::body::json())
            .and(with_rosetta_config(options))
            .and_then(handle(account_coins)))
}
