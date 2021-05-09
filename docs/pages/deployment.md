# Deployment

## IOTA nodes

**IOTA full-nodes** (such as [HORNET](https://github.com/gohornet/hornet) and [Bee](https://github.com/iotaledger/bee)) are able to start up from a more recent block instead of having to synchronize from genesis. This can be achieved by bootstrapping the IOTA full-node with recent [snapshots](https://github.com/luca-moser/protocol-rfcs/blob/local-snapshot-file-format/text/0000-local-snapshot-file-format/0000-local-snapshot-file-format.md). 
In addition, IOTA full-nodes are able to prune history from time to time in a safe way.
**IOTA full-nodes per default don't hold the entire history - they are not designed to do so.**

In contrast, **IOTA Permanodes** (such as [Chronicle](https://github.com/iotaledger/chronicle.rs)) **are designed to hold the entire history.**

`rosetta-iota` aims for a more reliable integration and better performance with limiting state storage. Therefore, **the Rosetta API implementation is deployed with an IOTA full-node** ([HORNET](https://github.com/gohornet/hornet.git)).

## Instructions

**Following instructions will start a HORNET node together with a Rosetta API instance:**

1) Ensure `docker` and `docker-compose` are installed.
2) Download the latest release of `rosetta-iota` and extract the files in a folder of your choice.
3) Add your peer(s) - to which your HORNET node should connect. For `chrysalis-mainnet`, add the peers to the `hornet/chrysalis-mainnet/peering.json` file. For `testnet7`, add the peers to the `hornet/testnet7/peering.json` file. If you don't have any peers, please contact us and we will help you find some.

    For better illustration, the `peering.json` file could then look like the following, for example:
    ```json
   {
     "peers": [
       {
         "multiAddress": "/dns/xyz.com/tcp/15600/p2p/12D3KooWEVGFchjYqCH1nAWt2norb6sJYEedmEkPofoNiyDwyptf",
         "alias": "Alice"
       },
       {
         "multiAddress": "/dns/121.56.12.23/tcp/15600/p2p/12D3KooWRNYKZXYqZngxQee5BefmzcW5Zk6Tc6iE92U2uZwArHw9",
         "alias": "Bob"
       }
     ]
   }
   ```
   
    Also, make sure that you tell your peers the multiaddress of your HORNET node so that they will be able to mutually tether. A multiaddress - as illustrated above - consists of the **address where you deploy the node and the node ID**. You can find your node ID in the logs when you start the implementation.
    
4) Run the implementation in the desired mode:

    **chrysalis-mainnet: online mode**
    ```
    MODE=online TX_TAG=Rosetta docker-compose -f docker-compose.chrysalis-mainnet.yml up
    ```
    
    **chrysalis-mainnet: offline mode**
    ```
    MODE=offline TX_TAG=Rosetta docker-compose -f docker-compose.chrysalis-mainnet.yml up
    ```
    
    **testnet7: online mode**
    ```
    MODE=online TX_TAG=Rosetta docker-compose -f docker-compose.testnet7.yml up
    ```
    
    **testnet7: offline mode**
    ```
    MODE=offline TX_TAG=Rosetta docker-compose -f docker-compose.testnet7.yml up
    ```

5) Since the node ID is tied with a private key make sure you back up the private key. Otherwise, you cannot preserve the same node ID for subsequent deployments. 
You can back up your node ID by preserving the `data/p2pstore` directory.


Once the HORNET node has synced with the network, the Rosetta API will be available at: http://localhost:3030
The health status of the HORNET node can be checked at: http://localhost:14265/api/v1/info

#### Environment variables:
- `MODE` ... the mode in which the implementation is to run. Can be either `offline` or `online`.
- `TX_TAG` ... the tag that transactions created by the Construction API should carry. If no tag is specified, the tag `Rosetta` is used by default. Constructed transactions can then be found by this tag. The tag can be a maximum of 64 bytes long.

## Further notes:

The HORNET node will be bootstrapped automatically with recent snapshots to start synchronizing from a recent block. **For `chrysalis-mainnet`, the snapshots will be automatically downloaded from https://chrysalis-dbfiles.iota.org. For `testnet7`, the snapshots will be automatically downloaded from https://dbfiles.testnet.chrysalis2.com. If you want to bootstrap the HORNET node yourself, you can do so by placing your snapshots appropriately in the `data/snapshots/` directory.**

