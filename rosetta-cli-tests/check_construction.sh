#!/bin/bash

CHRYSALIS_MAINNET_CONF_DIR="../rosetta-cli-conf/chrysalis-mainnet"
CHRYSALIS_DEVNET_CONF_DIR="../rosetta-cli-conf/chrysalis-devnet"
CHRYSALIS_MAINNET_BECH32_HRP="iota"
CHRYSALIS_DEVNET_BECH32_HRP="atoi"
DB_PATH="rosetta-cli-db"

if [[ "$NETWORK" == "chrysalis-mainnet" ]]; then
  CONF_DIR=$CHRYSALIS_MAINNET_CONF_DIR
  HRP=$CHRYSALIS_MAINNET_BECH32_HRP
elif [[ "$NETWORK" == "chrysalis-devnet" ]]; then
  CONF_DIR=$CHRYSALIS_DEVNET_CONF_DIR
  HRP=$CHRYSALIS_DEVNET_BECH32_HRP
else
  echo "please specify the network for which you want to test: you can test for chrysalis-mainnet or chrysalis-devnet"
  exit 1
fi

# in case some old rosetta-cli database already exists, remove it
rm -rf $DB_PATH

echo "installing rosetta-cli..."
curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s -- -b .
INSTALL_ROSETTA_CLI_EXIT=$?

if [ $INSTALL_ROSETTA_CLI_EXIT -ne 0 ]; then
  echo "unable to install rosetta-cli"
  exit 1
fi

echo "running rosetta-cli check:construction"
./rosetta-cli check:construction --configuration-file $CONF_DIR/config.json
CHECK_CONSTRUCTION_EXIT=$?

if [ $CHECK_CONSTRUCTION_EXIT -ne 0 ]; then
  echo "rosetta-cli check:construction unsuccessful..."
  exit 1
fi