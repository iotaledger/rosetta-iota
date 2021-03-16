#!/bin/bash

# kill any zombie process using ports 3030 + 3031
fuser -k 3030/tcp
fuser -k 3031/tcp

# clean up any previous modifications to config files
git checkout rosetta-cli-conf/rosetta-iota.json
git checkout rosetta-cli-conf/iota.ros

# define a few vars
NODE_URL="http://honeycombos.iota.cafe:14265"
NETWORK="testnet6"
DATA_DIR=".rosetta-cli"

ROOT=$(pwd)

# 1 to enable, comment out to disable
PRUNE=1
#INSTALL
#RECONCILE=1
#CLEAN=1
#DATA=1
#CONSTRUCTION=1

if [ $INSTALL ]; then
  # install rosetta-cli
  echo "installing rosetta-cli via curl..."
  curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s -- -b .
fi

# start servers (online and offline)
RUST_BACKTRACE=1 RUST_LOG=iota_rosetta=debug cargo run -- --network $NETWORK --iota-endpoint $NODE_URL --bech32-hrp atoi --indexation rosetta --port 3030 --mode online &
PID_ONLINE=$!

RUST_BACKTRACE=1 RUST_LOG=iota_rosetta=debug cargo run -- --network $NETWORK --iota-endpoint $NODE_URL --bech32-hrp atoi --indexation rosetta --port 3031 --mode offline &
PID_OFFLINE=$!

# wait for server to completely start
sleep 1

if [ $CLEAN ]; then
  rm -rf $DATA_DIR
else
  cat <<< $(jq 'del(.data.start_index)' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
fi

if [ $CONSTRUCTION ]; then
  echo "--------------------------------------------------------------------------------"
  echo "asking for faucet funds to load up prefunded_accounts..."

  cd src/utils
  PREFUNDED_ACCOUNT=$(RUST_BACKTRACE=1 cargo run 2> /dev/null)

  echo "prefunded_account: ${PREFUNDED_ACCOUNT}"

  SK=$(echo $PREFUNDED_ACCOUNT | jq '.sk')
  ADDR=$(echo $PREFUNDED_ACCOUNT | jq '.bech32_addr')

  cat <<< $(jq --argjson ADDR "$ADDR" '.construction.prefunded_accounts[0].account_identifier.address |= $ADDR' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
  cat <<< $(jq --argjson SK "$SK" '.construction.prefunded_accounts[0].privkey |= $SK' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json

  cd $ROOT

  # render $ADDR again, now without quotes
  ADDR=$(echo $PREFUNDED_ACCOUNT | jq '.bech32_addr' -r)

  OUTPUT_IDS=$(curl -s -X GET "$NODE_URL/api/v1/addresses/$ADDR/outputs" -H  "accept: application/json" | jq '.data.outputIds')
  OUTPUT_ID_A=$(echo $OUTPUT_IDS | jq '.[0]')
  OUTPUT_ID_B=$(echo $OUTPUT_IDS | jq '.[1]')

  echo "output_id_A: ${OUTPUT_ID_A}"
  echo "output_id_B: ${OUTPUT_ID_B}"

  sed -i 's/idA/'$OUTPUT_ID_A'/g' $ROOT/rosetta-cli-conf/iota.ros
  sed -i 's/idB/'$OUTPUT_ID_B'/g' $ROOT/rosetta-cli-conf/iota.ros
fi

if ([ $PRUNE ] && [ $CLEAN ]) || [ ! -d $DATA_DIR ]; then
  # modify rosetta-iota.json to make sure we are syncing from the pruned milestone
  PRUNE_MS=$(curl -s -X GET "$NODE_URL/api/v1/info" -H  "accept: application/json" | jq '.data.pruningIndex')
  START_MS=`expr $PRUNE_MS + 2`

  cat <<< $(jq --argjson START_MS "$START_MS" '.data.start_index |= $START_MS' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
  cat <<< $(jq '.data.pruning_disabled |= false' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
elif ! [ $PRUNE ]; then
  cat <<< $(jq 'del(.data.start_index)' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
  cat <<< $(jq '.data.pruning_disabled |= true' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
fi

if [ $RECONCILE ]; then
  cat <<< $(jq '.data.reconciliation_disabled |= false' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
  cat <<< $(jq '.data.end_conditions.reconciliation_coverage.coverage |= 0.95' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
  cat <<< $(jq '.data.end_conditions.reconciliation_coverage.from_tip |= true' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
else
  cat <<< $(jq '.data.reconciliation_disabled |= true' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
  cat <<< $(jq 'del(.data.end_conditions.reconciliation_coverage)' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
fi

cat <<< $(jq --arg NETWORK "$NETWORK" '.network.network |= $NETWORK' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
cat <<< $(jq --arg DATA_DIR "$DATA_DIR" '.data_directory |= $DATA_DIR' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json

if [ $DATA ]; then
  # test Data API
  echo "--------------------------------------------------------------------------------"
  echo "running rosetta-cli check:data"
  ./rosetta-cli check:data --configuration-file $ROOT/rosetta-cli-conf/rosetta-iota.json
fi

if [ $CONSTRUCTION ]; then
  # test Construction API
  echo "--------------------------------------------------------------------------------"
  echo "running rosetta-cli check:construction"
  ./rosetta-cli check:construction --configuration-file $ROOT/rosetta-cli-conf/rosetta-iota.json
fi

kill $PID_ONLINE
kill $PID_OFFLINE
