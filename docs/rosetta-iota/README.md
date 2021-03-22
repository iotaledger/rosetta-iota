# IOTA protocol
The IOTA protocol differs from most DLT technologies in the sense that it is not a Blockchain, but a Directed Acyclic Graph (DAG), popularly known as [the Tangle](https://assets.ctfassets.net/r1dr6vzfxhev/2t4uxvsIqk0EUau6g2sw0g/45eae33637ca92f85dd9f4a3a218e1ec/iota1_4_3.pdf).

For that reason, a few concepts have been adapted in relation to the Rosetta API.

## Messages and Transactions
IOTA uses messages as a envelope around a payload that can consist of data (indexation), value (transactions) or a combination of both (a indexation payload embedded in a transaction). Transactions use a utxo-based model for transfering value.

## Blocks and Milestones
Being a DAG, IOTA has the concept of Milestones as the closest analogy for Blocks. Periodically, nodes settle the current ledger state by creating a special message that defines a new Milestone.

IOTA Fullnodes (such as [HORNET](https://github.com/gohornet/hornet.git) and [BEE](https://github.com/iotaledger/bee.git)) don't contain the entire history of the Tangle. They have a parameter called `pruningIndex` which represents the oldest Milestone available on the node.

Only Permanodes (such as [Chronicle](https://github.com/iotaledger/chronicle.rs/tree/main/chronicle-node)) have the ability of holding the entire Tangle history.

## Node Endpoints
The following IOTA Node endpoints are necessary for the `rosetta-iota` implementation:
 - `/api/v1/info`
 - `/api/v1/milestones`
 - `/api/v1/milestones/{milestoneId}/utxo-changes`
 - `/api/v1/address`
 - `/api/v1/peers`
 - `/api/v1/outputs`
 - `/api/v1/messages`

# IOTA and Rosetta

Here we describe how concepts from the IOTA protocol were adapted for the Rosetta Endpoints.

## Genesis Milestone

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

## Transactions and Operations
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
  "transactions": [
    {
      "transaction_identifier": {
        "hash": "586d11477a48a25b4a554686ce8d6cf711be3a85cf07a0dc2d07d4e0f4c03636"
      },
      "operations": [
        {
          "operation_identifier": {
            "index": 0,
            "network_index": 2
          },
          "type": "UTXO_INPUT",
          "status": "SUCCESS",
          "account": {
            "address": "atoi1q8k69lxuxljdgeqt7tucvtdfk3hrvrly7rzz65w57te6drf3expsjth4u2j"
          },
          "amount": {
            "value": "-20000000",
            "currency": {
              "symbol": "IOTA",
              "decimals": 0
            }
          },
          "coin_change": {
            "coin_identifier": {
              "identifier": "9ce415875aa9ed67c4a3b97cb598861be11444e880fad604ec2a96ee65590da80000"
            },
            "coin_action": "coin_spent"
          },
          "metadata": {
            "is_spent": "UTXO_UNSPENT"
          }
        },
        {
          "operation_identifier": {
            "index": 1,
            "network_index": 0
          },
          "type": "UTXO_INPUT",
          "status": "SUCCESS",
          "account": {
            "address": "atoi1q86v9cgc8d9ue9nd9k4z8rp2dn29mcwf2y9jhm59u8dg25aukhq3jqwwfvf"
          },
          "amount": {
            "value": "-10000000",
            "currency": {
              "symbol": "IOTA",
              "decimals": 0
            }
          },
          "coin_change": {
            "coin_identifier": {
              "identifier": "586d11477a48a25b4a554686ce8d6cf711be3a85cf07a0dc2d07d4e0f4c036360100"
            },
            "coin_action": "coin_spent"
          },
          "metadata": {
            "is_spent": "UTXO_UNSPENT"
          }
        },
        {
          "operation_identifier": {
            "index": 2,
            "network_index": 0
          },
          "type": "UTXO_OUTPUT",
          "status": "SUCCESS",
          "account": {
            "address": "atoi1q8k69lxuxljdgeqt7tucvtdfk3hrvrly7rzz65w57te6drf3expsjth4u2j"
          },
          "amount": {
            "value": "30000000",
            "currency": {
              "symbol": "IOTA",
              "decimals": 0
            }
          },
          "coin_change": {
            "coin_identifier": {
              "identifier": "9ce415875aa9ed67c4a3b97cb598861be11444e880fad604ec2a96ee65590da80000"
            },
            "coin_action": "coin_created"
          },
          "metadata": {
            "is_spent": "UTXO_SPENT"
          }
        }
      ]
    }
  ]
}
```
