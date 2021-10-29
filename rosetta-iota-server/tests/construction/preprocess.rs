use crate::{test_request, Request};

use rosetta_iota_server::construction::preprocess::ConstructionPreprocessRequest;

use serial_test::serial;

#[tokio::test]
#[serial]
async fn valid_request() {
    let request: ConstructionPreprocessRequest = serde_json::from_str(r#"
        {
           "network_identifier":{
              "blockchain":"iota",
              "network":"chrysalis-mainnet"
           },
           "operations":[
              {
                 "operation_identifier":{
                    "index":0,
                    "network_index":0
                 },
                 "type":"INPUT",
                 "account":{
                    "address":"atoi1qr49znuapruu3fhwcfd4vsq2y3a0l9k8zc6pv6ak70g4hd9jq8fr2lqf6et"
                 },
                 "amount":{
                    "value":"-10000000",
                    "currency":{
                       "symbol":"IOTA",
                       "decimals":0
                    }
                 },
                 "coin_change":{
                    "coin_identifier":{
                       "identifier":"8bec7fd0a9fdc351adaaf07f595afefa7844eafd183625949e51dcb3b9632b890000"
                    },
                    "coin_action":"coin_spent"
                 }
              },
              {
                 "operation_identifier":{
                    "index":1
                 },
                 "type":"SIG_LOCKED_SINGLE_OUTPUT",
                 "account":{
                    "address":"atoi1qpmppfmvwlg5qjkwd8084ceh0emw6y9gegpmesn2vvrlacfep834wyqsxww"
                 },
                 "amount":{
                    "value":"8604736",
                    "currency":{
                       "symbol":"IOTA",
                       "decimals":0
                    }
                 }
              },
              {
                 "operation_identifier":{
                    "index":2
                 },
                 "type":"SIG_LOCKED_SINGLE_OUTPUT",
                 "account":{
                    "address":"atoi1qp08ypmqn53kxxmj7d60wqp6hwtcc25sv8y950j7e35fjnj3dmpxyp7l5y9"
                 },
                 "amount":{
                    "value":"395264",
                    "currency":{
                       "symbol":"IOTA",
                       "decimals":0
                    }
                 }
              },
              {
                 "operation_identifier":{
                    "index":3
                 },
                 "type":"DUST_ALLOWANCE_OUTPUT",
                 "account":{
                    "address":"atoi1qp08ypmqn53kxxmj7d60wqp6hwtcc25sv8y950j7e35fjnj3dmpxyp7l5y9"
                 },
                 "amount":{
                    "value":"1000000",
                    "currency":{
                       "symbol":"IOTA",
                       "decimals":0
                    }
                 }
              }
           ]
        }
    "#).unwrap();

    let response = test_request(Request::ConstructionPreprocess(request))
        .await
        .unwrap_construction_preprocess_response()
        .unwrap();

    assert_eq!(1, response.options.utxo_inputs.len());

    assert_eq!(
        "8bec7fd0a9fdc351adaaf07f595afefa7844eafd183625949e51dcb3b9632b890000",
        response.options.utxo_inputs[0]
    )
}
