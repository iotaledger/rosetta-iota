# Deployment

## IOTA nodes

**IOTA full-nodes** (such as [HORNET](https://github.com/gohornet/hornet) and [Bee](https://github.com/iotaledger/bee)) are able to start up from a more recent block instead of having to synchronize from genesis. This can be achieved by bootstrapping the IOTA full-node with recent [snapshots](https://github.com/luca-moser/protocol-rfcs/blob/local-snapshot-file-format/text/0000-local-snapshot-file-format/0000-local-snapshot-file-format.md). 
In addition, IOTA full-nodes are able to prune history from time to time in a safe way.
**IOTA full-nodes per default don't hold the entire history - they are not designed to do so.**

In contrast, **IOTA Permanodes** (such as [Chronicle](https://github.com/iotaledger/chronicle.rs)) **are designed to hold the entire history.**

`rosetta-iota` aims for a more reliable integration and better performance with limiting state storage. For these reasons, **the Rosetta API implementation is deployed with an IOTA full-node** ([HORNET](https://github.com/gohornet/hornet.git)).

## Testnet deployment

**Following instructions will start a HORNET node together with a Rosetta API instance:**

1) Ensure `docker` and `docker-compose` are installed.
2) Download the latest release of `rosetta-iota` and extract the files in a folder of your choice.
3) Add your HORNET peer(s) - to which your HORNET node should connect - to following configuration file: `hornet/peering_testnet.json`. If you don't have any peers, please contact us and we will help you find some.
4) Run the implementation in the desired mode:

**Testnet: online mode**
```
MODE=online TX_TAG=Rosetta docker-compose -f docker-compose.testnet7.yml up
```

**Testnet: offline mode**
```
MODE=offline TX_TAG=Rosetta docker-compose -f docker-compose.testnet7.yml up
```

Once the HORNET node has synced with the network, the Rosetta API will be available at: http://localhost:3030

The health status of the HORNET node can be checked at: http://localhost:14265/api/v1/info

The HORNET node will be bootstrapped automatically with recent snapshots to start synchronizing from a recent block. **The snapshots will be automatically downloaded from https://dbfiles.testnet.chrysalis2.com. If you want to bootstrap the HORNET node yourself, put your snapshots in the `data/snapshots/testnet` directory.**

### Environment variables:
- `MODE` ... the mode in which the implementation is to run. Can be either `offline` or `online`.
- `TX_TAG` ... the tag that transactions created by the Construction API should carry. If no tag is specified, the tag `Rosetta` is used by default. Constructed transactions can then be found by this tag. The tag can be a maximum of 64 bytes long.