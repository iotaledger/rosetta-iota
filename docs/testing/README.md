# Testing

## rosetta-cli

The `rosetta_cli.sh` shell script automates testing via `rosetta-cli`.

Make sure you have run the following (on a Debian-based Linux) to install dependencies:
```
$ sudo apt-get install sed jq psmisc
```

The script uses the following shell variables:
 - `NODE_URL`: specifies which IOTA Node will be used to enter the network.
 - `NETWORK`: specifies the network (e.g.: `mainnet` or `testnet6`).
 - `DATA_DIR`: specifies where `rosetta-cli` should write its files. 
 - `INSTALL`: enables installation of `rosetta-cli` via `curl`. Disabled by default.
 - `PRUNE`: enables pruning (useful when no Permanode is available). Disabled by default.
 - `RECONCILE`: enables reconciliation. Disabled by default.
 - `CLEAN`: enables deletion of `$DATA_DIR` everytime the script is executed. Disabled by default.
 - `DATA`: enables execution of `rosetta-cli check:data`. Disabled by default.
 - `CONSTRUCTION`: enables execution of `rosetta-cli check:construction`. Disabled by default.
 
For example, you could run the script with the following options:
```
$ INSTALL=1 PRUNE=1 DATA=1 CONSTRUCTION=1 ./rosetta_cli.sh
```
 
## curl

Curl commands can also be used for manual inspection of each API endpoint.

1. Kickstart a `rosetta-iota` server:
```
$ cargo run -- --iota-endpoint http://0.0.0.0:14265 --network testnet6 --bech32-hrp atoi --indexation rosetta --port 3030 --mode online
```

2. From a new terminal, you can test each endpoint via `curl`:

- `/network/list`
```
$ curl --request POST 'http://localhost:3030/network/list' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"metadata":{}}' | jq
```

- `/network/status`
```
$ curl --request POST 'http://localhost:3030/network/status' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"}}' | jq
```

- `/network/options`
```
$ curl --request POST 'http://localhost:3030/network/options' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"}}' | jq
```

- `/block`
```
$ curl --request POST 'http://localhost:3030/block' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"},"block_identifier":{"index":2,"hash":""}}' | jq
```

- `/account/balance`
```
$ curl --request POST 'http://localhost:3030/account/balance' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"},"account_identifier":{"address":"atoi1qx0pteshrd554xtea4v3rklr97kzgc95umcpckn9pl897gnedk7gugyk5ld"}}' | jq
```

- `/account/coins`
```
$ curl --request POST 'http://localhost:3030/account/coins' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"},"account_identifier":{"address":"atoi1qx0pteshrd554xtea4v3rklr97kzgc95umcpckn9pl897gnedk7gugyk5ld"}}' | jq
```

- `/construction/derive` (`offline` mode only)
```
$ curl --request POST 'http://localhost:3030/construction/derive' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"},"public_key":{"hex_bytes":"6f8f4d77e94bce3900078b89319e6e25b341d47669a76ae4bf26677d377533f0","curve_type":"edwards25519"}}' | jq
```

- `/construction/metadata` (`online` mode only)
```
$ curl -s --request POST 'http://localhost:3030/construction/metadata' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"},"options":{"inputs":["4cad8ab0113e2fba1b9abb15d701c2d86dc100bda6a414b5f5de22f4809b45860100","8b64c69388ce190bd2c53c951b0ca899851f41bf580a031b4f253c302dc173420100"]}}' | jq
```

