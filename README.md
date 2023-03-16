<div align="center">
  <h1>Beerus</h1>
    <img src="docs/images/beerus.png" height="200">
  <br />
  <br />
  <a href="https://github.com/keep-starknet-strange/beerus/issues/new?assignees=&labels=bug&template=01_BUG_REPORT.md&title=bug%3A+">Report a Bug</a>
  -
  <a href="https://github.com/keep-starknet-strange/beerus/issues/new?assignees=&labels=enhancement&template=02_FEATURE_REQUEST.md&title=feat%3A+">Request a Feature</a>
  -
  <a href="https://github.com/keep-starknet-strange/beerus/discussions">Ask a Question</a>
</div>

<div align="center">
<br />

[![GitHub Workflow Status](https://github.com/keep-starknet-strange/beerus/actions/workflows/check.yml/badge.svg)](https://github.com/keep-starknet-strange/beerus/actions/workflows/check.yml)
[![Project license](https://img.shields.io/github/license/keep-starknet-strange/beerus.svg?style=flat-square)](LICENSE)
[![Pull Requests welcome](https://img.shields.io/badge/PRs-welcome-ff69b4.svg?style=flat-square)](https://github.com/keep-starknet-strange/beerus/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)

</div>

<details>
<summary>Table of Contents</summary>

- [Report a Bug](#report-a-bug)
- [Request a Feature](#request-a-feature)
- [About](#about)
  - [Built With](#built-with)
- [Architecture](#architecture)
  - [Simple usage overview](#simple-usage-overview)
- [Getting Started](#getting-started)
  - [Installation](#installation)
    - [Beerusup](#beerusup)
    - [Build](#build)
    - [Test](#test)
    - [Config](#config)
      - [Config File](#config-file)
      - [Environment Variables](#environment-variables)
    - [Beerus CLI](#beerus-cli)
      - [Beerus CLI](#beerus-cli-1)
    - [Beerus RPC](#beerus-rpc)
      - [Beerus RPC](#beerus-rpc-1)
- [Roadmap](#roadmap)
- [Support](#support)
- [Project assistance](#project-assistance)
- [Contributing](#contributing)
- [Security](#security)
- [Acknowledgements](#acknowledgements)
- [Contributors ✨](#contributors-)

</details>

---

## About

Beerus is a StarkNet Light Client inspired by and using
[helios](https://github.com/a16z/helios/). The goal is to provide a simple and
easy to use client to query StarkNet state and interact with contracts.

### Built With

- [Rust](https://www.rust-lang.org/)
- [helios](https://github.com/a16z/helios)
- [ethers-rs](https://github.com/gakonst/ethers-rs)

## Architecture

Here is a high level overview of the architecture of Beerus.

[![Beerus architecture](docs/images/beerus-architecture-v1.0.png)](docs/images/beerus-architecture-v1.0.png)

### Simple usage overview

Here is a simple overview of how Beerus work. The example is for querying a
storage value of a StarkNet contract.

[![Beerus Query Contract Storage](docs/images/query-contract-storage.png)](docs/images/query-contract-storage.png)

## Getting Started

### Installation

#### Beerusup

To install with `beerusup`:

```bash
curl -sL https://raw.githubusercontent.com/keep-starknet-strange/beerus/main/beerusup | sh
```

#### Build

```bash
cargo build --release
```

#### Test

```bash
cargo test
```

#### Config

The project requires an Ethereum node and a Starknet node. For Ethereum nodes
you can use Alchemy (not Infura since it does not support getProof endpoint).

Ethereum execution layer RPC URL (must be an Ethereum provider that supports
the eth_getProof endpoint).

Ethereum consensus layer RPC URL (must be a consensus node that supports the
light client beacon chain api)

For StarkNet node for the moment you can use Infura but soon
[verify proof](<[#62](https://github.com/keep-starknet-strange/beerus/issues/62)>)
will be implemented in Pathfinder nodes, and so will these nodes be working as
well.

| Env Var | TOML | Mainnet | Goerli |
| -------------  | -------------  | ------------- | ------------- |
| ETHEREUM_NETWORK | ethereum_network | `mainnet` | `goerli(default)` |
| ETHEREUM_EXECUTION_RPC_URL | ethereum_consensus_rpc | <https://eth-mainnet.g.alchemy.com/v2/XXXXX> | <https://eth-goerli.g.alchemy.com/v2/XXXXX> |
| ETHEREUM_CONSENSUS_RPC_URL | ethereum_execution_rpc | <https://www.lightclientdata.org> | <http://testing.prater.beacon-api.nimbus.team> |
| STARKNET_RPC_URL  | starknet_rpc | <https://starknet-mainnet.infura.io/v3/XXXXX> | <https://starknet-goerli.infura.io/v3/XXXXX> |

##### Config File

Beerus is configurable via a config toml. If you have set the env var
`BEERUS_CONFIG` = `path/to/config` this will override all other environment
variables and take configuration from values defined herein.
Also the the cli can be directed via `berrus --config <path/to/config>`

[goerli.toml](./crates/beerus-core/tests/common/data/goerli.toml)

[mainnet.toml](./crates/beerus-core/tests/common/data/mainnet.toml)

##### Environment Variables

Beerus is configurable through environment variables.

```bash
cp examples/.env.example .env
source .env
```

#### [Beerus CLI](https://github.com/keep-starknet-strange/beerus/blob/main/docs/beerus-cli/cli.md)

##### Beerus CLI

```bash
cargo run --bin beerus starknet query-state-root
```

#### [Beerus RPC](https://github.com/keep-starknet-strange/beerus/blob/main/crates/beerus-rpc/rpc.md)

##### Beerus RPC

```bash
cargo run --bin beerus-rpc
```

## Roadmap

See the [open issues](https://github.com/keep-starknet-strange/beerus/issues) for
a list of proposed features (and known issues).

- [Top Feature Requests](https://github.com/keep-starknet-strange/beerus/issues?q=label%3Aenhancement+is%3Aopen+sort%3Areactions-%2B1-desc)
  (Add your votes using the 👍 reaction)
- [Top Bugs](https://github.com/keep-starknet-strange/beerus/issues?q=is%3Aissue+is%3Aopen+label%3Abug+sort%3Areactions-%2B1-desc)
  (Add your votes using the 👍 reaction)
- [Newest Bugs](https://github.com/keep-starknet-strange/beerus/issues?q=is%3Aopen+is%3Aissue+label%3Abug)

## Support

Reach out to the maintainer at one of the following places:

- [GitHub Discussions](https://github.com/keep-starknet-strange/beerus/discussions)
- Contact options listed on
  [this GitHub profile](https://github.com/keep-starknet-strange)

## Project assistance

If you want to say **thank you** or/and support active development of Beerus:

- Add a [GitHub Star](https://github.com/keep-starknet-strange/beerus) to the
  project.
- Tweet about the Beerus.
- Write interesting articles about the project on [Dev.to](https://dev.to/),
  [Medium](https://medium.com/) or your personal blog.

Together, we can make Beerus **better**!

## Contributing

First off, thanks for taking the time to contribute! Contributions are what make
the open-source community such an amazing place to learn, inspire, and create.
Any contributions you make will benefit everybody else and are **greatly
appreciated**.

Please read [our contribution guidelines](docs/CONTRIBUTING.md), and thank you
for being involved!

## Security

Beerus follows good practices of security, but 100% security cannot be assured.
Beerus is provided **"as is"** without any **warranty**. Use at your own risk.

_For more information and to report security issues, please refer to our
[security documentation](docs/SECURITY.md).

## Acknowledgements

- Huge props to A16z for their work on
  [helios](https://github.com/a16z/helios/).

## Contributors ✨

[The contributors page](https://github.com/keep-starknet-strange/beerus/contributors).

Thanks goes to these wonderful people
([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/abdelhamidbakhta"><img src="https://avatars.githubusercontent.com/u/45264458?v=4?s=100" width="100px;" alt="Abdel @ StarkWare"/><br /><sub><b>Abdel @ StarkWare</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=abdelhamidbakhta" title="Tests">⚠️</a> <a href="https://github.com/keep-starknet-strange/beerus/commits?author=abdelhamidbakhta" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/LucasLvy"><img src="https://avatars.githubusercontent.com/u/70894690?v=4?s=100" width="100px;" alt="Lucas @ StarkWare"/><br /><sub><b>Lucas @ StarkWare</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=LucasLvy" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/danilowhk"><img src="https://avatars.githubusercontent.com/u/12735159?v=4?s=100" width="100px;" alt="danilowhk"/><br /><sub><b>danilowhk</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=danilowhk" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://www.linkedin.com/in/clementwalter"><img src="https://avatars.githubusercontent.com/u/18620296?v=4?s=100" width="100px;" alt="Clément Walter"/><br /><sub><b>Clément Walter</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=ClementWalter" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Eikix"><img src="https://avatars.githubusercontent.com/u/66871571?v=4?s=100" width="100px;" alt="Elias Tazartes"/><br /><sub><b>Elias Tazartes</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=Eikix" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/drspacemn"><img src="https://avatars.githubusercontent.com/u/16685321?v=4?s=100" width="100px;" alt="drspacemn"/><br /><sub><b>drspacemn</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=drspacemn" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/msaug"><img src="https://avatars.githubusercontent.com/u/60658558?v=4?s=100" width="100px;" alt="Mathieu"/><br /><sub><b>Mathieu</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=msaug" title="Code">💻</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/hurrikaanig"><img src="https://avatars.githubusercontent.com/u/37303126?v=4?s=100" width="100px;" alt="TurcFort07"/><br /><sub><b>TurcFort07</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=hurrikaanig" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/florian-bellotti"><img src="https://avatars.githubusercontent.com/u/7861901?v=4?s=100" width="100px;" alt="Florian Bellotti"/><br /><sub><b>Florian Bellotti</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=florian-bellotti" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/bbrandtom"><img src="https://avatars.githubusercontent.com/u/45038918?v=4?s=100" width="100px;" alt="Tom Brand"/><br /><sub><b>Tom Brand</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=bbrandtom" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/ftupas"><img src="https://avatars.githubusercontent.com/u/35031356?v=4?s=100" width="100px;" alt="ftupas"/><br /><sub><b>ftupas</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=ftupas" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/pscott"><img src="https://avatars.githubusercontent.com/u/30843220?v=4?s=100" width="100px;" alt="pscott"/><br /><sub><b>pscott</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=pscott" title="Code">💻</a></td>
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
