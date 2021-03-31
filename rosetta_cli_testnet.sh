#!/bin/bash

# kill any zombie process using ports 3030 + 3031
fuser -k 3030/tcp
fuser -k 3031/tcp

# clean up any previous modifications to config files
echo "reset rosetta-cli-conf/testnet..."
git checkout rosetta-cli-conf/testnet

# define a few vars
NODE_URL="https://api.hornet-rosetta.testnet.chrysalis2.com"
NETWORK="testnet6"
DATA_DIR=".rosetta-cli-testnet"
INDEXATION="rosetta"
HRP="atoi"
ROOT=$(pwd)
CONF_DIR=$ROOT/rosetta-cli-conf/testnet

# 1 to enable, comment out to disable
#LOAD_GENESIS=1 to start synching from genesis
#LOAD_SNAPSHOT=1 to start synching from the latest snapshot
#CONTINUE_DATA_DIR=1 to continue with the state from $DATA_DIR
#INSTALL=1
#RECONCILE=1
#DATA=1
#CONSTRUCTION=1

if [ $INSTALL ]; then
  # install rosetta-cli
  echo "installing rosetta-cli via curl..."
  curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s -- -b .
fi

if [ -z "$LOAD_GENESIS" ] && [ -z "$LOAD_SNAPSHOT" ] && [ -z "$CONTINUE_DATA_DIR" ]; then
  echo "loading method not set"
  exit 1
fi

if [ -z "$DATA" ] && [ -z "$CONSTRUCTION" ]; then
  echo "nothing to do... exiting"
  exit 1
fi

# start servers (online and offline)
RUST_BACKTRACE=1 RUST_LOG=iota_rosetta=debug cargo run -p rosetta-iota-server -- --network $NETWORK --iota-endpoint $NODE_URL --bech32-hrp $HRP --indexation $INDEXATION --port 3030 --mode online &
PID_ONLINE=$!

RUST_BACKTRACE=1 RUST_LOG=iota_rosetta=debug cargo run -p rosetta-iota-server -- --network $NETWORK --iota-endpoint $NODE_URL --bech32-hrp $HRP --indexation $INDEXATION --port 3031 --mode offline &
PID_OFFLINE=$!

# wait for the server to completely start
sleep 5

if [ $LOAD_GENESIS ]; then
  # remove the data directory
  rm -rf $DATA_DIR
  # all other values are already set in the default config
fi

if [ $LOAD_SNAPSHOT ]; then
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

# if there is no genesis/snapshot to load continue with the state from $DATA_DIR
if [ "$CONTINUE_DATA_DIR" ]; then
  cat <<< $(jq 'del(.data.start_index)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq 'del(.data.bootstrap_balances)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
fi

if [ $CONSTRUCTION ]; then
  echo "--------------------------------------------------------------------------------"
  echo "asking for faucet funds to load up prefunded_accounts..."

  PREFUNDED_ACCOUNT=$(RUST_BACKTRACE=1 cargo run -p rosetta-iota-utils -- --mode faucet 2> /dev/null)

  if [ -z "$PREFUNDED_ACCOUNT" ]; then
    echo "error on getting funds from faucet... exiting"
    exit 1
  fi

  echo "prefunded_account: ${PREFUNDED_ACCOUNT}"

  SK=$(echo $PREFUNDED_ACCOUNT | jq '.sk')
  ADDR=$(echo $PREFUNDED_ACCOUNT | jq '.bech32_addr')

  cat <<< $(jq --argjson ADDR "$ADDR" '.construction.prefunded_accounts[0].account_identifier.address |= $ADDR' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq --argjson SK "$SK" '.construction.prefunded_accounts[0].privkey |= $SK' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json

  # render $ADDR again, now without quotes
  ADDR=$(echo $PREFUNDED_ACCOUNT | jq '.bech32_addr' -r)

  OUTPUT_IDS=$(curl -s -X GET "$NODE_URL/api/v1/addresses/$ADDR/outputs" -H  "accept: application/json" | jq '.data.outputIds')
  OUTPUT_ID_A=$(echo $OUTPUT_IDS | jq '.[0]')
  OUTPUT_ID_B=$(echo $OUTPUT_IDS | jq '.[1]')

  echo "output_id_A: ${OUTPUT_ID_A}"
  echo "output_id_B: ${OUTPUT_ID_B}"

  sed -i 's/idA/'$OUTPUT_ID_A'/g' $CONF_DIR/iota.ros
  sed -i 's/idB/'$OUTPUT_ID_B'/g' $CONF_DIR/iota.ros
fi

if [ $RECONCILE ]; then
  cat <<< $(jq '.data.reconciliation_disabled |= false' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq '.data.end_conditions.reconciliation_coverage.coverage |= 0.95' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq '.data.end_conditions.reconciliation_coverage.from_tip |= true' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq 'del(.data.end_conditions.tip)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
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

kill $PID_ONLINE
kill $PID_OFFLINE
