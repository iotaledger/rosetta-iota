use warp::Filter;

use std::net::SocketAddr;

pub async fn start_mocknet_node() {

    let bind_addr = "127.0.0.1:3029".to_string().parse::<SocketAddr>().expect("unable to parse socket address");

    println!("Mocknet node listening on {}.", bind_addr.to_string());

    let health = warp::path!("health").map(|| "");

    let node_info = warp::path!("api" / "v1" / "info").map(|| {
        r#"{"data":{"name":"HORNET","version":"0.6.0-alpha","isHealthy":true,"networkId":"testnet7","bech32HRP":"atoi","minPoWScore":4000,"messagesPerSecond":32.9,"referencedMessagesPerSecond":39.1,"referencedRate":118.84498480243163,"latestMilestoneTimestamp":1618486402,"latestMilestoneIndex":68910,"confirmedMilestoneIndex":68910,"pruningIndex":51391,"features":["PoW"]}}"#
    });

    let milestones = warp::path!("api" / "v1" / "milestones" / u32).map(|milestone_index| {
        if milestone_index == 68910 {
            r#"{"data":{"index":68910,"messageId":"339a467c3f950e28381aaef84aa82f3f650e6284574b156ccc1e574eb77afcac","timestamp":1618486402}}"#
        } else if milestone_index == 68910 -1 {
            r#"{"data":{"index":68909,"messageId":"8489917555634d94da2c5fa208fe9bc0a90a1cb03528147e43bc0b286e78b59d","timestamp":1618486392}}"#
        } else {
            unimplemented!()
        }
    });

    let utxo_changes = warp::path!("api" / "v1" / "milestones" / u32 / "utxo-changes").map(|milestone_index| {
        if milestone_index == 68910 {
            r#"{"data":{"index":68910,"createdOutputs":["6c1f317ed905c17710ea81af0a1183f0f8e93600208e7f38330da04b91c85b2d0000"],"consumedOutputs":["95535a4cc1976149d11a6e0b988118d1de435c50481749e351ef835d705ab1e70c00"]}}"#
        } else {
            unimplemented!()
        }
    });

    let outputs = warp::path!("api" / "v1" / "outputs" / String).map(|output_id| {
        if output_id == "6c1f317ed905c17710ea81af0a1183f0f8e93600208e7f38330da04b91c85b2d0000" {
            r#"{"data":{"messageId":"1f7af3dfb1582d189e435983a00ecc2585327b22e7074721a37ee7f8cbfdc393","transactionId":"6c1f317ed905c17710ea81af0a1183f0f8e93600208e7f38330da04b91c85b2d","outputIndex":0,"isSpent":false,"output":{"type":0,"address":{"type":0,"address":"8392c22e28144506cb505bac55d4efa6b1ecb4988ec9ce426ba2b7d5c88c67a7"},"amount":10000000}}}"#
        } else if output_id == "95535a4cc1976149d11a6e0b988118d1de435c50481749e351ef835d705ab1e70c00" {
            r#"{"data":{"messageId":"70a9a9bc408121b766cc20d9a5b8dba0829e41244c500b2d04cf34f1f20f4621","transactionId":"95535a4cc1976149d11a6e0b988118d1de435c50481749e351ef835d705ab1e7","outputIndex":12,"isSpent":true,"output":{"type":0,"address":{"type":0,"address":"d6b3d2cc85f3b82ce9d9e8073b775c46482282dd83411bc554fe12bc41ca573f"},"amount":10000000}}}"#
        } else if output_id == "95535a4cc1976149d11a6e0b988118d1de435c50481749e351ef835d705ab1e70c00" {
            r#"{"data":{"messageId":"70a9a9bc408121b766cc20d9a5b8dba0829e41244c500b2d04cf34f1f20f4621","transactionId":"95535a4cc1976149d11a6e0b988118d1de435c50481749e351ef835d705ab1e7","outputIndex":12,"isSpent":true,"output":{"type":0,"address":{"type":0,"address":"d6b3d2cc85f3b82ce9d9e8073b775c46482282dd83411bc554fe12bc41ca573f"},"amount":10000000}}}"#
        } else if output_id == "f3a53f04402be2f59634ee9b073898c84d2e08b4ba06046d440b1ac27bc5ded60000" {
            r#"{"data":{"messageId":"bb3c9744006d3b92897f9ff446bc983db28caa63e45aeaa4388ea4c11264a620","transactionId":"f3a53f04402be2f59634ee9b073898c84d2e08b4ba06046d440b1ac27bc5ded6","outputIndex":0,"isSpent":true,"output":{"type":0,"address":{"type":0,"address":"426d1f47b88952fb312776234a2d0d7bbcfe5fbd4daee3d068c6b42a899d94b9"},"amount":10000000}}}"#
        } else {
            unimplemented!()
        }
    });

    let addresses = warp::path!("api" / "v1" / "addresses" / String).map(|address| {
        if address == "atoi1qppx6868hzy497e3yamzxj3dp4ameljlh4x6ac7sdrrtg25fnk2tjlpxcek" {
            r#"{"data":{"addressType":0,"address":"426d1f47b88952fb312776234a2d0d7bbcfe5fbd4daee3d068c6b42a899d94b9","balance":11000000,"dustAllowed":false}}"#
        } else {
            unimplemented!()
        }
    });

    let outputs_for_address = warp::path!("api" / "v1" / "addresses" / String / "outputs").map(|address| {
        if address == "atoi1qppx6868hzy497e3yamzxj3dp4ameljlh4x6ac7sdrrtg25fnk2tjlpxcek" {
            r#"{"data":{"addressType":0,"address":"426d1f47b88952fb312776234a2d0d7bbcfe5fbd4daee3d068c6b42a899d94b9","maxResults":1000,"count":1,"outputIds":["f3a53f04402be2f59634ee9b073898c84d2e08b4ba06046d440b1ac27bc5ded60000"]}}"#
        } else {
            unimplemented!()
        }
    });

    let message = warp::path!("api" / "v1" / "messages" / String).map(|message_id| {
        if message_id == "1f7af3dfb1582d189e435983a00ecc2585327b22e7074721a37ee7f8cbfdc393" {
            r#"{"data":{"networkId":"14379272398717627559","parentMessageIds":["3e69f2d115293a33a1cd78e04d6a4ac39030310bd7ce4ec491fcbcdcb45afc49","a892576558d6dd078c886a035b68e8c7c229bf8e2a522d8de84e9c92726ec3db","de990411da55a744df215ba98f5af17533d05b47ec5c1916984f6f79f69295e0","fddb0444ca5f295dd6d7c5b966c94f5b3b304f611d99b83bb2c83f43ab65a0a8"],"payload":{"type":0,"essence":{"type":0,"inputs":[{"type":0,"transactionId":"95535a4cc1976149d11a6e0b988118d1de435c50481749e351ef835d705ab1e7","transactionOutputIndex":12}],"outputs":[{"type":0,"address":{"type":0,"address":"8392c22e28144506cb505bac55d4efa6b1ecb4988ec9ce426ba2b7d5c88c67a7"},"amount":10000000}],"payload":{"type":2,"index":"464155434554","data":""}},"unlockBlocks":[{"type":0,"signature":{"type":0,"publicKey":"25784766f4645412e615743db6d1027a83b470e6968464862a520f30f5a4c852","signature":"1580c145445aa3a876ca40d4b4666d7daf833126b5d682de4e0247dbdb4c2cff2586859f2fe63b60968a2eec56ed910443715425b616b7fc7a0cd5ef3fc9e707"}}]},"nonce":"189712"}}"#
        } else if message_id == "70a9a9bc408121b766cc20d9a5b8dba0829e41244c500b2d04cf34f1f20f4621" {
            r#"{"data":{"networkId":"14379272398717627559","parentMessageIds":["542d6f263b1d3917a40b09b844984a4a7f7800ef6fd2b52500df80880c0ac1d9","7f3821b3f6429a596fe2f27efef99aa3ca43320bb666cfe32c6d81aceb271700","9589741dfb651adc6c240f326d75cf1bfe92fcb7cbaf203cd7c5b1a5df2a1f0d","e7cd5c9d504976b615d45effa797bf3847d26b64841aec71e39f86e25c633acb"],"payload":{"type":0,"essence":{"type":0,"inputs":[{"type":0,"transactionId":"dd2bd7ef1d67a6247823416337c2938a26ca91322ce89674a8795ac7b4072d2f","transactionOutputIndex":13}],"outputs":[{"type":0,"address":{"type":0,"address":"14495a157f4bfb82e99dad269823cac7eea9c28bf7ae8e3d8ac3d748d5f0f871"},"amount":10000000},{"type":0,"address":{"type":0,"address":"1a99ca850eaf419acfb043501b94bfa36aff225f2775dd93936c92d5712e747b"},"amount":10000000},{"type":0,"address":{"type":0,"address":"3a3a93a3fa38cd9b3e850f64bee11d17c3eb645b290a4b0a1aa7cdf5855c5216"},"amount":10000000},{"type":0,"address":{"type":0,"address":"4aa1130ce85e5ca427a379f3cd061f93f5e059aa6a65c02780cf048254aff67e"},"amount":10000000},{"type":0,"address":{"type":0,"address":"4cacc90985df6589da267451c844481feb2184ab5333cd40a4d017856c77439b"},"amount":10000000},{"type":0,"address":{"type":0,"address":"7357c7ac6a8c2019ee1dedd1d2ac78545367a2dd10ce2620100d24880ba97404"},"amount":10000000},{"type":0,"address":{"type":0,"address":"73bd5034a902eb2f77b3687908a8c7e184e114b866b7fc87fb34aa977a70a373"},"amount":10000000},{"type":0,"address":{"type":0,"address":"80598b362fc1d5c7a350604794e7f7826d4041dbfb734607fe15695212c8abe4"},"amount":10000000},{"type":0,"address":{"type":0,"address":"9eaa14137fbe57d47d5f959ec03a16abb863e6dee6891cef5ac16564cc8d2051"},"amount":10000000},{"type":0,"address":{"type":0,"address":"a3a4afeb121479b838071622f3a0a3a371403cee58ad9144b7c2776f64d52008"},"amount":10000000},{"type":0,"address":{"type":0,"address":"a983b259d839fa7f10b1cb83a3e8f007290b1623fd3e1bcaeb7e31638d6b70e3"},"amount":10000000},{"type":0,"address":{"type":0,"address":"b67617ea177376281e5fb865c686fecd17c342b98546254a8659333dfee9ba80"},"amount":10000000},{"type":0,"address":{"type":0,"address":"d6b3d2cc85f3b82ce9d9e8073b775c46482282dd83411bc554fe12bc41ca573f"},"amount":10000000},{"type":0,"address":{"type":0,"address":"eda2fcdc37e4d4640bf2f9862da9b46e360fe4f0c42d51d4f2f3a68d31c98309"},"amount":100796083521054},{"type":0,"address":{"type":0,"address":"f3fa64cb5a1e11b420492db46cd8774c5213c629fac7d98a42f1b1964675ff73"},"amount":10000000},{"type":0,"address":{"type":0,"address":"f616c401d9eed517cc367665e2b90aa4e9c0a3cc2fee3b3b6f5eb76a2afc25b1"},"amount":10000000}],"payload":null},"unlockBlocks":[{"type":0,"signature":{"type":0,"publicKey":"35e5e3c0ad7a7b31837f7ed521f9acddf2381c8e1bad3c78107dec898b690a92","signature":"a5e0b6fc52a751c2b5d8b64acbd20e538e80de0e0b6646a2eb3091793f1e050458d11384d3185b94614ce22d1c9e23c5a909c0c3adbca037e705a236ae1a2f09"}}]},"nonce":"4611686018427745760"}}"#
        } else {
            unimplemented!()
        }
    });

    let peers = warp::path!("api" / "v1" / "peers").map(|| {
        r#"{"data":[{"id":"A","multiAddresses":["/dns/testnet.chrysalis2.com/tcp/15600"],"alias":"A","relation":"known","connected":true,"gossip":{"heartbeat":{"solidMilestoneIndex":69082,"prunedMilestoneIndex":0,"latestMilestoneIndex":69082,"connectedNeighbors":1,"syncedNeighbors":8},"metrics":{"newMessages":10139612,"knownMessages":1210097,"receivedMessages":11538518,"receivedMessageRequests":0,"receivedMilestoneRequests":1,"receivedHeartbeats":54797,"sentMessages":564098,"sentMessageRequests":2758,"sentMilestoneRequests":9,"sentHeartbeats":54807,"droppedPackets":0}}}]}"#
    });

    let routes = health.or(node_info.or(milestones.or(utxo_changes.or(outputs.or(message.or(addresses.or(outputs_for_address.or(peers))))))));

    let shutdown = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(bind_addr, shutdown);

    server.await;

    println!("Mocknet node stopped.");
}