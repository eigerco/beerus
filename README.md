<div align="center">
  <img src="etc/beerus.png" height="250" />
  <div align="center">

  [![check-job-status](https://github.com/eigerco/beerus/actions/workflows/check.yml/badge.svg)](https://github.com/eigerco/beerus/actions/workflows/check.yml)
  [![jsonrpc-spec-v0.6.0](https://img.shields.io/badge/JSON--RPC-v0.6.0-2ea44f?labelColor=282d33&logo=ethereum)](https://github.com/starkware-libs/starknet-specs/tree/v0.6.0)

  </div>
  <h1>Beerus</h1>

  Beerus is a Starknet Light Client inspired by and using
  [Helios](https://github.com/a16z/helios/).
</div>

## News

* 2024-JUN-18: "Beerus Reborn": brand new Beerus with RPC Codegen, Stateless Execution, State Proof Verification, release [v0.5.0](https://github.com/eigerco/beerus/releases/tag/v0.5.0)
* 2024-FEB-29: Migrate to the [Starknet v0.6.0 OpenRPC spec](https://github.com/starkware-libs/starknet-specs/tree/v0.6.0)
* 2024-JAN-17: [Eiger is taking over Beerus!](https://www.eiger.co/blog/eiger-taking-over-ownership-for-beerus-working-on-starknet-light-clients)

## Getting Started

### Running Beerus for the first time

Copy the configuration file from `etc/conf/beerus.toml` and set up the RPC provider URLs in the copy.
Make sure that providers are compatible. Read more about providers [here](#rpc-providers)

Then run:
```bash
cargo run --release -- -c ./path/to/config.toml
```

Once Beerus has started to verify that it is up and running, try this request:
```
curl -H 'Content-type: application/json' -d'{
    "jsonrpc": "2.0",
    "method": "starknet_getStateRoot",
    "params": [],
    "id": 1
}' http://127.0.0.1:3030
```

The successful result should look similar to the one below:
```
{"jsonrpc":"2.0","result":"0x539895aff28be4958188c1d4e8e68ee6772bdd49dd9362a4fbb189e61c54ff1","id":1}
```

### Configuration

| field   | example | description |
| ----------- | ----------- | ----------- |
| network | MAINNET or SEPOLIA| network to query |
| eth_execution_rpc | https://eth-mainnet.g.alchemy.com/v2/{YOUR_API_KEY}| untrusted l1 node provider url |
| starknet_rpc | https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0.6/{YOUR_API_KEY}| untrusted l2 node provider url |
| data_dir | tmp | `OPTIONAL` location to store both l1 and l2 data |
| poll_secs | 5 | `OPTIONAL` seconds to wait for querying sn state, min = 1 and max = 3600 |
| rpc_addr | 127.0.0.1:3030 | `OPTIONAL` local address to listen for rpc reqs |

When you select a network, check that `eth_execution_rpc` and `starknet_rpc` urls also point to their corresponding networks. For example:

MAINNET
```
eth_execution_rpc = "https://eth-mainnet.g.alchemy.com/v2/{YOUR_API_KEY}"
starknet_rpc = "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0.6/{YOUR_API_KEY}"
```
SEPOLIA
```
eth_execution_rpc = "https://eth-sepolia.g.alchemy.com/v2/{YOUR_API_KEY}"
starknet_rpc = "https://starknet-sepolia.g.alchemy.com/starknet/version/rpc/v0.6/{YOUR_API_KEY}"
```

#### RPC providers
Beerus relies on TWO untrusted RPC endpoints, one for L1 (Ethereum), and one for L2 (Starknet).
As these are untrusted they will typically not be nodes run on your local host or your local network.

##### Starknet RPC endpoint
Beerus requires the [v0.6.0 of the Starknet OpenRPC specs](https://github.com/starkware-libs/starknet-specs/tree/v0.6.0).

Starknet RPC provider must also support the [Pathfinder's extension API](https://github.com/eqlabs/pathfinder#pathfinder-extension-api) `pathfinder_getProof` endpoint. 

You can check if the provider is compatible by running this command:
```bash
# This is an example RPC url. Use your RPC provider url to check if the node is compatible.
STARKNET_RPC_URL="https://starknet-sepolia.g.alchemy.com/starknet/version/rpc/v0.6/{YOUR_API_KEY}"
curl --request POST \
     --url $STARKNET_RPC_URL \
     --header 'content-type: application/json' \
     --data '
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "pathfinder_getProof",
  "params": [
    {
      "block_number": 56072
    },
    "0x07cb0dca5767f238b056665d2f8350e83a2dee7eac8ec65e66bbc790a4fece8a",
    [
        "0x01d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
    ]
  ]
}
'
```

If you get a response similar to the one below, then the provider is **not compatible**.
```
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32601,
    "message": "method 'pathfinder_getProof' not found"
  }
}
```

We recommend using one of these providers:
- [Alchemy](https://docs.alchemy.com/reference/starknet-api-faq#what-versions-of-starknet-api-are-supported)
- [Chainstack](https://docs.chainstack.com/docs/starknet-tooling)
- [Reddio](https://docs.reddio.com/guide/node/starknet.html#grab-starknet-sepolia-endpoint)


More API providers can be found [here](https://docs.starknet.io/documentation/tools/api-services/).

##### Ethereum RPC endpoint
For the Ethereum RPC provider, there are no special requirements. The provider must support [Ethereum JSON-RPC Specification](https://ethereum.github.io/execution-apis/api-documentation/)

*NOTE: we rely on [helios](https://github.com/a16z/helios) for both valid checkpoint values and consensus rpc urls*

## Development

#### Build

```bash
cargo build --release
```

#### Test

```bash
cargo test

## Run integration tests against live endpoint
export BEERUS_TEST_STARKNET_URL=https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0.6/${ALCHEMY_API_KEY}
BEERUS_TEST_RUN=1 cargo test --features skip-zero-root-validation
```

#### Docker

```bash
docker build . -t beerus
```

```bash
docker run -e NETWORK=<arg> -e ETH_EXECUTION_RPC=<arg> -e STARKNET_RPC=<arg> -it beerus
```

#### Examples

```bash
ALCHEMY_API_KEY='YOURAPIKEY' cargo run --release --example call
ALCHEMY_API_KEY='YOURAPIKEY' cargo run --release --example state
```

## Security

Beerus follows good practices of security, but 100% security cannot be assured.
Beerus is provided **"as is"** without any **warranty**. Use at your own risk.
