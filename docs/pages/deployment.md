## IOTA nodes

**IOTA full-nodes** (such as [HORNET](https://github.com/gohornet/hornet.git) and [Bee](https://github.com/iotaledger/bee.git)) are able to start up securely from a recent block instead of having to synchronize from genesis.
Also, they are able to prune history from time to time in a safe way.
**IOTA full-nodes per default don't hold the entire Tangle history - they are not designed for it.**

Only **IOTA Permanodes** (such as [Chronicle](https://github.com/iotaledger/chronicle.rs/tree/main/chronicle-node)) **are designed to hold the entire Tangle history.**

`rosetta-iota` aims for a more reliable integration and better performance with limiting state storage. For this reasons **the Rosetta API implementation will be deployed with an IOTA full-node** ([HORNET](https://github.com/gohornet/hornet.git)).


## Quick start

Ensure `docker` and `docker-compose` are installed. As specified in the Rosetta API documentation, all Rosetta implementations must be deployable via Docker and support running via either an online or offline mode.

**Following commands will start an IOTA fullnode ([Hornet](https://github.com/gohornet/hornet)) together with a Rosetta API instance.**
Once the IOTA node has synced with the network, the Rosetta API will be available at: http://localhost:3030

**Testnet:Online**
```
NETWORK=testnet7 BECH32_HRP=atoi TX_TAG=Rosetta MODE=online docker-compose up
```

**Testnet:Offline**
```
NETWORK=testnet7 BECH32_HRP=atoi TX_TAG=Rosetta MODE=offline docker-compose up
```

## Testing with rosetta-cli
Ensure the IOTA node is running and an instance of the Rosetta API is available.
To validate the correctness of `rosetta-iota` run the commands below:

Testing the Data API **(Testnet)**:
```
ROSETTA_CLI_INSTALL=1 BOOTSTRAP_BALANCES=1 NODE_URL=http://localhost:14265 ./check_data_testnet.sh
```

Testing the Construction API **(Testnet)**:
```
ROSETTA_CLI_INSTALL=1 BOOTSTRAP_BALANCES=1 NODE_URL=http://localhost:14265 ./check_construction_testnet.sh
```