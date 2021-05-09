# Testing with rosetta-cli

## Testing on testnet

The provided scripts `check_data_testnet.sh` and `check_construction_testnet.sh` build on top of rosetta-cli. They help with bootstrapping rosetta-cli and make sure that rosetta-cli is runs with the correct configuration values.

1) To be able to run the testing scripts, make sure you have **the latest version of [Rust](https://www.rust-lang.org/tools/install) installed.** **Also make sure [jq](https://wiki.ubuntuusers.de/jq/) is installed.**

2) Ensure the IOTA node is running **for the correct network** and an instance of the Rosetta API is available.

    To validate the correctness of `rosetta-iota` run the commands below:

    Data API **(chrysalis-mainnet)**:
    ```
    INSTALL=1 BOOTSTRAP_BALANCES=1 NETWORK=testnet7 ./check_data.sh
    ```
    Construction API **(chrysalis-mainnet)**:
    ```
    INSTALL=1 BOOTSTRAP_BALANCES=1 NETWORK=testnet7 ./check_construction.sh
    ```
    Data API **(testnet7)**:
    ```
    INSTALL=1 BOOTSTRAP_BALANCES=1 NETWORK=testnet7 ./check_data.sh
    ```
    Construction API **(testnet7)**:
    ```
    INSTALL=1 BOOTSTRAP_BALANCES=1 NETWORK=testnet7 ./check_construction.sh
    ```
The testing scripts make use of following environment variables:
- `INSTALL_ROSETTA_CLI=1` ...installs rosetta-cli to the current folder
- `BOOTSTRAP_BALANCES=1` ...deletes the rosetta-cli database, downloads the latest available IOTA snapshots and bootstraps balances
- `NO_BOOTSTRAP=1` ...keeps the rosetta-cli database and continues synching from the available state
- `NETWORK` = ...the network that should be tested on; can be either `chrysalis-mainnet` or `testnet7`