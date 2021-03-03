
use bee_message::prelude::*;
use bee_common::packable::Packable;
use iota::Client;

use rosetta_iota::Options;
use rosetta_iota::consts::OFFLINE_MODE;
use rosetta_iota::types::*;
use rosetta_iota::construction::derive::construction_derive_request;

const DEFAULT_NODE_URL: &str = "https://api.lb-0.testnet.chrysalis2.com";

#[tokio::test]
async fn test_transfer_funds() {

    /// USE FOLLOWING KEYS ONLY FOR TESTING PURPOSES
    let secret_key = "7e828a3c369f1d963685aae2354ab7f3509bed9e6244a7d4c370daccb37ca606";
    let public_key = "82eeba00688da228b83bbe32d6c2e2d548550ab3c6e30752d9fe2617e89f554d";

    /// 1) Derive the address from the public key
    let derive_response: ConstructionDeriveResponse = {
        let request = ConstructionDeriveRequest {
            network_identifier: NetworkIdentifier {
                blockchain: "iota".to_string(),
                network: "testnet6".to_string(),
                sub_network_identifier: None
            },
            public_key: PublicKey { hex_bytes: public_key.to_string(), curve_type: CurveType::Edwards25519 }
        };
        let rosetta_options = Options {
            iota_endpoint: DEFAULT_NODE_URL.to_string(),
            network: "testnet6".to_string(),
            mode: OFFLINE_MODE.into(),
            port: 3030
        };
        construction_derive_request(request, rosetta_options).await.expect("derive request failed")
    };

    // 2) Build the operations that specify inputs and outputs of the transaction
    let operations = {

        // ...

    };

}