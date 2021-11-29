# Deployment

## About IOTA nodes

**IOTA full-nodes** (such as [HORNET](https://github.com/gohornet/hornet) and [Bee](https://github.com/iotaledger/bee)) are able to start up from a more recent block instead of having to synchronize from genesis. This is made possible by booting the IOTA full-node with recent [snapshots](https://github.com/luca-moser/protocol-rfcs/blob/local-snapshot-file-format/text/0000-local-snapshot-file-format/0000-local-snapshot-file-format.md). 
Also, by default, IOTA full-nodes prune history from time to time in a safe way - basically similar to nodes in the Bitcoin network that are running in pruning mode.

In contrast, **IOTA Permanodes** (such as [Chronicle](https://github.com/iotaledger/chronicle.rs)) are optimized for storing IOTA history which dates back further and provide special tools for querying data.

`rosetta-iota` aims for a more reliable integration and better performance with limiting state storage. Therefore, **the Rosetta API implementation is delivered with an IOTA full-node** ([HORNET](https://github.com/gohornet/hornet.git)).

## Instructions

**Following instructions will start a HORNET node together with a Rosetta API instance:**

1) Ensure `docker` and `docker-compose` are installed.
2) Download the [latest release](https://github.com/iotaledger/rosetta-iota/releases) of `rosetta-iota` and extract `rosetta-iota` to a folder of your choice.
3) **The docker image runs under user with user id 65532 and group id 65532.** To make sure there are no permission issues with the HORNET node, prepare the working directories so that HORNET can read and write data to the host system:
    ```
    cd rosetta-iota
    mkdir -p data/storage/chrysalis-mainnet && chown 65532:65532 data/storage/chrysalis-mainnet
    mkdir -p data/storage/chrysalis-devnet && chown 65532:65532 data/storage/chrysalis-devnet
    mkdir -p data/snapshots/chrysalis-mainnet && chown 65532:65532 data/snapshots/chrysalis-mainnet
    mkdir -p data/snapshots/chrysalis-devnet && chown 65532:65532 data/snapshots/chrysalis-devnet
    mkdir -p data/p2pstore && chown 65532:65532 data/p2pstore
   ```
4) **Optionally**: if you want your node to use an already existing node ID, replace the `data/p2pstore` folder accordingly and make sure that the correct permissions are set:
    ```
    chown 65532:65532 data/p2pstore -R
    ```
 
5) Add your peer(s) - to which your HORNET node should connect. For `chrysalis-mainnet`, add the peers to the `hornet/chrysalis-mainnet/peering.json` file. For `chrysalis-devnet`, add the peers to the `hornet/chrysalis-devnet/peering.json` file. If you don't have any peers, please contact us and we will help you find some.

    For better illustration, the `peering.json` file should then look like the following, for example:
    ```json
   {
     "peers": [
       {
         "multiAddress": "/dns/xyz.com/tcp/15600/p2p/12D3KooWEVGFchjYqCH1nAWt2norb6sJYEedmEkPofoNiyDwyptf",
         "alias": "Alice"
       },
       {
         "multiAddress": "/ip4/121.56.12.23/tcp/15600/p2p/12D3KooWRNYKZXYqZngxQee5BefmzcW5Zk6Tc6iE92U2uZwArHw9",
         "alias": "Bob"
       }
     ]
   }
   ```
   
    Also, make sure that your peers know the multiaddress of your HORNET node so that they will be able to mutually tether. A multiaddress - as illustrated above - consists of the **address where you deploy the implementation and the node ID**. You can find your node ID in the HORNET logs when you run the implementation.
    
6) Run the implementation in the desired mode:

    **chrysalis-mainnet: online mode**
    ```
    MODE=online docker-compose -f docker-compose.chrysalis-mainnet.yml up
    ```
    
    **chrysalis-mainnet: offline mode**
    ```
    MODE=offline docker-compose -f docker-compose.chrysalis-mainnet.yml up
    ```
    
    **chrysalis-devnet: online mode**
    ```
    MODE=online docker-compose -f docker-compose.chrysalis-devnet.yml up
    ```
    
    **chrysalis-devnet: offline mode**
    ```
    MODE=offline docker-compose -f docker-compose.chrysalis-devnet.yml up
    ```

7) If you want to reuse the node ID of your HORNET node with a later deployment (**see step 4.**) make sure you back up the private and public key files that make up your node ID. Otherwise, you cannot preserve the same node ID for subsequent deployments. 
You can back up your node ID by preserving the `data/p2pstore` directory.

Once the HORNET node has synced with the network, the Rosetta API will be available at:
http://127.0.0.1:3030

The health status of the HORNET node can be checked at: http://127.0.0.1:14265/api/v1/info

#### Environment variables:
- `MODE` ... the mode in which the implementation is to run; can be either `offline` or `online`

## Further notes:

The HORNET node will be bootstrapped automatically with recent snapshots to start synchronizing from a more recent block. For `chrysalis-mainnet`, the snapshots will be automatically downloaded from https://chrysalis-dbfiles.iota.org. For `chrysalis-devnet`, the snapshots will be automatically downloaded from http://dbfiles.chrysalis-devnet.iota.cafe. If you want to bootstrap the HORNET node yourself, you can do so by placing your snapshots appropriately in the `data/snapshots` directory.

