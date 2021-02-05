#!/bin/bash

# start server
RUST_LOG=iota_rosetta=debug cargo run -- --network alphanet1 --iota-endpoint http://0.0.0.0:14265 &
PID=$!

sleep 1

# test Data API
~/bin/rosetta-cli check:data --configuration-file rosetta-iota.json

# test Construction API
~/bin/rosetta-cli check:construction --configuration-file rosetta-iota.json

kill $PID