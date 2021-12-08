# Testing with curl

Curl commands can be used for manual inspection of each API endpoint.

1. Ensure the IOTA node is running and an instance of the Rosetta API is available.

2. From a new terminal, you can test each endpoint via `curl`:

## Data API

`/network/list`
```
curl --request POST 'http://localhost:3030/network/list' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"metadata":{}}' | jq
```

`/network/status`
```
curl --request POST 'http://localhost:3030/network/status' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"}}' | jq
```

`/network/options`
```
curl --request POST 'http://localhost:3030/network/options' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"}}' | jq
```

`/block`
```
curl --request POST 'http://localhost:3030/block' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"block_identifier":{"index":61200}}' | jq
```

`/account/balance`
```
curl --request POST 'http://localhost:3030/account/balance' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"account_identifier":{"address":"atoi1qx0pteshrd554xtea4v3rklr97kzgc95umcpckn9pl897gnedk7gugyk5ld"}}' | jq
```

`/account/coins`
```
curl --request POST 'http://localhost:3030/account/coins' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"account_identifier":{"address":"atoi1qzpe9s3w9q2y2pkt2pd6c4w5a7ntrm95nz8vnnjzdw3t04wg33n6w3tk40e"}}' | jq
```

## Construction API

`/construction/derive`
```
curl --request POST 'http://localhost:3030/construction/derive' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"public_key":{"hex_bytes":"6f8f4d77e94bce3900078b89319e6e25b341d47669a76ae4bf26677d377533f0","curve_type":"edwards25519"}}' | jq
```

`/construction/preprocess`
```
curl --request POST 'http://localhost:3030/construction/preprocess' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"operations":[{"operation_identifier":{"index":0,"network_index":0},"type":"UTXO_INPUT","account":{"address":"atoi1qr49znuapruu3fhwcfd4vsq2y3a0l9k8zc6pv6ak70g4hd9jq8fr2lqf6et"},"amount":{"value":"-10000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"8bec7fd0a9fdc351adaaf07f595afefa7844eafd183625949e51dcb3b9632b890000"},"coin_action":"coin_spent"}},{"operation_identifier":{"index":1},"type":"UTXO_OUTPUT","account":{"address":"atoi1qpmppfmvwlg5qjkwd8084ceh0emw6y9gegpmesn2vvrlacfep834wyqsxww"},"amount":{"value":"8604736","currency":{"symbol":"IOTA","decimals":0}}},{"operation_identifier":{"index":2},"type":"UTXO_OUTPUT","account":{"address":"atoi1qp08ypmqn53kxxmj7d60wqp6hwtcc25sv8y950j7e35fjnj3dmpxyp7l5y9"},"amount":{"value":"395264","currency":{"symbol":"IOTA","decimals":0}}},{"operation_identifier":{"index":3},"type":"DUST_ALLOWANCE_OUTPUT","account":{"address":"atoi1qp08ypmqn53kxxmj7d60wqp6hwtcc25sv8y950j7e35fjnj3dmpxyp7l5y9"},"amount":{"value":"1000000","currency":{"symbol":"IOTA","decimals":0}}}]}' | jq
```

`/construction/metadata`
```
curl -s --request POST 'http://localhost:3030/construction/metadata' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"options":{"utxo_inputs":["8bec7fd0a9fdc351adaaf07f595afefa7844eafd183625949e51dcb3b9632b890000"]}}' | jq
```

`/construction/payloads`
```
curl --request POST 'http://localhost:3030/construction/payloads' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"operations":[{"operation_identifier":{"index":0,"network_index":0},"type":"UTXO_INPUT","account":{"address":"atoi1qr49znuapruu3fhwcfd4vsq2y3a0l9k8zc6pv6ak70g4hd9jq8fr2lqf6et"},"amount":{"value":"-10000000","currency":{"symbol":"IOTA","decimals":0}},"coin_change":{"coin_identifier":{"identifier":"8bec7fd0a9fdc351adaaf07f595afefa7844eafd183625949e51dcb3b9632b890000"},"coin_action":"coin_spent"}},{"operation_identifier":{"index":1},"type":"UTXO_OUTPUT","account":{"address":"atoi1qpmppfmvwlg5qjkwd8084ceh0emw6y9gegpmesn2vvrlacfep834wyqsxww"},"amount":{"value":"8604736","currency":{"symbol":"IOTA","decimals":0}}},{"operation_identifier":{"index":2},"type":"UTXO_OUTPUT","account":{"address":"atoi1qp08ypmqn53kxxmj7d60wqp6hwtcc25sv8y950j7e35fjnj3dmpxyp7l5y9"},"amount":{"value":"395264","currency":{"symbol":"IOTA","decimals":0}}},{"operation_identifier":{"index":3},"type":"DUST_ALLOWANCE_OUTPUT","account":{"address":"atoi1qp08ypmqn53kxxmj7d60wqp6hwtcc25sv8y950j7e35fjnj3dmpxyp7l5y9"},"amount":{"value":"1000000","currency":{"symbol":"IOTA","decimals":0}}}], "metadata":{"utxo_inputs_metadata":{"8bec7fd0a9fdc351adaaf07f595afefa7844eafd183625949e51dcb3b9632b890000":{"messageId":"2f2e4f2d79cae50aecf9a26292f693b1335a692d1e3e452983aeec165968dad8","transactionId":"8bec7fd0a9fdc351adaaf07f595afefa7844eafd183625949e51dcb3b9632b89","outputIndex":0,"isSpent":true,"output":{"type":0,"address":{"type":0,"address":"ea514f9d08f9c8a6eec25b56400a247aff96c71634166bb6f3d15bb4b201d235"},"amount":10000000}}}}}' | jq
```

