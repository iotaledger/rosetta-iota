# rosetta-iota
IOTA Rosetta API Implementation

## IOTA protocol
The IOTA protocol differs from most DLT technologies in the sense that it is not a Blockchain, but a Directed Acyclic Graph (DAG), popularly known as [the Tangle](https://assets.ctfassets.net/r1dr6vzfxhev/2t4uxvsIqk0EUau6g2sw0g/45eae33637ca92f85dd9f4a3a218e1ec/iota1_4_3.pdf).

For that reason, a few concepts have been adapted in relation to the Rosetta API.

### Messages and Transactions
IOTA uses messages as a envelope around a payload that can consist of data (indexation), value (transactions) or a combination of both (a indexation payload embedded in a transaction). Transactions use a utxo-based model for transfering value.

### Blocks and Milestones
Being a DAG, IOTA has the concept of Milestones as the closest analogy for Blocks. Periodically, nodes settle the current ledger state by creating a special message that defines a new Milestone.

IOTA Fullnodes (such as [HORNET](https://github.com/gohornet/hornet.git) and [BEE](https://github.com/iotaledger/bee.git)) don't contain the entire history of the Tangle. They have a parameter called `pruningIndex` which represents the oldest Milestone available on the node.

Only Permanodes (such as [Chronicle](https://github.com/iotaledger/chronicle.rs/tree/main/chronicle-node)) have the ability of holding the entire Tangle history.

## IOTA and Rosetta

Here we describe how concepts from the IOTA protocol were adapted for the Rosetta Endpoints.

### Genesis Milestone

The genesis milestone is not available on Fullnodes, but the `/network/status` endpoint response contains a `genesis_block_identifier` field. Therefore, whenever a Fullnode is used by the `rosetta-iota` server, the `genesis_block_identifier` is populated as such:
```
"genesis_block_identifier": {
  "index": 1,
  "hash": "0000000000000000000000000000000000000000000000000000000000000000"
}
```

Technically speaking, that is not 100% accurate, as the genesis milestone identifier is not a series of `0`s. However, a dummy value is used as a compromise in order not to render the `/network/status` endpoint unavailable. 

Nevertheless, the correct value is returned when a Permanode with full Tangle history is used.

Syncing `rosetta-cli` is also affected by this. Syncing from genesis is only possible with a Permanode, as well. Otherwise, the `test.sh` shows how to use the Fullnode's `pruningIndex` as the `start_index` for the configuration `JSON`.

### Transactions and Operations
The `/block` endpoint responds with information about balance changing Transactions that happened on a specific Milestone.
That is achieved via the `/api/v1/milestones/:milestoneId/utxo-changes` IOTA Fullnode endpoint, where a list of **Created** and **Consumed** UTXO Outputs is returned.

Each UTXO Output contains a `output_id`, a `transaction_id` and a `output_index`, where `output_id = transaction_id + output_index`.

In terms of Rosetta Models, the `TransactionIdentifier` of a `Transaction` Object is defined by an IOTA `transaction_id`.

Each [Transaction](https://www.rosetta-api.org/docs/models/Transaction.html) Object has an array of [Operations](https://www.rosetta-api.org/docs/models/Operation.html), each one representing a UTXO Output.

The `UTXO Operation` `operation_identifier` Object ([OperationIdentifier](https://www.rosetta-api.org/docs/models/OperationIdentifier.html) type) is defined as:
* `index`: incremented from `0` for each `Operation` Object in the `Transaction`
* `network_index`: `output_index`

The `UTXO Operation` `type` field can be either:
 * `"UTXO_INPUT"`, which describes the funds to spend
 * `"UTXO_OUTPUT"`, which describes where the funds should be transfered to

The `UTXO Operation` `status` field is defined as:
* `"SUCCESS"`, meaning that the transaction was included in the ledger.
* `"SKIPPED"`, meaning that the UTXO Output has not been included in the ledger (e.g. already spent by another Transaction).

Note, that the status field will not get populated when constructing a transaction by the Construction API.

The `UTXO Operation` `account` field is defined as the `address` value from the UTXO.

The `UTXO Operation` `coin_change` Object ([CoinChange](https://www.rosetta-api.org/docs/models/CoinChange.html) type) is defined as:
* `coin_identifier`: `output_id`
* `coin_action`: either
    - `"coin_spent"`, where coins are coming from into the Transaction
    - `"coin_created"`, where coins are going out from the Transaction

**Note:** Rosetta's definition of [CoinAction](https://www.rosetta-api.org/docs/models/CoinAction.html) is an `enum` valued with `"coin_spent"` and `"coin_created"`. These terms are analogous to IOTA's `"UTXO_CONSUMED"` and `"UTXO_CREATED"`, and **must not be confused** with IOTA's `"UTXO_SPENT"` and `"UTXO_UNSPENT"`.

Here's an example of two Transaction Objects in the same Milestone:
```
{
   "transactions":[
      {
         "transaction_identifier":{
            "hash":"9ce415875aa9ed67c4a3b97cb598861be11444e880fad604ec2a96ee65590da8"
         },
         "operations":[
            {
               "operation_identifier":{
                  "index":0,
                  "network_index":0
               },
               "type":"UTXO_OUTPUT",
               "status":"SUCCESS",
               "account":{
                  "address":"atoi1q8k69lxuxljdgeqt7tucvtdfk3hrvrly7rzz65w57te6drf3expsjth4u2j"
               },
               "amount":{
                  "value":"999880000000",
                  "currency":{
                     "symbol":"IOTA",
                     "decimals":0
                  }
               },
               "coin_change":{
                  "coin_identifier":{
                     "identifier":"9ce415875aa9ed67c4a3b97cb598861be11444e880fad604ec2a96ee65590da80000"
                  },
                  "coin_action":"coin_created"
               },
               "metadata":{
                  "is_spent":"UTXO_SPENT"
               }
            }
         ]
      },
      {
         "transaction_identifier":{
            "hash":"586d11477a48a25b4a554686ce8d6cf711be3a85cf07a0dc2d07d4e0f4c03636"
         },
         "operations":[
            {
               "operation_identifier":{
                  "index":0,
                  "network_index":0
               },
               "type":"UTXO_INPUT",
               "status":"SUCCESS",
               "account":{
                  "address":"atoi1q8k69lxuxljdgeqt7tucvtdfk3hrvrly7rzz65w57te6drf3expsjth4u2j"
               },
               "amount":{
                  "value":"999870000000",
                  "currency":{
                     "symbol":"IOTA",
                     "decimals":0
                  }
               },
               "coin_change":{
                  "coin_identifier":{
                     "identifier":"586d11477a48a25b4a554686ce8d6cf711be3a85cf07a0dc2d07d4e0f4c036360000"
                  },
                  "coin_action":"coin_spent"
               },
               "metadata":{
                  "is_spent":"UTXO_UNSPENT"
               }
            },
            {
               "operation_identifier":{
                  "index":1,
                  "network_index":1
               },
               "type":"UTXO_INPUT",
               "status":"SUCCESS",
               "account":{
                  "address":"atoi1q86v9cgc8d9ue9nd9k4z8rp2dn29mcwf2y9jhm59u8dg25aukhq3jqwwfvf"
               },
               "amount":{
                  "value":"10000000",
                  "currency":{
                     "symbol":"IOTA",
                     "decimals":0
                  }
               },
               "coin_change":{
                  "coin_identifier":{
                     "identifier":"586d11477a48a25b4a554686ce8d6cf711be3a85cf07a0dc2d07d4e0f4c036360100"
                  },
                  "coin_action":"coin_spent"
               },
               "metadata":{
                  "is_spent":"UTXO_UNSPENT"
               }
            }
         ]
      }
   ]
}
```

## Testing

### rosetta-cli

The `test.sh` shell script automates testing via `rosetta-cli`.

It will use the shell variables `NODE_URL` and `NETWORK` to specify the network parameters to be tested.
Bear in mind that syncing will start from the pruned Milestone, not Genesis.

### curl

Curl commands can also be used for manual inspection of each API endpoint.

1. Kickstart a `rosetta-iota` server:
```
$ cargo run -- --iota-endpoint http://0.0.0.0:14265 --network testnet5 --port 3030 --mode online
```

2. From a new terminal, you can test each endpoint via `curl`:

- `/network/list`
```
$ curl --request POST 'http://localhost:3030/network/list' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"metadata":{}}' | jq
```

- `/network/status`
```
$ curl --request POST 'http://localhost:3030/network/status' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"}}' | jq
```

- `/network/options`
```
$ curl --request POST 'http://localhost:3030/network/options' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"}}' | jq
```

- `/block`
```
$ curl --request POST 'http://localhost:3030/block' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"},"block_identifier":{"index":2,"hash":""}}' | jq
```

- `/account/balance`
```
$ curl --request POST 'http://localhost:3030/account/balance' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"},"account_identifier":{"address":"atoi1qx0pteshrd554xtea4v3rklr97kzgc95umcpckn9pl897gnedk7gugyk5ld"}}' | jq
```

- `/account/coins`
```
$ curl --request POST 'http://localhost:3030/account/coins' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"},"account_identifier":{"address":"atoi1qx0pteshrd554xtea4v3rklr97kzgc95umcpckn9pl897gnedk7gugyk5ld"}}' | jq
```

- `/construction/derive`
```
$ curl --request POST 'http://localhost:3030/construction/derive' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"},"public_key":{"hex_bytes":"6f8f4d77e94bce3900078b89319e6e25b341d47669a76ae4bf26677d377533f0","curve_type":"edwards25519"}}' | jq
```

- `/construction/metadata`
```
$ curl --request POST 'http://localhost:3030/construction/metadata' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"}}' | jq
```

- `/construction/preprocess`
```
$ curl --request POST 'http://localhost:3030/construction/preprocess' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"},"operations":[{"operation_identifier":{"index":0,"network_index":1},"related_operations":[],"type":"UTXO_INPUT","status":"SUCCESS","account":{"address":"atoi1qz4u7jdqn6d2qjt5y7tauctckl77nt6qv69reuhm7pdc98weu6cl2qylcgr"},"amount":{"value":"999820000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"837f9646c7cc5e748099d3abb946302e44927c96c648bcc3dd0f693258a61e1b0100"},"coin_action":"coin_spent"},"metadata":{"is_spent":"UTXO_UNSPENT"}},{"operation_identifier":{"index":0,"network_index":1},"related_operations":[{"index":1}],"type":"UTXO_OUTPUT","status":"SUCCESS","account":{"address":"atoi1qzcts3mukxthlwg90u2e5nrhpqt566fghhxdga83grmy60kt2akx2q0de5u"},"amount":{"value":"10000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"331bfc6eb2a2e02d7b7b9ce7aede370d8b4f6db3236887c5615ae36c523b2f060100"},"coin_action":"coin_created"},"metadata":{"is_spent":"UTXO_UNSPENT"}},{"operation_identifier":{"index":1,"network_index":0},"related_operations":[{"index":0}],"type":"UTXO_OUTPUT","status":"SUCCESS","account":{"address":"atoi1qz4u7jdqn6d2qjt5y7tauctckl77nt6qv69reuhm7pdc98weu6cl2qylcgr"},"amount":{"value":"999810000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"331bfc6eb2a2e02d7b7b9ce7aede370d8b4f6db3236887c5615ae36c523b2f060000"},"coin_action":"coin_created"},"metadata":{"is_spent":"UTXO_UNSPENT"}}],"metadata":{},"public_keys":[{"hex_bytes":"3ada770dca2c802df6837df29bdd8cd6c6d9d72cc4041743c7942cc11e6834bd","curve_type":"edwards25519"}]}' | jq
```

- `/construction/payloads`
```
$ curl --request POST 'http://localhost:3030/construction/payloads' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"},"operations":[{"operation_identifier":{"index":0,"network_index":1},"related_operations":[],"type":"UTXO_INPUT","status":"SUCCESS","account":{"address":"atoi1qz4u7jdqn6d2qjt5y7tauctckl77nt6qv69reuhm7pdc98weu6cl2qylcgr"},"amount":{"value":"999820000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"837f9646c7cc5e748099d3abb946302e44927c96c648bcc3dd0f693258a61e1b0100"},"coin_action":"coin_spent"},"metadata":{"is_spent":"UTXO_UNSPENT"}},{"operation_identifier":{"index":0,"network_index":1},"related_operations":[{"index":1}],"type":"UTXO_OUTPUT","status":"SUCCESS","account":{"address":"atoi1qzcts3mukxthlwg90u2e5nrhpqt566fghhxdga83grmy60kt2akx2q0de5u"},"amount":{"value":"10000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"331bfc6eb2a2e02d7b7b9ce7aede370d8b4f6db3236887c5615ae36c523b2f060100"},"coin_action":"coin_created"},"metadata":{"is_spent":"UTXO_UNSPENT"}},{"operation_identifier":{"index":1,"network_index":0},"related_operations":[{"index":0}],"type":"UTXO_OUTPUT","status":"SUCCESS","account":{"address":"atoi1qz4u7jdqn6d2qjt5y7tauctckl77nt6qv69reuhm7pdc98weu6cl2qylcgr"},"amount":{"value":"999810000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"331bfc6eb2a2e02d7b7b9ce7aede370d8b4f6db3236887c5615ae36c523b2f060000"},"coin_action":"coin_created"},"metadata":{"is_spent":"UTXO_UNSPENT"}}]}' | jq
```

- `/construction/parse`
```
$ curl --request POST 'http://localhost:3030/construction/parse' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"},"signed":false,"transaction":"00010000837f9646c7cc5e748099d3abb946302e44927c96c648bcc3dd0f693258a61e1b010002000000abcf49a09e9aa049742797de6178b7fde9af40668a3cf2fbf05b829dd9e6b1f580e451c9e80000000000b0b8477cb1977fb9057f159a4c7708174d6928bdccd474f140f64d3ecb576c65809698000000000000000000"}' | jq
```

- `/construction/combine`
```
$ curl --request POST 'http://localhost:3030/construction/combine' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet5"},"unsigned_transaction":"0001000063aa5b2476105ad4ebbb995968216cef0d940791d900afd1180a8df4e527b3460000010000005eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8050000000000000000000000","signatures":[{"signing_payload":{"account_identifier":{"address":"ae98475c63cfebc918b57193a4183f4374f67974971aff9034699793d331d7de"},"hex_bytes":"c8be532dd8f351c28b6cdb7903d076b356129189d11ad50c46feb25ca472b984","signature_type":"edwards25519"},"public_key":{"hex_bytes":"b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde","curve_type":"edwards25519"},"signature_type":"edwards25519","hex_bytes":"59057634a166d84edebb88ab17a33394c0ccf1d46adffc3a0095c0f5e7e9dbe766ab05cb0244486d272e61f1e09db1bdb6e4fbb3a0398013415769e484606a02"}]}' | jq
```
