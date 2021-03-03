#!/bin/bash

NODE_URL="http://honeycombos.iota.cafe:14265"
NETWORK="testnet6"

# 1 to enable, comment out to disable
PRUNE=1

# start server
RUST_LOG=iota_rosetta=debug cargo run -- --network $NETWORK --iota-endpoint $NODE_URL --port 3030 --mode online &
PID=$!

# wait for server to completely start
sleep 1

if [ $PRUNE ]; then

  # modify rosetta-iota.json to make sure we are syncing from the pruned milestone
  PRUNE_MS=$(curl -X GET "$NODE_URL/api/v1/info" -H  "accept: application/json" | jq '.data.pruningIndex')
  START_MS=`expr $PRUNE_MS + 2`

  cat <<< $(jq --argjson START_MS "$START_MS" '.data.start_index |= $START_MS' rosetta-cli-conf/rosetta-iota.json) > rosetta-cli-conf/rosetta-iota.json
  cat <<< $(jq '.data.pruning_disabled |= false' rosetta-cli-conf/rosetta-iota.json) > rosetta-cli-conf/rosetta-iota.json

else

  cat <<< $(jq 'del(.data.start_index)' rosetta-cli-conf/rosetta-iota.json) > rosetta-cli-conf/rosetta-iota.json
  cat <<< $(jq '.data.pruning_disabled |= true' rosetta-cli-conf/rosetta-iota.json) > rosetta-cli-conf/rosetta-iota.json

fi

cat <<< $(jq --arg NETWORK "$NETWORK" '.network.network |= $NETWORK' rosetta-cli-conf/rosetta-iota.json) > rosetta-cli-conf/rosetta-iota.json

# test Data API
~/bin/rosetta-cli check:data --configuration-file rosetta-cli-conf/rosetta-iota.json

# test Construction API
# ~/bin/rosetta-cli check:construction --configuration-file rosetta-cli-conf/rosetta-iota.json

kill $PID