`/construction/parse` (unsigned)
```
curl --request POST 'http://localhost:3030/construction/parse' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"signed":false,"transaction":"7b22657373656e6365223a7b2274797065223a22526567756c6172222c2264617461223a7b22696e70757473223a5b7b2274797065223a225574786f222c2264617461223a223862656337666430613966646333353161646161663037663539356166656661373834346561666431383336323539343965353164636233623936333262383930303030227d5d2c226f757470757473223a5b7b2274797065223a225369676e61747572654c6f636b656453696e676c65222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2235653732303736303964323336333162373266333734663730303361626239373863326139303631633835613365356563633638393934653531366563323632227d2c22616d6f756e74223a3339353236347d7d2c7b2274797065223a225369676e61747572654c6f636b656453696e676c65222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2237363130613736633737643134303461636536396465376165333337376537366564313061386361303362636332366136333037666565313339303965333537227d2c22616d6f756e74223a383630343733367d7d2c7b2274797065223a225369676e61747572654c6f636b656444757374416c6c6f77616e6365222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2235653732303736303964323336333162373266333734663730303361626239373863326139303631633835613365356563633638393934653531366563323632227d2c22616d6f756e74223a313030303030307d7d5d2c227061796c6f6164223a7b2274797065223a22496e6465786174696f6e222c2264617461223a7b22696e646578223a5b38322c3131312c3131352c3130312c3131362c3131362c39375d2c2264617461223a5b5d7d7d7d7d2c22696e707574735f6d65746164617461223a7b223862656337666430613966646333353161646161663037663539356166656661373834346561666431383336323539343965353164636233623936333262383930303030223a7b226d6573736167654964223a2232663265346632643739636165353061656366396132363239326636393362313333356136393264316533653435323938336165656331363539363864616438222c227472616e73616374696f6e4964223a2238626563376664306139666463333531616461616630376635393561666566613738343465616664313833363235393439653531646362336239363332623839222c226f7574707574496e646578223a302c2269735370656e74223a66616c73652c226f7574707574223a7b2274797065223a302c2261646472657373223a7b2274797065223a302c2261646472657373223a2265613531346639643038663963386136656563323562353634303061323437616666393663373136333431363662623666336431356262346232303164323335227d2c22616d6f756e74223a31303030303030307d7d7d7d"}' | jq
```

