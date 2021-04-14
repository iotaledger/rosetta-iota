# Testing with rosetta-cli

## Testing on testnet

The provided scripts `check_data_testnet.sh` and `check_construction_testnet.sh` build on top of rosetta-cli. They help with bootstrapping rosetta-cli and make sure that rosetta-cli is runs with the correct configuration values.

1) To be able to run the testing scripts, install following dependencies:
    ```
    sudo apt-get install sed jq psmisc
    ```

2) Ensure the IOTA node is running and an instance of the Rosetta API is available.
Also ensure that the REST API of the IOTA is available at http://localhost:14265/api/v1/info.

    To validate the correctness of `rosetta-iota` run the commands below:

    Testing the Data API **(Testnet)**:
    ```
    ROSETTA_CLI_INSTALL=1 BOOTSTRAP_BALANCES=1 NODE_URL=http://localhost:14265 ./check_data_testnet.sh
    ```

    Testing the Construction API **(Testnet)**:
    ```
    ROSETTA_CLI_INSTALL=1 BOOTSTRAP_BALANCES=1 NODE_URL=http://localhost:14265 ./check_construction_testnet.sh
    ```
The testing scripts make use of following environment variables:
- `ROSETTA_CLI_INSTALL=1` ...installs rosetta-cli
- `BOOTSTRAP_BALANCES=1` ...deletes the rosetta-cli storage, downloads the most recent snapshots, bootstraps balances and starts synchronizing from the snapshot state
- `NO_BOOTSTRAP=1` ...keeps the rosetta-cli storage and continues to synchronize from the available state
- `NODE_URL=http://localhost:14265` ...the URL of the REST API from the IOTA node.  