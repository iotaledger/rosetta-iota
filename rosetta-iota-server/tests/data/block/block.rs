use crate::config::{VALID_BLOCKCHAIN, VALID_NETWORK};
use crate::{test_request, Request};

use rosetta_iota_server::types::{NetworkIdentifier, PartialBlockIdentifier};
use rosetta_iota_server::data::block::block_handler::{BlockRequest};

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    let request = BlockRequest { network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
        }, block_identifier: PartialBlockIdentifier {
            index: Some(1438448),
            hash: None,
        }, };

    let response = test_request(Request::Block(request)).await.unwrap_block_response().unwrap();
}