`/construction/combine`
```
curl --request POST 'http://localhost:3030/construction/combine' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"unsigned_transaction":"7b22657373656e6365223a7b2274797065223a22526567756c6172222c2264617461223a7b22696e70757473223a5b7b2274797065223a225574786f222c2264617461223a223862656337666430613966646333353161646161663037663539356166656661373834346561666431383336323539343965353164636233623936333262383930303030227d5d2c226f757470757473223a5b7b2274797065223a225369676e61747572654c6f636b656453696e676c65222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2235653732303736303964323336333162373266333734663730303361626239373863326139303631633835613365356563633638393934653531366563323632227d2c22616d6f756e74223a3339353236347d7d2c7b2274797065223a225369676e61747572654c6f636b656453696e676c65222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2237363130613736633737643134303461636536396465376165333337376537366564313061386361303362636332366136333037666565313339303965333537227d2c22616d6f756e74223a383630343733367d7d2c7b2274797065223a225369676e61747572654c6f636b656444757374416c6c6f77616e6365222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2235653732303736303964323336333162373266333734663730303361626239373863326139303631633835613365356563633638393934653531366563323632227d2c22616d6f756e74223a313030303030307d7d5d2c227061796c6f6164223a7b2274797065223a22496e6465786174696f6e222c2264617461223a7b22696e646578223a5b38322c3131312c3131352c3130312c3131362c3131362c39375d2c2264617461223a5b5d7d7d7d7d2c22696e707574735f6d65746164617461223a7b223862656337666430613966646333353161646161663037663539356166656661373834346561666431383336323539343965353164636233623936333262383930303030223a7b226d6573736167654964223a2232663265346632643739636165353061656366396132363239326636393362313333356136393264316533653435323938336165656331363539363864616438222c227472616e73616374696f6e4964223a2238626563376664306139666463333531616461616630376635393561666566613738343465616664313833363235393439653531646362336239363332623839222c226f7574707574496e646578223a302c2269735370656e74223a66616c73652c226f7574707574223a7b2274797065223a302c2261646472657373223a7b2274797065223a302c2261646472657373223a2265613531346639643038663963386136656563323562353634303061323437616666393663373136333431363662623666336431356262346232303164323335227d2c22616d6f756e74223a31303030303030307d7d7d7d","signatures":[{"hex_bytes":"8040b3697a6e8c5de051c4e03fd3f3bccec99f3dba3ec303821919877dd0ed9b0e21f741bca8d86b8eb4d611754f94f51b73a5da928cafb01fcce8cc8f39a70a","signing_payload":{"address":"atoi1qr49znuapruu3fhwcfd4vsq2y3a0l9k8zc6pv6ak70g4hd9jq8fr2lqf6et","hex_bytes":"49f14e6ebc16aff834c86618832b8dc641bb80ff7ed0721c1d56259e6e26e884","account_identifier":{"address":"atoi1qr49znuapruu3fhwcfd4vsq2y3a0l9k8zc6pv6ak70g4hd9jq8fr2lqf6et"},"signature_type":"ed25519"},"public_key":{"hex_bytes":"0bcf06e9bff171f08f7cb8a8c551abead99f539421b84a3fcfd3609a5343b334","curve_type":"edwards25519"},"signature_type":"ed25519"}]}' | jq
```

`/construction/parse` (signed)
```
curl --request POST 'http://localhost:3030/construction/parse' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"signed":true,"transaction":"7b227472616e73616374696f6e223a7b22657373656e6365223a7b2274797065223a22526567756c6172222c2264617461223a7b22696e70757473223a5b7b2274797065223a225574786f222c2264617461223a223862656337666430613966646333353161646161663037663539356166656661373834346561666431383336323539343965353164636233623936333262383930303030227d5d2c226f757470757473223a5b7b2274797065223a225369676e61747572654c6f636b656453696e676c65222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2235653732303736303964323336333162373266333734663730303361626239373863326139303631633835613365356563633638393934653531366563323632227d2c22616d6f756e74223a3339353236347d7d2c7b2274797065223a225369676e61747572654c6f636b656453696e676c65222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2237363130613736633737643134303461636536396465376165333337376537366564313061386361303362636332366136333037666565313339303965333537227d2c22616d6f756e74223a383630343733367d7d2c7b2274797065223a225369676e61747572654c6f636b656444757374416c6c6f77616e6365222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2235653732303736303964323336333162373266333734663730303361626239373863326139303631633835613365356563633638393934653531366563323632227d2c22616d6f756e74223a313030303030307d7d5d2c227061796c6f6164223a7b2274797065223a22496e6465786174696f6e222c2264617461223a7b22696e646578223a5b38322c3131312c3131352c3130312c3131362c3131362c39375d2c2264617461223a5b5d7d7d7d7d2c22756e6c6f636b5f626c6f636b73223a5b7b2274797065223a225369676e6174757265222c2264617461223a7b2274797065223a2245643235353139222c2264617461223a7b227075626c69635f6b6579223a5b31312c3230372c362c3233332c3139312c3234312c3131332c3234302c3134332c3132342c3138342c3136382c3139372c38312c3137312c3233342c3231372c3135392c38332c3134382c33332c3138342c37342c36332c3230372c3231312c39362c3135342c38332c36372c3137392c35325d2c227369676e6174757265223a5b3132382c36342c3137392c3130352c3132322c3131302c3134302c39332c3232342c38312c3139362c3232342c36332c3231312c3234332c3138382c3230362c3230312c3135392c36312c3138362c36322c3139352c332c3133302c32352c32352c3133352c3132352c3230382c3233372c3135352c31342c33332c3234372c36352c3138382c3136382c3231362c3130372c3134322c3138302c3231342c31372c3131372c37392c3134382c3234352c32372c3131352c3136352c3231382c3134362c3134302c3137352c3137362c33312c3230342c3233322c3230342c3134332c35372c3136372c31305d7d7d7d5d7d2c22696e707574735f6d65746164617461223a7b223862656337666430613966646333353161646161663037663539356166656661373834346561666431383336323539343965353164636233623936333262383930303030223a7b226d6573736167654964223a2232663265346632643739636165353061656366396132363239326636393362313333356136393264316533653435323938336165656331363539363864616438222c227472616e73616374696f6e4964223a2238626563376664306139666463333531616461616630376635393561666566613738343465616664313833363235393439653531646362336239363332623839222c226f7574707574496e646578223a302c2269735370656e74223a66616c73652c226f7574707574223a7b2274797065223a302c2261646472657373223a7b2274797065223a302c2261646472657373223a2265613531346639643038663963386136656563323562353634303061323437616666393663373136333431363662623666336431356262346232303164323335227d2c22616d6f756e74223a31303030303030307d7d7d7d"}' | jq
```

