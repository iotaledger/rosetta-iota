// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::Currency;

pub const BLOCKCHAIN: &str = "iota";
pub const ROSETTA_VERSION: &str = "1.4.10";
pub const NODE_VERSION: &str = "1.0.5";

pub const DUST_THRESHOLD: u64 = 1_000_000;

pub fn iota_currency() -> Currency {
    Currency {
        symbol: String::from("IOTA"),
        decimals: 0,
    }
}
