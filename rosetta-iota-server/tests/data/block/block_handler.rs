use crate::{
    config::{VALID_BLOCKCHAIN, VALID_NETWORK}
};

use rosetta_iota_server::{
    data::block::block_handler::BlockRequest,
    types::{NetworkIdentifier, PartialBlockIdentifier},
};

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    // TODO
}
