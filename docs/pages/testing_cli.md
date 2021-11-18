# Testing with rosetta-cli

The provided scripts `check_data.sh` and `check_construction.sh` do set up rosetta-cli. They start rosetta-cli and make sure that rosetta-cli runs with the correct settings.

1) To be able to run the testing scripts, make sure you have **the latest version of [Rust](https://www.rust-lang.org/tools/install) installed.** **Also make sure [jq](https://wiki.ubuntuusers.de/jq/) is installed.**
2) Before continuing with the next step, make sure that the supplied HORNET node is running for the correct network and is fully synchronized with the network. Also make sure that an instance of the Rosetta API is available.
3) Switch to the `rosetta-cli-tests/` directory.
4) Start your desired rosetta-cli test:

    **chrysalis-mainnet: check data**
    ```
    NETWORK=chrysalis-mainnet ./check_data.sh
    ```
    **chrysalis-mainnet: check construction**
    ```
    NETWORK=chrysalis-mainnet ./check_construction.sh
    ```   
    **chrysalis-devnet: check data**
    ```
    NETWORK=chrysalis-devnet ./check_data.sh
    ```
    **chrysalis-devnet: check construction**
    ```
    NETWORK=chrysalis-devnet ./check_construction.sh
    ```
   
The testing scripts make use of following environment variable:
- `NETWORK` = ...the network that should be tested on; can be either `chrysalis-mainnet` or `chrysalis-devnet`