`/construction/hash`
```
curl --request POST 'http://localhost:3030/construction/hash' \--header 'Accept: application/json' \--header 'Content-Type: application/json' \--data-raw '{"network_identifier":{"blockchain":"iota","network":"chrysalis-devnet"},"signed_transaction":"7b227472616e73616374696f6e223a7b22657373656e6365223a7b2274797065223a22526567756c6172222c2264617461223a7b22696e70757473223a5b7b2274797065223a225574786f222c2264617461223a223862656337666430613966646333353161646161663037663539356166656661373834346561666431383336323539343965353164636233623936333262383930303030227d5d2c226f757470757473223a5b7b2274797065223a225369676e61747572654c6f636b656453696e676c65222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2235653732303736303964323336333162373266333734663730303361626239373863326139303631633835613365356563633638393934653531366563323632227d2c22616d6f756e74223a3339353236347d7d2c7b2274797065223a225369676e61747572654c6f636b656453696e676c65222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2237363130613736633737643134303461636536396465376165333337376537366564313061386361303362636332366136333037666565313339303965333537227d2c22616d6f756e74223a383630343733367d7d2c7b2274797065223a225369676e61747572654c6f636b656444757374416c6c6f77616e6365222c2264617461223a7b2261646472657373223a7b2274797065223a2245643235353139222c2264617461223a2235653732303736303964323336333162373266333734663730303361626239373863326139303631633835613365356563633638393934653531366563323632227d2c22616d6f756e74223a313030303030307d7d5d2c227061796c6f6164223a7b2274797065223a22496e6465786174696f6e222c2264617461223a7b22696e646578223a5b38322c3131312c3131352c3130312c3131362c3131362c39375d2c2264617461223a5b5d7d7d7d7d2c22756e6c6f636b5f626c6f636b73223a5b7b2274797065223a225369676e6174757265222c2264617461223a7b2274797065223a2245643235353139222c2264617461223a7b227075626c69635f6b6579223a5b31312c3230372c362c3233332c3139312c3234312c3131332c3234302c3134332c3132342c3138342c3136382c3139372c38312c3137312c3233342c3231372c3135392c38332c3134382c33332c3138342c37342c36332c3230372c3231312c39362c3135342c38332c36372c3137392c35325d2c227369676e6174757265223a5b3132382c36342c3137392c3130352c3132322c3131302c3134302c39332c3232342c38312c3139362c3232342c36332c3231312c3234332c3138382c3230362c3230312c3135392c36312c3138362c36322c3139352c332c3133302c32352c32352c3133352c3132352c3230382c3233372c3135352c31342c33332c3234372c36352c3138382c3136382c3231362c3130372c3134322c3138302c3231342c31372c3131372c37392c3134382c3234352c32372c3131352c3136352c3231382c3134362c3134302c3137352c3137362c33312c3230342c3233322c3230342c3134332c35372c3136372c31305d7d7d7d5d7d2c22696e707574735f6d65746164617461223a7b223862656337666430613966646333353161646161663037663539356166656661373834346561666431383336323539343965353164636233623936333262383930303030223a7b226d6573736167654964223a2232663265346632643739636165353061656366396132363239326636393362313333356136393264316533653435323938336165656331363539363864616438222c227472616e73616374696f6e4964223a2238626563376664306139666463333531616461616630376635393561666566613738343465616664313833363235393439653531646362336239363332623839222c226f7574707574496e646578223a302c2269735370656e74223a66616c73652c226f7574707574223a7b2274797065223a302c2261646472657373223a7b2274797065223a302c2261646472657373223a2265613531346639643038663963386136656563323562353634303061323437616666393663373136333431363662623666336431356262346232303164323335227d2c22616d6f756e74223a31303030303030307d7d7d7d"}' | jq
```