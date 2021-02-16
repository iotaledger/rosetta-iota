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

The genesis milestone is not available on Fullnodes, but the `/network/status` endpoint response contains a `genesis_block_identifier` field. Therefore, whenever a fullnode is used by the `rosetta-iota` server, the `genesis_block_identifier` is populated as such:
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
That is achieved via the `/api/v1/milestones/:milestoneId/utxo-changes` IOTA fullnode endpoint, where a list of **Created** and **Consumed** UTXO Outputs is returned.

Each UTXO Output contains a `output_id`, a `transaction_id` and a `output_index`, where `output_id = transaction_id + output_index`.

In terms of Rosetta Models, the `TransactionIdentifier` of a `Transaction` Object is defined by an IOTA `transaction_id`.

Each `Transaction` Object has an array of `Operations`, each one representing a UTXO Output. The `index` of the `OperationIdentifier` is incremented from 0 and its `network_index` is defined by `output_index`.

UTXO Operation types are defined as:
- `UTXO_CONSUMED`
- `UTXO_CREATED`

UTXO Operation statuses are defined as:
- `UTXO_SPENT`
- `UTXO_UNSPENT`

Here's an example of a Transaction Object:
```
{
   "transaction_identifier":{
      "hash":"61ac6191f8821ee0685b9f402e7a73a93914b54ff1454492bf94fe9fbc6f59b2"
   },
   "operations":[
      {
         "operation_identifier":{
            "index":0,
            "network_index":0
         },
         "type":"UTXO_CONSUMED",
         "status":"UTXO_SPENT",
         "account":{
            "address":"atoi1qx0s7p6z46eupqp5njqv0hhm8q5hgejtr67mexmtgzvn57rjd8k8jd43kz8"
         },
         "amount":{
            "value":"9000000",
            "currency":{
               "symbol":"IOTA",
               "decimals":0
            }
         }
      },
      {
         "operation_identifier":{
            "index":1,
            "network_index":1
         },
         "type":"UTXO_CREATED",
         "status":"UTXO_UNSPENT",
         "account":{
            "address":"atoi1qxstpvdjjku0g5756ej65sj6xcxgjux0q4k5rpjn8qjy9mn38tdhwdwn505"
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
$ cargo run -- --iota-endpoint http://0.0.0.0:14265 --network alphanet1
```

2. From a new terminal, you can test each endpoint via `curl`:

- `/network/list`
```
$ curl --request POST 'http://localhost:3030/network/list' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"metadata":{}}' | jq
```

- `/network/status`
```
$ curl --request POST 'http://localhost:3030/network/status' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"alphanet1"}}' | jq
```

- `/network/options`
```
$ curl --request POST 'http://localhost:3030/network/options' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"alphanet1"}}' | jq
```

- `/block`
```
$ curl --request POST 'http://localhost:3030/block' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"alphanet1"},"block_identifier":{"index":2,"hash":""}}' | jq
```

- `/account/balance`
```
$ curl --request POST 'http://localhost:3030/account/balance' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"alphanet1"},"account_identifier":{"address":"atoi1qx0pteshrd554xtea4v3rklr97kzgc95umcpckn9pl897gnedk7gugyk5ld"}}' | jq```
```

- `/construction/derive`
```
$ curl --request POST 'http://localhost:3030/construction/derive' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"alphanet1"},"public_key":{"hex_bytes":"6f8f4d77e94bce3900078b89319e6e25b341d47669a76ae4bf26677d377533f0","curve_type":"edwards25519"}}' | jq```
```