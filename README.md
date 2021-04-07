<p align="center">
  <a href="https://www.rosetta-api.org">
    <img width="90%" alt="Rosetta" src="https://www.rosetta-api.org/img/rosetta_header.png">
  </a>
</p>
<h3 align="center">
   Rosetta IOTA
</h3>

## Overview

`rosetta-iota` provides a reference implementation of the Rosetta API for IOTA in Rust. If you haven't heard of the Rosetta API, you can find more information [here](https://www.rosetta-api.org/).

## Features

 - Implementation of both Data API and Construction API
 - Stateless, offline, curve-based transaction construction
 - [Storage Pruning](https://www.rosetta-api.org/docs/storage_pruning.html#docsNav) using a block height-based configuration setting
  
## Quick start

Ensure `docker` and `docker-compose` is installed. As specified in the Rosetta API documentation, all Rosetta implementations must be deployable via Docker and support running via either an online or offline mode.

**Following commands will start an IOTA fullnode ([Hornet](https://github.com/gohornet/hornet)) together with a Rosetta API instance.**
Once the IOTA node has synced with the network, the Rosetta API will be available at: http://localhost:3030

**Testnet:Online**
```
ROSETTA_BECH32_HRP=atoi ROSETTA_NETWORK_NAME=testnet7 ROSETTA_INDEXATION=Rosetta ROSETTA_MODE=online docker-compose up
```

**Testnet:Offline**
```
ROSETTA_BECH32_HRP=atoi ROSETTA_NETWORK_NAME=testnet7 ROSETTA_INDEXATION=Rosetta ROSETTA_MODE=offline docker-compose up
```

## Testing with rosetta-cli
Ensure the IOTA node is running and an instance of the Rosetta API is available.
To validate the correctness of `rosetta-iota`, [install `rosetta-cli`](https://github.com/coinbase/rosetta-cli#install) and run the commands below.

Testing the Data API **(Testnet)**:
```
./check_data_testnet.sh
```

Testing the Construction API **(Testnet)**:
```
./check_construction_testnet.sh
```
 
## Documentation

Please refer to [https://rosetta-api.docs.iota.org](https://rosetta-api.docs.iota.org) for further documentation.