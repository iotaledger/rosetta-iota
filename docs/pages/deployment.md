## IOTA nodes

**IOTA full-nodes** (such as [HORNET](https://github.com/gohornet/hornet.git) and [Bee](https://github.com/iotaledger/bee.git)) are able to start up from a recent block instead of having to synchronize from genesis. This can be achieved by bootstrapping the node with recent [snapshots](https://github.com/luca-moser/protocol-rfcs/blob/local-snapshot-file-format/text/0000-local-snapshot-file-format/0000-local-snapshot-file-format.md). 
Also, IOTA full-nodes are able to prune history from time to time in a safe way.
**IOTA full-nodes per default don't hold the entire Tangle history - they are not designed for it.**

Only **IOTA Permanodes** (such as [Chronicle](https://github.com/iotaledger/chronicle.rs/tree/main/chronicle-node)) **are designed to hold the entire Tangle history.**

`rosetta-iota` aims for a more reliable integration and better performance with limiting state storage. For these reasons **the Rosetta API implementation will be deployed with an IOTA full-node** ([HORNET](https://github.com/gohornet/hornet.git)).

## Testnet deployment

1) Ensure `docker` and `docker-compose` are installed.
2) Download the latest release of `rosetta-iota` and extract the files in a folder of your choice.
3) Add your HORNET peers (to which your HORNET node should connect) to following configuration file: `hornet/peering.json`. If you don't have any peers, please contact us and we will help you.
4) **Run following commands to start a HORNET node together with a Rosetta API instance:**

    **Testnet:Online**
    ```
    NETWORK=testnet7 BECH32_HRP=atoi TX_TAG=Rosetta MODE=online docker-compose up
    ```

    **Testnet:Offline**
    ```
    NETWORK=testnet7 BECH32_HRP=atoi TX_TAG=Rosetta MODE=offline docker-compose up
    ```
Once the HORNET node has synced with the network, the Rosetta API will be available at: http://localhost:3030
The REST API of the HORNET node will be available at: http://localhost:14265/api/v1/info

The HORNET node will be bootstrapped automatically with recent snapshots to start synchronizing from a recent block. The snapshots will be automatically downloaded from https://dbfiles.testnet.chrysalis2.com. **If you want to bootstrap the HORNET node yourself with custom snapshots, put them in the `data/snapshots/` directory.**

