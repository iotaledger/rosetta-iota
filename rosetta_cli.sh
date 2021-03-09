#!/bin/bash

git checkout rosetta-cli-conf/rosetta-iota.json
git checkout rosetta-cli-conf/iota.ros

NODE_URL="http://honeycombos.iota.cafe:14265"
NETWORK="testnet6"
DATA_DIR=".rosetta-cli"

ROOT=$(pwd)

# 1 to enable, comment out to disable
PRUNE=1
#RECONCILE=1
CLEAN=1
PREFUNDED_ACCOUNT=1

# start server
RUST_LOG=iota_rosetta=debug cargo run -- --network $NETWORK --iota-endpoint $NODE_URL --port 3030 --mode online &
PID=$!

# wait for server to completely start
sleep 1

if [ $CLEAN ]; then
  rm -rf $DATA_DIR
fi

if [ $PREFUNDED_ACCOUNT ]; then

  echo "--------------------------------------------------------------------------------"
  echo "asking for faucet funds to load up prefunded_accounts..."

  cd src/utils
  PREFUNDED_ACCOUNT=$(cargo run)

  SK=$(echo $PREFUNDED_ACCOUNT | jq '.sk')
  ADDR=$(echo $PREFUNDED_ACCOUNT | jq '.bech32_addr')

  cat <<< $(jq --argjson ADDR "$ADDR" '.construction.prefunded_accounts[0].account_identifier.address |= $ADDR' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
  cat <<< $(jq --argjson SK "$SK" '.construction.prefunded_accounts[0].privkey |= $SK' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json

  cd $ROOT

  OUTPUT_IDS=$(curl -X GET "$NODE_URL/api/v1/addresses/$ADDR/outputs" -H  "accept: application/json" | jq '.data.outputIds')
  OUTPUT_ID_A=$(echo $OUTPUT_IDS | jq '.[0]')
  OUTPUT_ID_B=$(echo $OUTPUT_IDS | jq '.[1]')

  sed -i 's/idA/'$OUTPUT_ID_A'/g' $ROOT/rosetta-cli-conf/iota.ros
  sed -i 's/idB/'$OUTPUT_ID_B'/g' $ROOT/rosetta-cli-conf/iota.ros

fi

if [ $PRUNE ]; then

  # modify rosetta-iota.json to make sure we are syncing from the pruned milestone
  PRUNE_MS=$(curl -X GET "$NODE_URL/api/v1/info" -H  "accept: application/json" | jq '.data.pruningIndex')
  START_MS=`expr $PRUNE_MS + 2`

  cat <<< $(jq --argjson START_MS "$START_MS" '.data.start_index |= $START_MS' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json
  cat <<< $(jq '.data.pruning_disabled |= false' $ROOT/rosetta-cli-conf/rosetta-iota.json) > $ROOT/rosetta-cli-conf/rosetta-iota.json

else

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

# test Data API
echo "--------------------------------------------------------------------------------"
echo "running rosetta-cli check:data"
~/bin/rosetta-cli check:data --configuration-file $ROOT/rosetta-cli-conf/rosetta-iota.json

# test Construction API
echo "--------------------------------------------------------------------------------"
echo "running rosetta-cli check:construction"
~/bin/rosetta-cli check:construction --configuration-file $ROOT/rosetta-cli-conf/rosetta-iota.json

kill $PID