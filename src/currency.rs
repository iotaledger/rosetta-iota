// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::Currency;

pub fn iota_currency() -> Currency {
    Currency {
        symbol: String::from("IOTA"),
        decimals: 0,
    }
}
