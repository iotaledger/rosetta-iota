# rosetta-iota
IOTA Rosetta API Implementation

## Testing

## Environment

1. Bootstrap a HORNET Coordinator for a private testnet:
```
$ git clone https://github.com/gohornet/hornet.git
$ cd hornet/alphanet
$ ./run_coo_bootstrap.sh
``` 

2. From a new terminal, start a second HORNET node:
```
$ cd hornet/alphanet
$ ./run_2nd.sh
```

### rosetta-cli

The `test.sh` shell script automates testing via `rosetta-cli`.

The overall development goal is to pass all tests imposed by `rosetta-cli` with options:
- `check:data`
- `check:construction`

### curl

Curl commands can also be used for manual inspection of each API endpoint.

1. Kickstart a `rosetta-iota` server:
```
$ cargo run -- --iota-endpoint http://0.0.0.0:14265 --network alphanet1
```

2. From a new terminal:
```
$ curl --request POST 'http://localhost:3030/network/list' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"metadata":{}}' | jq
$ curl --request POST 'http://localhost:3030/network/status' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"alphanet1"}}' | jq
$ curl --request POST 'http://localhost:3030/block' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"alphanet1"},"block_identifier":{"index":2,"hash":""}}' | jq
```
