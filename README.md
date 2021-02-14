# rosetta-iota
IOTA Rosetta API Implementation

## IOTA protocol
The IOTA protocol differs from most DLT technologies in the sense that it is not a Blockchain, but a Directed Acyclic Graph (DAG), popularly known as [the Tangle](https://assets.ctfassets.net/r1dr6vzfxhev/2t4uxvsIqk0EUau6g2sw0g/45eae33637ca92f85dd9f4a3a218e1ec/iota1_4_3.pdf).

For that reason, a few concepts have been adapted. For example, a "Milestone" is the IOTA equivalent of a "Block".

### Genesis Milestone
IOTA fullnodes (such as [HORNET](https://github.com/gohornet/hornet.git) and [BEE](https://github.com/iotaledger/bee.git)) don't contain the entire history of the Tangle. They have a parameter called `pruningIndex` which represents the oldest Milestone available on the node.

Only permanodes (such as [Chronicle](https://github.com/iotaledger/chronicle.rs/tree/main/chronicle-node)) have the ability of holding the entire Tangle history.

The genesis milestone is not available on fullnodes, but the `/network/status` endpoint response contains a `genesis_block_identifier` field. Therefore, whenever a fullnode is used by the `rosetta-iota` server, the `genesis_block_identifier` is populated as such:
```
"genesis_block_identifier": {
  "index": 1,
  "hash": "0000000000000000000000000000000000000000000000000000000000000000"
}
```

Technically speaking, that is not 100% accurate, as the genesis milestone identifier is not a series of `0`s. However, a dummy value is used as a compromise in order not to render the `/network/status` endpoint unavailable.

## Testing

### rosetta-cli

The `test.sh` shell script automates testing via `rosetta-cli`.

It will use the shell variables `NODE_URL` and `NETWORK` to specify the network parameters to be tested.
Bear in mind that syncing will start from the pruned Milestone, not Genesis.

### curl

Curl commands can also be used for manual inspection of each API endpoint.

1. Kickstart a `rosetta-iota` server:
```
$ cargo run -- --iota-endpoint http://0.0.0.0:14265 --network alphanet1 --hrp iota
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