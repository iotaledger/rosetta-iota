use crate::dummy_node::dummy_node::{start_dummy_node};
use crate::{test_request, Request};

use rosetta_iota_server::RosettaConfig;
use rosetta_iota_server::config::RosettaMode;
use rosetta_iota_server::filters::EmptyRequest;
use rosetta_iota_server::data::network::list::network_list;

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    let response = test_request(Request::NetworkList(EmptyRequest)).await.unwrap_network_list_response().unwrap();
    assert_eq!("iota", response.network_identifiers[0].blockchain);
    assert_eq!("chrysalis-mainnet", response.network_identifiers[0].network);
    assert_eq!(false, response.network_identifiers[0].sub_network_identifier.is_some());
}