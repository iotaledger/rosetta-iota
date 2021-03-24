#!/bin/bash

# kill any zombie process using ports 3030 + 3031
fuser -k 3030/tcp
fuser -k 3031/tcp

# clean up any previous modifications to config files
git checkout rosetta-cli-conf

# define a few vars
if [ -z "$NODE_URL" ]; then
  NODE_URL="https://api.hornet-rosetta.testnet.chrysalis2.com"
fi
if [ -z "$NETWORK" ]; then
  NETWORK="testnet6"
fi
if [ -z "$DATA_DIR" ]; then
  DATA_DIR=".rosetta-cli"
fi
if [ -z "$INDEXATION" ]; then
  INDEXATION="rosetta"
fi
if [ -z "$HRP" ]; then
  HRP="atoi"
fi

ROOT=$(pwd)

# 1 to enable, comment out to disable
PRUNE=1
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

# start servers (online and offline)
RUST_BACKTRACE=1 RUST_LOG=iota_rosetta=debug cargo run -- --network $NETWORK --iota-endpoint $NODE_URL --bech32-hrp $HRP --indexation $INDEXATION --port 3030 --mode online &
PID_ONLINE=$!

RUST_BACKTRACE=1 RUST_LOG=iota_rosetta=debug cargo run -- --network $NETWORK --iota-endpoint $NODE_URL --bech32-hrp $HRP --indexation $INDEXATION --port 3031 --mode offline &
PID_OFFLINE=$!

# wait for server to completely start
sleep 1

if [ $MAINNET ]; then
  CONF_DIR=$ROOT/rosetta-cli-conf/mainnet
else
  CONF_DIR=$ROOT/rosetta-cli-conf/testnet
fi

if [ $CLEAN ]; then
  rm -rf $DATA_DIR
else
  cat <<< $(jq 'del(.data.start_index)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
fi

if [ $CONSTRUCTION ]; then
  echo "--------------------------------------------------------------------------------"
  echo "asking for faucet funds to load up prefunded_accounts..."

  cd src/utils
  PREFUNDED_ACCOUNT=$(RUST_BACKTRACE=1 cargo run -- --mode faucet 2> /dev/null)

  if [ -z "$PREFUNDED_ACCOUNT" ]; then
    echo "error on getting funds from faucet... exiting"
    exit 1
  fi

  echo "prefunded_account: ${PREFUNDED_ACCOUNT}"

  SK=$(echo $PREFUNDED_ACCOUNT | jq '.sk')
  ADDR=$(echo $PREFUNDED_ACCOUNT | jq '.bech32_addr')

  cat <<< $(jq --argjson ADDR "$ADDR" '.construction.prefunded_accounts[0].account_identifier.address |= $ADDR' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq --argjson SK "$SK" '.construction.prefunded_accounts[0].privkey |= $SK' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json

  cd $ROOT

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

if ([ $PRUNE ] && [ $CLEAN ]) || [ ! -d $DATA_DIR ]; then
  # modify rosetta-iota.json to make sure we are syncing from the pruned milestone
  PRUNE_MS=$(curl -s -X GET "$NODE_URL/api/v1/info" -H  "accept: application/json" | jq '.data.pruningIndex')

  # when starting from a $PRUNE_MS != 0, jump 2 because of parent block + unavailable pruning MS
  if [ "$PRUNE_MS" -gt "0" ]; then
    START_MS=`expr $PRUNE_MS + 2`
  else
    START_MS="1"
  fi

  cat <<< $(jq --argjson START_MS "$START_MS" '.data.start_index |= $START_MS' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq '.data.pruning_disabled |= false' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
elif ! [ $PRUNE ]; then
  cat <<< $(jq 'del(.data.start_index)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq '.data.pruning_disabled |= true' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
fi

if [ $RECONCILE ]; then
  cat <<< $(jq '.data.reconciliation_disabled |= false' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq '.data.end_conditions.reconciliation_coverage.coverage |= 0.95' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq '.data.end_conditions.reconciliation_coverage.from_tip |= true' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq 'del(.data.end_conditions.tip)' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
else
  cat <<< $(jq '.data.reconciliation_disabled |= true' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
  cat <<< $(jq 'del(.data.end_conditions.reconciliation_coverage)' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
fi

cat <<< $(jq --arg NETWORK "$NETWORK" '.network.network |= $NETWORK' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json
cat <<< $(jq --arg DATA_DIR "$DATA_DIR" '.data_directory |= $DATA_DIR' $CONF_DIR/rosetta-iota.json) > $CONF_DIR/rosetta-iota.json

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

if [ -z "$DATA" ] && [ -z "$CONSTRUCTION" ]; then
  echo "nothing to do... exiting"
fi

kill $PID_ONLINE
kill $PID_OFFLINE
