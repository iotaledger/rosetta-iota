use crate::{
    config::{VALID_BLOCKCHAIN, VALID_NETWORK},
    test_request, Request,
};

use rosetta_iota_server::{
    data::block::block_handler::BlockRequest,
    types::{NetworkIdentifier, PartialBlockIdentifier},
};

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    let request = BlockRequest {
        network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        },
        block_identifier: PartialBlockIdentifier {
            index: Some(1438448),
            hash: None,
        },
    };

    // TODO
}
