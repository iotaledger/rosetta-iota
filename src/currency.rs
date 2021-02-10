use crate::types::Currency;

pub fn iota_currency() -> Currency {
    Currency {
        symbol: String::from("IOTA"),
        decimals: 0,
    }
}