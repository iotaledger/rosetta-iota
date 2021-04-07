#!/bin/bash

# define a few vars
DATA_DIR=".rosetta-cli-testnet"
ROOT=$(pwd)
CONF_DIR=$ROOT/rosetta-cli-conf/testnet

# 1 to enable, comment out to disable
# INSTALL_ROSETTA_CLI=1 ...installs rosetta-cli
# BOOTSTRAP_BALANCES=1 ...deletes the DATA_DIR, downloads the latest available IOTA snapshot and starts synching from the snapshot state
# NO_BOOTSTRAP=1 ...continues to synch where it ended last time (DATA_DIR must exist and the ledger state must be present)

if [ -z "$BOOTSTRAP_BALANCES" ] && [ -z "$NO_BOOTSTRAP" ]; then
  echo "bootstrapping method not specified..."
  exit 1
fi

# bootstrap balances
if [ $BOOTSTRAP_BALANCES ]; then
  # remove the data directory
  rm -rf $DATA_DIR

  # download latest snapshot and create the bootstrap_balances.json
  RUST_BACKTRACE=1 cargo run -p rosetta-iota-utils -- --mode snapshot 2> /dev/null

  # move generated file to $CONF_DIR
  mv bootstrap_balances.json $CONF_DIR

  SEP_INDEX=$(cat sep_index)
  START_MS=`expr $SEP_INDEX + 1`

  cat <<< $(jq --argjson START_MS "$START_MS" '.data.start_index |= $START_MS' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json

  # clean up artifacts
  rm delta_snapshot.bin
  rm full_snapshot.bin
  rm sep_index
fi

# start synching from $DATA_DIR
if [ "$NO_BOOTSTRAP" ]; then
  cat <<< $(jq 'del(.data.start_index)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq 'del(.data.bootstrap_balances)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  if [ -d "$DATA_DIR" ]; then
    echo "can not find data directory, please boostrap rosetta-cli..."
  fi
fi

# install rosetta-cli
if [ $INSTALL ]; then
  # install rosetta-cli
  echo "installing rosetta-cli via curl..."
  curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s -- -b .
fi
# test Data API
echo "--------------------------------------------------------------------------------"
echo "running rosetta-cli check:data"
./rosetta-cli check:data --configuration-file $CONF_DIR/rosetta-iota.json
DATA_EXIT=$?

if [ $DATA_EXIT -ne 0 ]; then
  echo "rosetta-cli check:data unsuccessful..."
  exit 1
fi