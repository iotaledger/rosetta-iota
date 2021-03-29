#!/bin/bash

# kill any zombie process using ports 3030 + 3031 + 14265 + 14266
fuser -k 3030/tcp
fuser -k 3031/tcp
fuser -k 14265/tcp
fuser -k 14266/tcp

# clean up any previous modifications to config files
git checkout rosetta-cli-conf

# define a few vars
NODE_URL="http://localhost:14266"
NETWORK="alphanet1"
DATA_DIR=".rosetta-cli-alphanet"
INDEXATION="rosetta"
HRP="atoi"
ROOT=$(pwd)
CONF_DIR=$ROOT/rosetta-cli-conf/alphanet

# 1 to enable, comment out to disable
#PRUNE=1
#INSTALL
#RECONCILE=1
#CLEAN=1
#DATA=1
#CONSTRUCTION=1
#MAINNET=1

if [ $INSTALL ]; then
  # install rosetta-cli
  echo "installing rosetta-cli via curl..."
  curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s -- -b .
fi

# clone hornet
if [ ! -d hornet ]; then
  git clone https://github.com/gohornet/hornet.git -b develop
fi

# build hornet
cd hornet; go build

# initial funds go to atoi1qztz8a6c6wm5m4386vzmelfkcsuqewzzwnec7hnv88jtvh9wgq3hjch2kvv
cd alphanet
sed -i 's/fb9de5f493239dff165e574ec3f5be0f1f5a4c9e4ff2568d6b137445ebe4ff40/9623f758d3b74dd627d305bcfd36c4380cb84274f38f5e6c39e4b65cae402379/g' create_snapshot_alphanet.sh

# clean up databases and snapshots from previous runs
rm -rf alphanetdb
rm -rf alphanetdb2
rm -rf snapshots/alphanet1
rm -rf snapshots/alphanet2

# start coo + 2nd node
./run_coo_bootstrap.sh &> /dev/null &
COO=$!
./run_2nd.sh &> /dev/null &
SECOND=$!

# wait for alphanet to completely start
sleep 10

# start servers (online and offline)
cd $ROOT
cargo build -p rosetta-iota-server
RUST_BACKTRACE=1 RUST_LOG=iota_rosetta=debug cargo run -p rosetta-iota-server -- --network $NETWORK --iota-endpoint $NODE_URL --bech32-hrp $HRP --indexation $INDEXATION --port 3030 --mode online &
PID_ONLINE=$!

RUST_BACKTRACE=1 RUST_LOG=iota_rosetta=debug cargo run -p rosetta-iota-server -- --network $NETWORK --iota-endpoint $NODE_URL --bech32-hrp $HRP --indexation $INDEXATION --port 3031 --mode offline &
PID_OFFLINE=$!

# wait for server to completely start
sleep 5

if [ $CLEAN ]; then
  rm -rf $DATA_DIR
else
  cat <<< $(jq 'del(.data.start_index)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
fi

if [ $PRUNE ]; then
  RUST_BACKTRACE=1 cargo run -p rosetta-iota-utils -- --mode snapshot 2> /dev/null

  # move generated file to $CONF_DIR
  mv bootstrap_balances.json $CONF_DIR

  SEP_INDEX=$(cat sep_index)
  START_MS=`expr $SEP_INDEX + 1`

  cat <<< $(jq --argjson START_MS "$START_MS" '.data.start_index |= $START_MS' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq '.data.pruning_disabled |= false' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json

  # clean up artifacts
  rm delta_snapshot.bin
  rm full_snapshot.bin
  rm sep_index
else
  cat <<< $(jq 'del(.data.start_index)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq '.data.pruning_disabled |= true' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
fi

if [ $RECONCILE ]; then
  cat <<< $(jq '.data.reconciliation_disabled |= false' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq '.data.end_conditions.reconciliation_coverage.from_tip |= true' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq '.data.log_reconciliations |= true' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq 'del(.data.end_conditions.tip)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
else
  cat <<< $(jq '.data.reconciliation_disabled |= true' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq 'del(.data.end_conditions.reconciliation_coverage)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
fi

cat <<< $(jq --arg NETWORK "$NETWORK" '.network.network |= $NETWORK' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
cat <<< $(jq --arg DATA_DIR "$DATA_DIR" '.data_directory |= $DATA_DIR' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json

if [ $CONSTRUCTION ]; then
  # test Construction API
  echo "--------------------------------------------------------------------------------"
  echo "running rosetta-cli check:construction"
  ./rosetta-cli check:construction --configuration-file $CONF_DIR/rosetta-iota.json
  CONSTRUCTION_EXIT=$?
fi

if [ $CONSTRUCTION ] && [ $CONSTRUCTION_EXIT -ne 0 ]; then
  echo "rosetta-cli check:construction unsuccessful..."
  exit $CONSTRUCTION_EXIT
fi

if [ $DATA ]; then
  # test Data API
  echo "--------------------------------------------------------------------------------"
  echo "running rosetta-cli check:data"
  ./rosetta-cli check:data --configuration-file $CONF_DIR/rosetta-iota.json
  DATA_EXIT=$?
fi

if [ $DATA ] && [ $DATA_EXIT -ne 0 ]; then
  echo "rosetta-cli check:data unsuccessful..."
  exit 1
fi

if [ -z "$DATA" ] && [ -z "$CONSTRUCTION" ]; then
  echo "nothing to do... exiting"
fi

kill $PID_ONLINE
kill $PID_OFFLINE

# todo: find out how to kill coo + 2nd via pid
fuser -k 14265/tcp
fuser -k 14266/tcp
