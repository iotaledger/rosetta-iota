// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use tokio::{
    sync::{
        oneshot,
        oneshot::{Receiver, Sender},
    },
    time::Duration,
};
use warp::Filter;

use crate::config::DUMMY_NODE_BIND_ADDR;

use std::net::SocketAddr;

pub async fn start_dummy_node() -> DummyNodeHandle {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let (return_tx, return_rx) = oneshot::channel();
    tokio::task::spawn(run_server(shutdown_rx, return_tx));
    // sleep some time to make sure the dummy node is up
    tokio::time::sleep(Duration::from_millis(100)).await;
    DummyNodeHandle { shutdown_tx, return_rx }
}

pub struct DummyNodeHandle {
    shutdown_tx: Sender<()>,
    return_rx: Receiver<()>,
}

impl DummyNodeHandle {
    pub async fn shutdown(self) {
        self.shutdown_tx
            .send(())
            .expect("can not send shutdown signal to dummy node");
        self.return_rx.await.expect("can not wait for the dummy node to return")
    }
}

async fn run_server(shutdown_rx: Receiver<()>, return_tx: Sender<()>) {
    let bind_addr = DUMMY_NODE_BIND_ADDR.to_string().parse::<SocketAddr>().unwrap();

    let info = warp::path!("api" / "v1" / "info").map(|| {
        r#"{"data":{"name":"HORNET","version":"1.0.5","isHealthy":true,"networkId":"chrysalis-mainnet","bech32HRP":"iota","minPoWScore":4000,"messagesPerSecond":12,"referencedMessagesPerSecond":11,"referencedRate":91.66666666666666,"latestMilestoneTimestamp":1635331891,"latestMilestoneIndex":1438449,"confirmedMilestoneIndex":1438448,"pruningIndex":1438000,"features":["PoW"]}}"#
    });

    let address = warp::path!("api" / "v1" / "addresses" / String).map(|address| {
        if address == "iota1qp6gwwy7rruk0d3j9fqzcxnfrstfedk2m65jst2tx7xmkad4agjc5r7ptjz" {
            r#"{"data":{"addressType":0,"address":"7487389e18f967b6322a402c1a691c169cb6cadea9282d4b378dbb75b5ea258a","balance":20651169480,"dustAllowed":false,"ledgerIndex":1438441}}"#
        } else {
            unimplemented!()
        }
    });

    let address_outputs = warp::path!("api" / "v1" / "addresses" / String / "outputs").map(|address| {
        if address == "iota1qp6gwwy7rruk0d3j9fqzcxnfrstfedk2m65jst2tx7xmkad4agjc5r7ptjz" {
            r#"{"data":{"addressType":0,"address":"7487389e18f967b6322a402c1a691c169cb6cadea9282d4b378dbb75b5ea258a","maxResults":1000,"count":1,"outputIds":["d2e2faaf394a5d22045668a55df27c9abe2057c1f2ce319999bed373269b50190000"],"ledgerIndex":1438495}}"#
        } else {
            unimplemented!()
        }
    });

    let outputs = warp::path!("api" / "v1" / "outputs" / String).map(|output_id| {
        if output_id == "d2e2faaf394a5d22045668a55df27c9abe2057c1f2ce319999bed373269b50190000" {
            r#"{"data":{"messageId":"1c7a3c3f262dc1bb75adf4709d73cb73d6de7c3616ed6fa6ae44efbb38ea522b","transactionId":"d2e2faaf394a5d22045668a55df27c9abe2057c1f2ce319999bed373269b5019","outputIndex":0,"isSpent":false,"ledgerIndex":1438614,"output":{"type":0,"address":{"type":0,"address":"7487389e18f967b6322a402c1a691c169cb6cadea9282d4b378dbb75b5ea258a"},"amount":20651169480}}}"#
        } else {
            unimplemented!()
        }
    });

    let milestones = warp::path!("api" / "v1" / "milestones" / String).map(|index| {
        if index == "1438448" {
            r#"{"data":{"index":1438448,"messageId":"8abc7c6b1b871a2bf6c5112d9bd0d7f310176fbe35127be269006bc1becc94e4","timestamp":1634052071}}"#
        } else {
            unimplemented!()
        }
    });

    let utxo_changes = warp::path!("api" / "v1" / "milestones" / u32 / "utxo-changes").map(|milestone_index| {
        if milestone_index == 1438448 {
            r#"{"data":{"index":68910,"createdOutputs":["d2e2faaf394a5d22045668a55df27c9abe2057c1f2ce319999bed373269b50190000"],"consumedOutputs":["95535a4cc1976149d11a6e0b988118d1de435c50481749e351ef835d705ab1e70c00"]}}"#
        } else {
            unimplemented!()
        }
    });

    let peers = warp::path!("api" / "v1" / "peers").map(|| {
        r#"{"data":[{"id":"A","multiAddresses":["/dns/chrysalis-nodes.iota.org/tcp/15600"],"alias":"A","relation":"known","connected":true,"gossip":{"heartbeat":{"solidMilestoneIndex":69082,"prunedMilestoneIndex":0,"latestMilestoneIndex":69082,"connectedNeighbors":1,"syncedNeighbors":8},"metrics":{"newMessages":10139612,"knownMessages":1210097,"receivedMessages":11538518,"receivedMessageRequests":0,"receivedMilestoneRequests":1,"receivedHeartbeats":54797,"sentMessages":564098,"sentMessageRequests":2758,"sentMilestoneRequests":9,"sentHeartbeats":54807,"droppedPackets":0}}}]}"#
    });

    let messages = warp::path!("api" / "v1" / "messages" / String).map(|message_id| {
        if message_id == "1c7a3c3f262dc1bb75adf4709d73cb73d6de7c3616ed6fa6ae44efbb38ea522b" || message_id == "70a9a9bc408121b766cc20d9a5b8dba0829e41244c500b2d04cf34f1f20f4621" {
            r#"{"data":{"networkId":"14379272398717627559","parentMessageIds":["542d6f263b1d3917a40b09b844984a4a7f7800ef6fd2b52500df80880c0ac1d9","7f3821b3f6429a596fe2f27efef99aa3ca43320bb666cfe32c6d81aceb271700","9589741dfb651adc6c240f326d75cf1bfe92fcb7cbaf203cd7c5b1a5df2a1f0d","e7cd5c9d504976b615d45effa797bf3847d26b64841aec71e39f86e25c633acb"],"payload":{"type":0,"essence":{"type":0,"inputs":[{"type":0,"transactionId":"dd2bd7ef1d67a6247823416337c2938a26ca91322ce89674a8795ac7b4072d2f","transactionOutputIndex":13}],"outputs":[{"type":0,"address":{"type":0,"address":"14495a157f4bfb82e99dad269823cac7eea9c28bf7ae8e3d8ac3d748d5f0f871"},"amount":10000000},{"type":0,"address":{"type":0,"address":"1a99ca850eaf419acfb043501b94bfa36aff225f2775dd93936c92d5712e747b"},"amount":10000000},{"type":0,"address":{"type":0,"address":"3a3a93a3fa38cd9b3e850f64bee11d17c3eb645b290a4b0a1aa7cdf5855c5216"},"amount":10000000},{"type":0,"address":{"type":0,"address":"4aa1130ce85e5ca427a379f3cd061f93f5e059aa6a65c02780cf048254aff67e"},"amount":10000000},{"type":0,"address":{"type":0,"address":"4cacc90985df6589da267451c844481feb2184ab5333cd40a4d017856c77439b"},"amount":10000000},{"type":0,"address":{"type":0,"address":"7357c7ac6a8c2019ee1dedd1d2ac78545367a2dd10ce2620100d24880ba97404"},"amount":10000000},{"type":0,"address":{"type":0,"address":"73bd5034a902eb2f77b3687908a8c7e184e114b866b7fc87fb34aa977a70a373"},"amount":10000000},{"type":0,"address":{"type":0,"address":"80598b362fc1d5c7a350604794e7f7826d4041dbfb734607fe15695212c8abe4"},"amount":10000000},{"type":0,"address":{"type":0,"address":"9eaa14137fbe57d47d5f959ec03a16abb863e6dee6891cef5ac16564cc8d2051"},"amount":10000000},{"type":0,"address":{"type":0,"address":"a3a4afeb121479b838071622f3a0a3a371403cee58ad9144b7c2776f64d52008"},"amount":10000000},{"type":0,"address":{"type":0,"address":"a983b259d839fa7f10b1cb83a3e8f007290b1623fd3e1bcaeb7e31638d6b70e3"},"amount":10000000},{"type":0,"address":{"type":0,"address":"b67617ea177376281e5fb865c686fecd17c342b98546254a8659333dfee9ba80"},"amount":10000000},{"type":0,"address":{"type":0,"address":"d6b3d2cc85f3b82ce9d9e8073b775c46482282dd83411bc554fe12bc41ca573f"},"amount":10000000},{"type":0,"address":{"type":0,"address":"eda2fcdc37e4d4640bf2f9862da9b46e360fe4f0c42d51d4f2f3a68d31c98309"},"amount":100796083521054},{"type":0,"address":{"type":0,"address":"f3fa64cb5a1e11b420492db46cd8774c5213c629fac7d98a42f1b1964675ff73"},"amount":10000000},{"type":0,"address":{"type":0,"address":"f616c401d9eed517cc367665e2b90aa4e9c0a3cc2fee3b3b6f5eb76a2afc25b1"},"amount":10000000}],"payload":null},"unlockBlocks":[{"type":0,"signature":{"type":0,"publicKey":"35e5e3c0ad7a7b31837f7ed521f9acddf2381c8e1bad3c78107dec898b690a92","signature":"a5e0b6fc52a751c2b5d8b64acbd20e538e80de0e0b6646a2eb3091793f1e050458d11384d3185b94614ce22d1c9e23c5a909c0c3adbca037e705a236ae1a2f09"}}]},"nonce":"4611686018427745760"}}"#
        } else {
            unimplemented!()
        }
    });

    let routes =
        info.or(address.or(address_outputs.or(outputs.or(milestones.or(peers.or(utxo_changes.or(messages)))))));

    println!("binding dummy node at {}", bind_addr);

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(bind_addr, async {
        shutdown_rx.await.ok();
    });

    server.await;

    println!("dummy node stopped");

    let _ = return_tx.send(());
}
