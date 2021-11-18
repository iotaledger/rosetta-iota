# Testing with rosetta-cli

The provided scripts `check_data.sh` and `check_construction.sh` build on top of rosetta-cli. They help with bootstrapping rosetta-cli and make sure that rosetta-cli runs with the correct configuration values.

1) To be able to run the testing scripts, make sure you have **the latest version of [Rust](https://www.rust-lang.org/tools/install) installed.** **Also make sure [jq](https://wiki.ubuntuusers.de/jq/) is installed.**
2) Switch to the `rosetta-cli-tests/` directory.
3) Ensure the HORNET node is running **for the correct network** and an instance of the Rosetta API is available. Then you can run the tests below:

    **chrysalis-mainnet: check data**
    ```
    NETWORK=chrysalis-mainnet ./check_data.sh
    ```
    **chrysalis-devnet: check construction**
    ```
    NETWORK=chrysalis-devnet ./check_construction.sh
    ```
   
The testing scripts make use of following environment variables:
- `NETWORK` = ...the network that should be tested on; can be either `chrysalis-mainnet` or `chrysalis-devnet`