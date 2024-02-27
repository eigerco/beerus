<div align="center">
  <img src="etc/beerus.png" height="250" />
  <div align="center">

  [![CI Action Status](https://github.com/eigerco/beerus/actions/workflows/ci.yml/badge.svg)](https://github.com/eigerco/beerus/actions/workflows/ci.yml)
  [![Check Workflow Status](https://github.com/eigerco/beerus/actions/workflows/check.yml/badge.svg)](https://github.com/eigerco/beerus/actions/workflows/check.yml)
  ![starknet-version-v0.13.0](https://img.shields.io/badge/Starknet_Version-v0.13.0-2ea44f?labelColor=282d33&logo=ethereum)
  [![jsonrpc-spec-v0.6.0](https://img.shields.io/badge/JSON--RPC-v0.6.0-2ea44f?labelColor=282d33&logo=ethereum)](https://github.com/starkware-libs/starknet-specs/tree/v0.6.0)


  [![Exploration_Team](https://img.shields.io/badge/Exploration_Team-29296E.svg?&style=for-the-badge&logo=data:image/svg%2bxml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz48c3ZnIGlkPSJhIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxODEgMTgxIj48ZGVmcz48c3R5bGU+LmJ7ZmlsbDojZmZmO308L3N0eWxlPjwvZGVmcz48cGF0aCBjbGFzcz0iYiIgZD0iTTE3Ni43Niw4OC4xOGwtMzYtMzcuNDNjLTEuMzMtMS40OC0zLjQxLTIuMDQtNS4zMS0xLjQybC0xMC42MiwyLjk4LTEyLjk1LDMuNjNoLjc4YzUuMTQtNC41Nyw5LjktOS41NSwxNC4yNS0xNC44OSwxLjY4LTEuNjgsMS44MS0yLjcyLDAtNC4yN0w5Mi40NSwuNzZxLTEuOTQtMS4wNC00LjAxLC4xM2MtMTIuMDQsMTIuNDMtMjMuODMsMjQuNzQtMzYsMzcuNjktMS4yLDEuNDUtMS41LDMuNDQtLjc4LDUuMThsNC4yNywxNi41OGMwLDIuNzIsMS40Miw1LjU3LDIuMDcsOC4yOS00LjczLTUuNjEtOS43NC0xMC45Ny0xNS4wMi0xNi4wNi0xLjY4LTEuODEtMi41OS0xLjgxLTQuNCwwTDQuMzksODguMDVjLTEuNjgsMi4zMy0xLjgxLDIuMzMsMCw0LjUzbDM1Ljg3LDM3LjNjMS4zNiwxLjUzLDMuNSwyLjEsNS40NCwxLjQybDExLjQtMy4xMSwxMi45NS0zLjYzdi45MWMtNS4yOSw0LjE3LTEwLjIyLDguNzYtMTQuNzYsMTMuNzNxLTMuNjMsMi45OC0uNzgsNS4zMWwzMy40MSwzNC44NGMyLjIsMi4yLDIuOTgsMi4yLDUuMTgsMGwzNS40OC0zNy4xN2MxLjU5LTEuMzgsMi4xNi0zLjYsMS40Mi01LjU3LTEuNjgtNi4wOS0zLjI0LTEyLjMtNC43OS0xOC4zOS0uNzQtMi4yNy0xLjIyLTQuNjItMS40Mi02Ljk5LDQuMyw1LjkzLDkuMDcsMTEuNTIsMTQuMjUsMTYuNzEsMS42OCwxLjY4LDIuNzIsMS42OCw0LjQsMGwzNC4zMi0zNS43NHExLjU1LTEuODEsMC00LjAxWm0tNzIuMjYsMTUuMTVjLTMuMTEtLjc4LTYuMDktMS41NS05LjE5LTIuNTktMS43OC0uMzQtMy42MSwuMy00Ljc5LDEuNjhsLTEyLjk1LDEzLjg2Yy0uNzYsLjg1LTEuNDUsMS43Ni0yLjA3LDIuNzJoLS42NWMxLjMtNS4zMSwyLjcyLTEwLjYyLDQuMDEtMTUuOGwxLjY4LTYuNzNjLjg0LTIuMTgsLjE1LTQuNjUtMS42OC02LjA5bC0xMi45NS0xNC4xMmMtLjY0LS40NS0xLjE0LTEuMDgtMS40Mi0xLjgxbDE5LjA0LDUuMTgsMi41OSwuNzhjMi4wNCwuNzYsNC4zMywuMTQsNS43LTEuNTVsMTIuOTUtMTQuMzhzLjc4LTEuMDQsMS42OC0xLjE3Yy0xLjgxLDYuNi0yLjk4LDE0LjEyLTUuNDQsMjAuNDYtMS4wOCwyLjk2LS4wOCw2LjI4LDIuNDYsOC4xNiw0LjI3LDQuMTQsOC4yOSw4LjU1LDEyLjk1LDEyLjk1LDAsMCwxLjMsLjkxLDEuNDIsMi4wN2wtMTMuMzQtMy42M1oiLz48L3N2Zz4=)](https://github.com/eigerco)

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
./target/release/beerus -c examples/conf/beerus.json

# wait for server to start
hurl examples/rpc/starknet_getStateRoot.hurl
```

### Config

Beerus relies on TWO untrusted RPC endpoints. As these are untrusted they will
typically not be nodes run on your local host or your local network. These 
untrusted RPC providers must adhere to both the l1 `eth_getProof` endpoint
as well as the l2 `pathfinder_getProof` endpoint. For this we recommend using
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
cd examples/wasm

# install node deps
npm i

# build webpack & wasm modules
npm run build

# run example
./run.sh

# navigate browser to http://localhost:8080
# open developer console
```

## Endpoint support

*Starknet endpoints* (in compliance with [Starknet specs](https://github.com/starkware-libs/starknet-specs)):

| Endpoint                                   | Supported          |
|:-------------------------------------------| :----------------- |
| `starknet_specVersion`                     | :white_check_mark: |
| `starknet_getBlockWithTxHashes`            | :white_check_mark: |
| `starknet_getBlockWithTxs`                 | :white_check_mark: |
| `starknet_getStateUpdate`                  | :white_check_mark: |
| `starknet_getStorageAt`                    | :white_check_mark: |
| `starknet_getTransactionByHash`            | :white_check_mark: |
| `starknet_getTransactionStatus`            | :white_check_mark: |
| `starknet_getTransactionByBlockIdAndIndex` | :white_check_mark: |
| `starknet_getTransactionReceipt`           | :white_check_mark: |
| `starknet_getClass`                        | :white_check_mark: |
| `starknet_getClassHashAt`                  | :white_check_mark: |
| `starknet_getClassAt`                      | :white_check_mark: |
| `starknet_getBlockTransactionCount`        | :white_check_mark: |
| `starknet_call`                            | :white_check_mark: |
| `starknet_estimateFee`                     | :white_check_mark: |
| `starknet_estimateFeeSingle`               | :white_check_mark: |
| `starknet_blockNumber`                     | :white_check_mark: |
| `starknet_blockHashAndNumber`              | :white_check_mark: |
| `starknet_chainId`                         | :white_check_mark: |
| `starknet_syncing`                         | :white_check_mark: |
| `starknet_syncing`                         | :white_check_mark: |
| `starknet_getEvents`(not validated)        | :white_check_mark: |
| `starknet_getNonce`                        | :white_check_mark: |
| `starknet_addDeclareTransaction`           | :x:                |
| `starknet_addDeployAccountTransaction`     | :x:                |
| `starknet_addInvokeTransaction`            | :x:                |
| `starknet_getProof`                        | :white_check_mark: |
| `starknet_getBalance`                      | :white_check_mark: |
| `starknet_getStateRoot`                    | :white_check_mark: |

## Support

Reach out to the maintainer at one of the following places:

- [GitHub Discussions](https://github.com/eigerco/beerus/discussions)
- Contact options listed on
  [this GitHub profile](https://github.com/eigerco)

## Security

Beerus follows good practices of security, but 100% security cannot be assured.
Beerus is provided **"as is"** without any **warranty**. Use at your own risk.

For more information and to report security issues, please refer to our
[security documentation](docs/SECURITY.md).

## Acknowledgements

- Huge props to A16z for their work on
  [helios](https://github.com/a16z/helios/).

## Contributors ✨

[The contributors page](https://github.com/eigerco/beerus/contributors).

Thanks goes to these wonderful people
([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/abdelhamidbakhta"><img src="https://avatars.githubusercontent.com/u/45264458?v=4?s=100" width="100px;" alt="Abdel @ StarkWare"/><br /><sub><b>Abdel @ StarkWare</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=abdelhamidbakhta" title="Tests">⚠️</a> <a href="https://github.com/eigerco/beerus/commits?author=abdelhamidbakhta" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/LucasLvy"><img src="https://avatars.githubusercontent.com/u/70894690?v=4?s=100" width="100px;" alt="Lucas @ StarkWare"/><br /><sub><b>Lucas @ StarkWare</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=LucasLvy" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/danilowhk"><img src="https://avatars.githubusercontent.com/u/12735159?v=4?s=100" width="100px;" alt="danilowhk"/><br /><sub><b>danilowhk</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=danilowhk" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://www.linkedin.com/in/clementwalter"><img src="https://avatars.githubusercontent.com/u/18620296?v=4?s=100" width="100px;" alt="Clément Walter"/><br /><sub><b>Clément Walter</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=ClementWalter" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Eikix"><img src="https://avatars.githubusercontent.com/u/66871571?v=4?s=100" width="100px;" alt="Elias Tazartes"/><br /><sub><b>Elias Tazartes</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=Eikix" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/drspacemn"><img src="https://avatars.githubusercontent.com/u/16685321?v=4?s=100" width="100px;" alt="drspacemn"/><br /><sub><b>drspacemn</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=drspacemn" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/msaug"><img src="https://avatars.githubusercontent.com/u/60658558?v=4?s=100" width="100px;" alt="Mathieu"/><br /><sub><b>Mathieu</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=msaug" title="Code">💻</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/hurrikaanig"><img src="https://avatars.githubusercontent.com/u/37303126?v=4?s=100" width="100px;" alt="TurcFort07"/><br /><sub><b>TurcFort07</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=hurrikaanig" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/florian-bellotti"><img src="https://avatars.githubusercontent.com/u/7861901?v=4?s=100" width="100px;" alt="Florian Bellotti"/><br /><sub><b>Florian Bellotti</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=florian-bellotti" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/bbrandtom"><img src="https://avatars.githubusercontent.com/u/45038918?v=4?s=100" width="100px;" alt="Tom Brand"/><br /><sub><b>Tom Brand</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=bbrandtom" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/ftupas"><img src="https://avatars.githubusercontent.com/u/35031356?v=4?s=100" width="100px;" alt="ftupas"/><br /><sub><b>ftupas</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=ftupas" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/pscott"><img src="https://avatars.githubusercontent.com/u/30843220?v=4?s=100" width="100px;" alt="pscott"/><br /><sub><b>pscott</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=pscott" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/robinstraub"><img src="https://avatars.githubusercontent.com/u/17799181?v=4?s=100" width="100px;" alt="Robin Straub"/><br /><sub><b>Robin Straub</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=robinstraub" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/fkrause98"><img src="https://avatars.githubusercontent.com/u/56402156?v=4?s=100" width="100px;" alt="Francisco Krause Arnim"/><br /><sub><b>Francisco Krause Arnim</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=fkrause98" title="Documentation">📖</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/joshualyguessennd"><img src="https://avatars.githubusercontent.com/u/75019812?v=4?s=100" width="100px;" alt="joshualyguessennd"/><br /><sub><b>joshualyguessennd</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=joshualyguessennd" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/dubzn"><img src="https://avatars.githubusercontent.com/u/58611754?v=4?s=100" width="100px;" alt="Santiago Galván (Dub)"/><br /><sub><b>Santiago Galván (Dub)</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=dubzn" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/chirag-bgh"><img src="https://avatars.githubusercontent.com/u/76247491?v=4?s=100" width="100px;" alt="chirag-bgh"/><br /><sub><b>chirag-bgh</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=chirag-bgh" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/greged93"><img src="https://avatars.githubusercontent.com/u/82421016?v=4?s=100" width="100px;" alt="greged93"/><br /><sub><b>greged93</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=greged93" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/bigherc18"><img src="https://avatars.githubusercontent.com/u/126212764?v=4?s=100" width="100px;" alt="bigherc18"/><br /><sub><b>bigherc18</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=bigherc18" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Kelvyne"><img src="https://avatars.githubusercontent.com/u/8125532?v=4?s=100" width="100px;" alt="Lakhdar Slaim"/><br /><sub><b>Lakhdar Slaim</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=Kelvyne" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://linktr.ee/lndavis"><img src="https://avatars.githubusercontent.com/u/40670744?v=4?s=100" width="100px;" alt="Lance N. Davis"/><br /><sub><b>Lance N. Davis</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=lancenonce" title="Code">💻</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/tinoh9"><img src="https://avatars.githubusercontent.com/u/97869487?v=4?s=100" width="100px;" alt="Tino Huynh"/><br /><sub><b>Tino Huynh</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=tinoh9" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/irisdv"><img src="https://avatars.githubusercontent.com/u/8224462?v=4?s=100" width="100px;" alt="Iris"/><br /><sub><b>Iris</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=irisdv" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Aragar199"><img src="https://avatars.githubusercontent.com/u/14187644?v=4?s=100" width="100px;" alt="Alex Ponce"/><br /><sub><b>Alex Ponce</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=Aragar199" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/glihm"><img src="https://avatars.githubusercontent.com/u/7962849?v=4?s=100" width="100px;" alt="glihm"/><br /><sub><b>glihm</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=glihm" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/phklive"><img src="https://avatars.githubusercontent.com/u/42912740?v=4?s=100" width="100px;" alt="Paul-Henry Kajfasz"/><br /><sub><b>Paul-Henry Kajfasz</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=phklive" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/dpinones"><img src="https://avatars.githubusercontent.com/u/30808181?v=4?s=100" width="100px;" alt="Damián Piñones"/><br /><sub><b>Damián Piñones</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=dpinones" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/betacodd"><img src="https://avatars.githubusercontent.com/u/97968794?v=4?s=100" width="100px;" alt="Betacodd"/><br /><sub><b>Betacodd</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=betacodd" title="Code">💻</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Godspower-Eze"><img src="https://avatars.githubusercontent.com/u/61994334?v=4?s=100" width="100px;" alt="Godspower-Eze"/><br /><sub><b>Godspower-eze</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=Godspower-Eze" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/somthn0somthn"><img src="https://avatars.githubusercontent.com/u/41335589?v=4?s=100" width="100px;" alt="somthn0somthn"/><br /><sub><b>somthn0somthn</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=somthn0somthn" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/tonypony220"><img src="https://avatars.githubusercontent.com/u/61715244?v=4?s=100" width="100px;" alt="tonypony220"/><br /><sub><b>tonypony220</b></sub></a><br /><a href="https://github.com/eigerco/beerus/commits?author=tonypony220" title="Code">💻</a></td>
    </tr>
  </tbody>
  <tfoot>
    <tr>
      <td align="center" size="13px" colspan="7">
        <img src="https://raw.githubusercontent.com/all-contributors/all-contributors-cli/1b8533af435da9854653492b1327a23a4dbd0a10/assets/logo-small.svg">
          <a href="https://all-contributors.js.org/docs/en/bot/usage">Add your contributions</a>
        </img>
      </td>
    </tr>
  </tfoot>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the
[all-contributors](https://github.com/all-contributors/all-contributors)
specification. Contributions of any kind welcome!
