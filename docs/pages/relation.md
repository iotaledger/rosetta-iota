# Introduction to IOTA

The IOTA protocol differs from most DLT technologies in the sense that it uses a directed acyclic graph (DAG) data structure that allows transactions to be added in parallel.

IOTA nodes gossip "messages". A message is an envelope that may contain a payload. You can find more information about the message format [here](https://github.com/GalRogozinski/protocol-rfcs/blob/message/text/0017-message/0017-message.md). There are different types of payloads for different purposes. To transfer value, a "transaction payload" will be used.
Furthermore, transactions of funds are carried out in **UTXO** style. To learn more about the transaction payload click [here](https://github.com/luca-moser/protocol-rfcs/blob/signed-tx-payload/text/0000-transaction-payload/0000-transaction-payload.md).

## Blocks and Milestones

IOTA has the concept of Milestones as the closest analogy for Blocks. Periodically, an authorized entity issues a milestone (imagine as checkpoint) to the network to settle the ledger state.

## /block
The `/block` endpoint returns all transactions that happened on a specific Milestone.
To get all the balance changes from the IOTA node, `/block` will call the `http://localhost:14265/api/v1/milestones/:index/utxo-changes` endpoint which responds with a list of **created** and **consumed** output ids.
An output id is composed of: `transaction_id + output_index`.

### Rosetta::Transaction
The `Rosetta::TransactionIdentifier` of a `Rosetta::Transaction` gets populated by `IOTA::TransactionIdentifier`.
An `IOTA::TransactionIdentifier` describes the id of an `IOTA::TransactionPayload`.

Each Rosetta::Transaction has an array of [Operations](https://www.rosetta-api.org/docs/models/Operation.html), each one representing a UTXO Output.

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

```
