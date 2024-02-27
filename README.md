<div align="center">
  <img src="etc/beerus.png" height="250" />
  <div align="center">

  [![CI Action Status](https://github.com/eigerco/beerus/actions/workflows/ci.yml/badge.svg)](https://github.com/eigerco/beerus/actions/workflows/ci.yml)
  [![Check Workflow Status](https://github.com/eigerco/beerus/actions/workflows/check.yml/badge.svg)](https://github.com/eigerco/beerus/actions/workflows/check.yml)
  ![starknet-version-v0.13.0](https://img.shields.io/badge/Starknet_Version-v0.13.0-2ea44f?labelColor=282d33&logo=ethereum)
  [![jsonrpc-spec-v0.6.0](https://img.shields.io/badge/JSON--RPC-v0.6.0-2ea44f?labelColor=282d33&logo=ethereum)](https://github.com/starkware-libs/starknet-specs/tree/v0.6.0)

  </div>
  <h1>Beerus</h1>

  Beerus is a Starknet Light Client inspired by and using
  [helios](https://github.com/a16z/helios/).

  See the [Beerus Book](book/README.md) for more info.
</div>

## News

* 17/01/24: [Eiger is taking over Beerus!](https://www.eiger.co/blog/eiger-taking-over-ownership-for-beerus-working-on-starknet-light-clients)

## Getting Started

```bash
cargo build --release

# insert valid api keys
./target/release/beerus -c etc/conf/beerus.json

# wait for server to start
hurl etc/rpc/starknet_getStateRoot.hurl
```

### Config

Beerus relies on TWO untrusted RPC endpoints. As these are untrusted they will
typically not be nodes run on your local host or your local network. These 
untrusted RPC providers must adhere to both the L1 `eth_getProof` endpoint
as well as the L2 `pathfinder_getProof` endpoint. For this we recommend using
[Alchemy](https://www.alchemy.com) as your untrusted L2 node provider. 

*NOTE: we rely on helios for both valid checkpoint values and consensus rpc urls*

| field   | example | description |
| ----------- | ----------- | ----------- |
| network | MAINNET or GOERLI | network to query |
| eth_execution_rpc | https://eth-mainnet.g.alchemy.com/v2/YOURAPIKEY | untrusted l1 node provider url |
| starknet_rpc | https://starknet-mainnet.g.alchemy.com/v2/YOURAPIKEY | untrusted l2 node provider url |
| data_dir | tmp | `OPTIONAL` location to store both l1 and l2 data |
| poll_secs | 5 | `OPTIONAL` seconds to wait for querying sn state |
| rpc_addr | 127.0.0.1:3030 | `OPTIONAL` local address to listen for rpc reqs |
| fee_token_addr | 0x049d36...e004dc7 | `OPTIONAL` fee token to check for `getBalance` |

## Development

#### Build

```bash
cargo build --all --release
```

#### Test

```bash
cargo test --all
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
ALCHEMY_API_KEY='YOURAPIKEY' cargo run --example basic
ALCHEMY_API_KEY='YOURAPIKEY' cargo run --example call
```

##### Beerus JS (wasm demo)

Dependencies:

- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [CORS bypass](https://github.com/garmeeh/local-cors-proxy/blob/master/README.md)
- local pathfinder node at `http://localhost:9545`
- execution env var - `ETH_EXECUTION_RPC`

```bash
cd etc/wasm

# install node deps
npm i

# build webpack & wasm modules
npm run build

# run example
./run.sh

# navigate browser to http://localhost:8080
# open developer console
```

## Security

Beerus follows good practices of security, but 100% security cannot be assured.
Beerus is provided **"as is"** without any **warranty**. Use at your own risk.

## Acknowledgements

- Huge props to A16z for their work on
  [helios](https://github.com/a16z/helios/).

## [Contributors ✨](https://github.com/eigerco/beerus/contributors)

