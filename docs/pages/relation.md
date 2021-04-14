# Relation between IOTA and Rosetta

The IOTA protocol differs from most DLT technologies in the sense that it uses a directed acyclic graph (DAG) data structure that allows transactions to be added in parallel.

## Blocks and Milestones

IOTA has the concept of Milestones as the closest analogy for Blocks. Periodically, an authorized entity issues a milestone (imagine as checkpoint) to the network to settle the ledger state.

## Messages and Transactions
IOTA nodes gossip "messages". A message is an envelope that may contain a payload. You can find more information about the message format [here](https://github.com/GalRogozinski/protocol-rfcs/blob/message/text/0017-message/0017-message.md). There are different types of payloads for different purposes. To transfer value, a "transaction payload" will be used.
Furthermore, transactions of funds are carried out in **UTXO** style. To learn more about the transaction payload click [here](https://github.com/luca-moser/protocol-rfcs/blob/signed-tx-payload/text/0000-transaction-payload/0000-transaction-payload.md).

# Design choices

Here we describe how concepts from the IOTA protocol were adapted for the Rosetta Endpoints.

## Genesis Milestone

Per default, the genesis milestone is not available on full-nodes, but the `/network/status` endpoint response contains a required `genesis_block_identifier` field. The `genesis_block_identifier` is populated as such:
```
"genesis_block_identifier": {
  "index": 1,
  "hash": "0000000000000000000000000000000000000000000000000000000000000000"
}
```

Technically speaking, that is not accurate, as the genesis milestone identifier is not a series of `0`s. However, a dummy value is used as a compromise in order not to render the `/network/status` endpoint unavailable. 

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
