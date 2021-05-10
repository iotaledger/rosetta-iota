# Testing with rosetta-cli

## Testing on testnet

The provided scripts `check_data.sh` and `check_construction.sh` build on top of rosetta-cli. They help with bootstrapping rosetta-cli and make sure that rosetta-cli runs with the correct configuration values.

1) To be able to run the testing scripts, make sure you have **the latest version of [Rust](https://www.rust-lang.org/tools/install) installed.** **Also make sure [jq](https://wiki.ubuntuusers.de/jq/) is installed.**
2) Switch to the `rosetta-cli-tests/` directory.
3) Ensure the HORNET node is running **for the correct network** and an instance of the Rosetta API is available.

    To validate the correctness of `rosetta-iota` run the commands below:

    **chrysalis-mainnet: check data**
    ```
    INSTALL=1 BOOTSTRAP_BALANCES=1 NETWORK=chrysalis-mainnet ./check_data.sh
    ```
    **chrysalis-mainnet: check construction**
    ```
    INSTALL=1 BOOTSTRAP_BALANCES=1 NETWORK=chrysalis-mainnet ./check_construction.sh
    ```
    **testnet7: check data**
    ```
    INSTALL=1 BOOTSTRAP_BALANCES=1 NETWORK=testnet7 ./check_data.sh
    ```
    **testnet7: check construction**
    ```
    INSTALL=1 BOOTSTRAP_BALANCES=1 NETWORK=testnet7 ./check_construction.sh
    ```
The testing scripts make use of following environment variables:
- `INSTALL_ROSETTA_CLI=1` ...installs rosetta-cli to the current folder
- `BOOTSTRAP_BALANCES=1` ...deletes the rosetta-cli database, downloads the latest available IOTA snapshots and bootstraps balances
- `NO_BOOTSTRAP=1` ...keeps the rosetta-cli database and continues synching from the available state
- `NETWORK` = ...the network that should be tested on; can be either `chrysalis-mainnet` or `testnet7`