- `/construction/preprocess` (`offline` mode only)
```
$ curl --request POST 'http://localhost:3030/construction/preprocess' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"},"operations":[{"operation_identifier":{"index":0,"network_index":1},"related_operations":[],"type":"UTXO_INPUT","status":"SUCCESS","account":{"address":"atoi1qz4u7jdqn6d2qjt5y7tauctckl77nt6qv69reuhm7pdc98weu6cl2qylcgr"},"amount":{"value":"999820000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"837f9646c7cc5e748099d3abb946302e44927c96c648bcc3dd0f693258a61e1b0100"},"coin_action":"coin_spent"},"metadata":{"is_spent":"UTXO_UNSPENT"}},{"operation_identifier":{"index":0,"network_index":1},"related_operations":[{"index":1}],"type":"UTXO_OUTPUT","status":"SUCCESS","account":{"address":"atoi1qzcts3mukxthlwg90u2e5nrhpqt566fghhxdga83grmy60kt2akx2q0de5u"},"amount":{"value":"10000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"331bfc6eb2a2e02d7b7b9ce7aede370d8b4f6db3236887c5615ae36c523b2f060100"},"coin_action":"coin_created"},"metadata":{"is_spent":"UTXO_UNSPENT"}},{"operation_identifier":{"index":1,"network_index":0},"related_operations":[{"index":0}],"type":"UTXO_OUTPUT","status":"SUCCESS","account":{"address":"atoi1qz4u7jdqn6d2qjt5y7tauctckl77nt6qv69reuhm7pdc98weu6cl2qylcgr"},"amount":{"value":"999810000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"331bfc6eb2a2e02d7b7b9ce7aede370d8b4f6db3236887c5615ae36c523b2f060000"},"coin_action":"coin_created"},"metadata":{"is_spent":"UTXO_UNSPENT"}}],"metadata":{},"public_keys":[{"hex_bytes":"3ada770dca2c802df6837df29bdd8cd6c6d9d72cc4041743c7942cc11e6834bd","curve_type":"edwards25519"}]}' | jq
```

- `/construction/payloads` (`offline` mode only)
```
$ curl --request POST 'http://localhost:3030/construction/payloads' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"},"operations":[{"operation_identifier":{"index":0,"network_index":1},"related_operations":[],"type":"UTXO_INPUT","status":"SUCCESS","account":{"address":"atoi1qzpgz6elgl3t9wvn05q87y2l5vpse86t4xxh7g3fg8kk9zc4r3wmymtmd46"},"amount":{"value":"10000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"18063ffd63b305f81a17f633a7340ea0441555c19a2ed3d702763f103d2c91a10000"},"coin_action":"coin_spent"},"metadata":{"is_spent":"UTXO_UNSPENT"}},{"operation_identifier":{"index":0,"network_index":1},"related_operations":[],"type":"UTXO_INPUT","status":"SUCCESS","account":{"address":"atoi1qzpgz6elgl3t9wvn05q87y2l5vpse86t4xxh7g3fg8kk9zc4r3wmymtmd46"},"amount":{"value":"10000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"f504290ec9fad68987782e590e7201bf94776e0cea00bbd36a4253bbe536deda0000"},"coin_action":"coin_spent"},"metadata":{"is_spent":"UTXO_UNSPENT"}},{"operation_identifier":{"index":0,"network_index":1},"related_operations":[{"index":1}],"type":"UTXO_OUTPUT","status":"SUCCESS","account":{"address":"atoi1qq2hqdxds09ps7ge5pmczrpnaarllew2tmgcp648g40sdx8lg0vpzh0yvda"},"amount":{"value":"10000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"97dd9cb0cb873cf31a9ed6cbae111dc50760fca25012e9914201f8050bf734ac0000"},"coin_action":"coin_created"},"metadata":{"is_spent":"UTXO_UNSPENT"}},{"operation_identifier":{"index":1,"network_index":0},"related_operations":[{"index":0}],"type":"UTXO_OUTPUT","status":"SUCCESS","account":{"address":"atoi1qr5fk57dfqzcg64tvsm7xaeenarg376k0sc4zsjczwymvhtey2j9k853f2k"},"amount":{"value":"10000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"97dd9cb0cb873cf31a9ed6cbae111dc50760fca25012e9914201f8050bf734ac0100"},"coin_action":"coin_created"},"metadata":{"is_spent":"UTXO_UNSPENT"}}],"metadata":{"inputs_metadata":{"18063ffd63b305f81a17f633a7340ea0441555c19a2ed3d702763f103d2c91a10000":{"messageId":"729f7d96a38b85aa6fe82663553681460eb5236c4729e8f8378ae3094de570c9","transactionId":"18063ffd63b305f81a17f633a7340ea0441555c19a2ed3d702763f103d2c91a1","outputIndex":0,"isSpent":true,"output":{"type":0,"address":{"type":0,"address":"82816b3f47e2b2b9937d007f115fa3030c9f4ba98d7f222941ed628b151c5db2"},"amount":10000000}},"f504290ec9fad68987782e590e7201bf94776e0cea00bbd36a4253bbe536deda0000":{"messageId":"6bf253a0d0b352ea5b4506233f0a7792e43f118b874252e242d6ddaa89f185cc","transactionId":"f504290ec9fad68987782e590e7201bf94776e0cea00bbd36a4253bbe536deda","outputIndex":0,"isSpent":true,"output":{"type":0,"address":{"type":0,"address":"82816b3f47e2b2b9937d007f115fa3030c9f4ba98d7f222941ed628b151c5db2"},"amount":10000000}}}}}' | jq
```

