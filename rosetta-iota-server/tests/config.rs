use rosetta_iota_server::RosettaConfig;
use rosetta_iota_server::config::RosettaMode;

pub const DUMMY_NODE_BIND_ADDR: &str = "127.0.0.1:12345";

pub const VALID_NETWORK: &str = "chrysalis-mainnet";
pub const VALID_BLOCKCHAIN: &str = "iota";
pub const VALID_BECH32_ADDRESS_WITH_BALANCE: &str = "iota1qp6gwwy7rruk0d3j9fqzcxnfrstfedk2m65jst2tx7xmkad4agjc5r7ptjz";

pub const WRONG_NETWORK: &str = "xyz";
pub const WRONG_BLOCKCHAIN: &str = "ethereum";
pub const WRONG_ADDRESS_FORMAT: &str = "abc";

pub fn default_rosetta_config() -> RosettaConfig {
    RosettaConfig {
        node_url: format!("http://{}", DUMMY_NODE_BIND_ADDR),
        network: VALID_NETWORK.to_string(),
        tx_tag: "rosetta".to_string(),
        bech32_hrp: "iota".to_string(),
        mode: RosettaMode::Online,
        bind_addr: "0.0.0.0:3030".to_string(),
    }
}