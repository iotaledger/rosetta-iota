use crate::dummy_node::dummy_node::{start_dummy_node};

use rosetta_iota_server::types::{NetworkIdentifier, PartialBlockIdentifier};
use rosetta_iota_server::RosettaConfig;
use rosetta_iota_server::config::RosettaMode;

use serial_test::serial;
use rosetta_iota_server::data::block::block::{BlockRequest, block};
use crate::config::{VALID_BLOCKCHAIN, VALID_NETWORK};
use crate::{test_request, Request};

#[tokio::test]
#[serial]
async fn valid_request() {
    let request = BlockRequest { network_identifier: NetworkIdentifier {
            blockchain: VALID_BLOCKCHAIN.to_string(),
            network: VALID_NETWORK.to_string(),
            sub_network_identifier: None,
        }, block_identifier: PartialBlockIdentifier {
            index: Some(1438448),
            hash: None,
        }, };

    let response = test_request(Request::Block(request)).await.unwrap_block_response().unwrap();
}