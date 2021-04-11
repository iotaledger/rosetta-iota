// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    data::account::{balance::account_balance, coins::account_coins},
    filters::{handle, with_options},
    config::Config,
};

use warp::Filter;

mod balance;
mod coins;

pub fn routes(options: Config) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(
            warp::path!("account" / "balance")
                .and(warp::body::json())
                .and(with_options(options.clone()))
                .and_then(handle(account_balance)),
        )
        .or(warp::path!("account" / "coins")
            .and(warp::body::json())
            .and(with_options(options.clone()))
            .and_then(handle(account_coins)))
}