- `/construction/parse` (`offline` mode only)
```
$ curl --request POST 'http://localhost:3030/construction/parse' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"},"signed":false,"transaction":"7b22657373656e6365223a7b2274797065223a22526567756c6172222c2264617461223a7b22696e70757473223a5b7b2274797065223a225554584f222c2264617461223a223138303633666664363362333035663831613137663633336137333430656130343431353535633139613265643364373032373633663130336432633931613130303030227d2c7b2274797065223a225554584f222c2264617461223a226635303432393065633966616436383938373738326535393065373230316266393437373665306365613030626264333661343235336262653533366465646130303030227d5d2c226f757470757473223a5b7b2274797065223a225369676e61747572654c6f636b656453696e676c65222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2231353730333463643833636131383739313961303737383130633333656634376666653563613565643138306561613734353566303639386666343364383131227d2c22616d6f756e74223a31303030303030307d7d2c7b2274797065223a225369676e61747572654c6f636b656453696e676c65222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2265383962353363643438303538343661616236343337653337373339396634363838666235363763333135313432353831333839623635643739323261343562227d2c22616d6f756e74223a31303030303030307d7d5d2c227061796c6f6164223a6e756c6c7d7d2c22696e707574735f6d65746164617461223a7b223138303633666664363362333035663831613137663633336137333430656130343431353535633139613265643364373032373633663130336432633931613130303030223a7b226d6573736167654964223a2237323966376439366133386238356161366665383236363335353336383134363065623532333663343732396538663833373861653330393464653537306339222c227472616e73616374696f6e4964223a2231383036336666643633623330356638316131376636333361373334306561303434313535356331396132656433643730323736336631303364326339316131222c226f7574707574496e646578223a302c2269735370656e74223a747275652c226f7574707574223a7b2274797065223a302c2261646472657373223a7b2274797065223a302c2261646472657373223a2238323831366233663437653262326239393337643030376631313566613330333063396634626139386437663232323934316564363238623135316335646232227d2c22616d6f756e74223a31303030303030307d7d2c226635303432393065633966616436383938373738326535393065373230316266393437373665306365613030626264333661343235336262653533366465646130303030223a7b226d6573736167654964223a2236626632353361306430623335326561356234353036323333663061373739326534336631313862383734323532653234326436646461613839663138356363222c227472616e73616374696f6e4964223a2266353034323930656339666164363839383737383265353930653732303162663934373736653063656130306262643336613432353362626535333664656461222c226f7574707574496e646578223a302c2269735370656e74223a747275652c226f7574707574223a7b2274797065223a302c2261646472657373223a7b2274797065223a302c2261646472657373223a2238323831366233663437653262326239393337643030376631313566613330333063396634626139386437663232323934316564363238623135316335646232227d2c22616d6f756e74223a31303030303030307d7d7d7d"}' | jq
```

- `/construction/combine` (`offline` mode only)
```
$ curl --request POST 'http://localhost:3030/construction/combine' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"testnet6"},"unsigned_transaction":"0001000063aa5b2476105ad4ebbb995968216cef0d940791d900afd1180a8df4e527b3460000010000005eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8050000000000000000000000","signatures":[{"signing_payload":{"account_identifier":{"address":"ae98475c63cfebc918b57193a4183f4374f67974971aff9034699793d331d7de"},"hex_bytes":"c8be532dd8f351c28b6cdb7903d076b356129189d11ad50c46feb25ca472b984","signature_type":"ed25519"},"public_key":{"hex_bytes":"b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde","curve_type":"edwards25519"},"signature_type":"ed25519","hex_bytes":"59057634a166d84edebb88ab17a33394c0ccf1d46adffc3a0095c0f5e7e9dbe766ab05cb0244486d272e61f1e09db1bdb6e4fbb3a0398013415769e484606a02"}]}' | jq
```