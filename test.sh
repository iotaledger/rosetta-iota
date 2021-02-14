#!/bin/bash

NODE_URL="http://honeycombos.iota.cafe:14265"
NETWORK="testnet4"

# start server
RUST_LOG=iota_rosetta=debug cargo run -- --network $NETWORK --iota-endpoint $NODE_URL &
PID=$!

# wait for server to completely start
sleep 1

# modify rosetta-iota.json
PRUNE_MS=$(curl -X GET "$NODE_URL/api/v1/info" -H  "accept: application/json" | jq '.data.pruningIndex')
START_MS=`expr $PRUNE_MS + 2`

cat <<< $(jq --argjson START_MS "$START_MS" '.data.start_index |= $START_MS' rosetta-iota.json) > rosetta-iota.json
cat <<< $(jq --arg NETWORK "$NETWORK" '.network.network |= $NETWORK' rosetta-iota.json) > rosetta-iota.json

# test Data API
~/bin/rosetta-cli check:data --configuration-file rosetta-iota.json

# test Construction API
# ~/bin/rosetta-cli check:construction --configuration-file rosetta-iota.json

kill $PID