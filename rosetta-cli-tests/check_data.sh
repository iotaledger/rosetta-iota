#!/bin/bash

CHRYSALIS_MAINNET_CONF_DIR="../rosetta-cli-conf/chrysalis-mainnet"
CHRYSALIS_DEVNET_CONF_DIR="../rosetta-cli-conf/chrysalis-devnet"
CHRYSALIS_MAINNET_BECH32_HRP="iota"
CHRYSALIS_DEVNET_BECH32_HRP="atoi"
DB_PATH="rosetta-cli-db"

if [ -z "$NETWORK" ]; then
  echo "please specify the network for which you want to test: chrysalis-mainnet or chrysalis-devnet"
  exit 1
fi

if [[ "$NETWORK" == "chrysalis-mainnet" ]]; then
  CONF_DIR=$CHRYSALIS_MAINNET_CONF_DIR
  HRP=$CHRYSALIS_MAINNET_BECH32_HRP
elif [[ "$NETWORK" == "chrysalis-devnet" ]]; then
  CONF_DIR=$CHRYSALIS_DEVNET_CONF_DIR
  HRP=$CHRYSALIS_DEVNET_BECH32_HRP
else
  echo "the provided network is not supported; you can test for: chrysalis-mainnet or chrysalis-devnet"
  exit 1
fi

# in case some rosetta-cli database already exists, remove it
rm -rf $DB_PATH

echo "copy node snapshots..."
if [[ "$NETWORK" == "chrysalis-mainnet" ]]; then
  cp -r ../data/snapshots/chrysalis-mainnet/* .
elif [[ "$NETWORK" == "chrysalis-devnet" ]]; then
  cp -r ../data/snapshots/chrysalis-devnet/* .
fi

# generate the bootstrap_balances.json file
echo "generate the bootstrap_balances.json file from the node snapshots..."
RUST_BACKTRACE=1 cargo run -p rosetta-iota-snapshot --release -- --bech32-hrp $HRP
ROSETTA_UTILS_EXIT=$?

if [ $ROSETTA_UTILS_EXIT -ne 0 ]; then
  exit 1
fi

# move bootstrap_balances file to $CONF_DIR
mv bootstrap_balances.json $CONF_DIR

SEP_INDEX=$(cat sep_index)
START_MS=`expr $SEP_INDEX + 1`

cat <<< $(jq --argjson START_MS "$START_MS" '.data.start_index |= $START_MS' $CONF_DIR/config.json) > $CONF_DIR/config.json

# clean up artifacts
rm delta_snapshot.bin
rm full_snapshot.bin
rm sep_index

# installing rosetta-cli
echo "installing rosetta-cli..."
curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s -- -b .
INSTALL_ROSETTA_CLI_EXIT=$?

if [ $INSTALL_ROSETTA_CLI_EXIT -ne 0 ]; then
  echo "unable to install rosetta-cli"
  exit 1
fi

# test Data API
echo "running rosetta-cli check:data"
./rosetta-cli check:data --configuration-file $CONF_DIR/config.json
DATA_EXIT=$?

if [ $DATA_EXIT -ne 0 ]; then
  echo "rosetta-cli check:data unsuccessful"
  exit 1
fi