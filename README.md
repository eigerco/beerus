<div align="center">
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
  <h1>Beerus</h1>
    <img src="docs/images/beerus.png" height="200">
  <br />
  <a href="#about"><strong>Explore the screenshots »</strong></a>
  <br />
  <br />
  <a href="https://github.com/starknet-exploration/beerus/issues/new?assignees=&labels=bug&template=01_BUG_REPORT.md&title=bug%3A+">Report a Bug</a>
  -
  <a href="https://github.com/starknet-exploration/beerus/issues/new?assignees=&labels=enhancement&template=02_FEATURE_REQUEST.md&title=feat%3A+">Request a Feature</a>
  -
  <a href="https://github.com/starknet-exploration/beerus/discussions">Ask a Question</a>
</div>

<div align="center">
<br />

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/starknet-exploration/beerus/test?style=flat-square&logo=github)
[![Project license](https://img.shields.io/github/license/starknet-exploration/beerus.svg?style=flat-square)](LICENSE)
[![Pull Requests welcome](https://img.shields.io/badge/PRs-welcome-ff69b4.svg?style=flat-square)](https://github.com/starknet-exploration/beerus/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)

</div>

![](docs/images/beerus.gif)

<details open="open">
<summary>Table of Contents</summary>

- [Report a Bug](#report-a-bug)
- [Request a Feature](#request-a-feature)
- [About](#about)
  - [Built With](#built-with)
- [Architecture](#architecture)
- [Simple usage overview](#simple-usage-overview)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Configuration](#configuration)
- [Usage](#usage)
  - [CLI](#cli)
    - [Ethereum](#ethereum)
      - [Query balance](#query-balance)
    - [StarkNet](#starknet)
      - [Query contract view](#query-contract-view)
      - [Query get storage at](#query-get-storage-at)
  - [API](#api)
- [Roadmap](#roadmap)
- [Support](#support)
- [Project assistance](#project-assistance)
- [Contributing](#contributing)
- [Authors \& contributors](#authors--contributors)
- [Security](#security)
- [License](#license)
- [Acknowledgements](#acknowledgements)

</details>

---

## About

> Beerus is a StarkNet Light Client inspired by and using [helios](https://github.com/a16z/helios/).
> The goal is to provide a simple and easy to use client to query StarkNet state and interact with contracts.

<details>
<summary>Screenshots</summary>
<br>

|                             Screenshot 1                              |                              Screenshot 2                              |
| :-------------------------------------------------------------------: | :--------------------------------------------------------------------: |
| <img src="docs/images/screenshot.png" title="Home Page" width="100%"> | <img src="docs/images/screenshot.png" title="Login Page" width="100%"> |

</details>

### Built With

- [Rust](https://www.rust-lang.org/)
- [helios](https://github.com/a16z/helios)
- [ethers-rs](https://github.com/gakonst/ethers-rs)

## Architecture

Here is a high level overview of the architecture of Beerus.

[![Beerus architecture](docs/images/beerus-architecture-v1.0.png)](docs/images/beerus-architecture-v1.0.png)

## Simple usage overview

Here is a simple overview of how Beerus work. The example is for querying a storage value of a StarkNet contract.

[Beerus Query Contract Storage](docs/images/query-contract-storage.png)](docs/images/query-contract-storage.png)

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Installation

> **[TODO]**

### Configuration

Beerus is configurable through environment variables.

Here is the list of all the available environment variables:

| Name                       | Default value | Description                                                                                               |
| -------------------------- | ------------- | --------------------------------------------------------------------------------------------------------- |
| ETHEREUM_NETWORK           | goerli        | The Ethereum network to use. Can be one of `mainnet`, `goerli`.                                           |
| ETHEREUM_EXECUTION_RPC_URL | No            | Ethereum execution layer RPC URL (must be an Ethereum provider that supports the eth_getProof endpoint)   |
| ETHEREUM_CONSENSUS_RPC_URL | No            | Ethereum consensus layer RPC URL (must be a consenus node that supports the light client beaconchain api) |
| STARKNET_RPC_URL           | No            | StarkNet RPC URL                                                                                          |

## Usage

### CLI

```bash
Usage: beerus-cli [OPTIONS] <COMMAND>

Commands:
  ethereum  Ethereum related subcommands
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>  Sets a custom config file
  -h, --help           Print help information
  -V, --version        Print version information
```

#### Ethereum

##### Query balance

```bash
beerus-cli ethereum query-balance --address 0x00000000219ab540356cBB839Cbe05303d7705Fa
# 2011.286832686010020640 ETH
```

#### StarkNet

##### Query contract view

```bash
beerus-cli starknet query-contract --address 0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7 --selector 0x1e888a1026b19c8c0b57c72d63ed1737106aa10034105b980ba117bd0c29fe1 --calldata 0x00,0x01
[FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000000 }, FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000000 }]
```

##### Query get storage at

```bash
beerus-cli starknet query-get-storage-at --address 0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7 --key 0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1
298305742194
```

### API

> **[TODO]**

## Roadmap

See the [open issues](https://github.com/starknet-exploration/beerus/issues) for a list of proposed features (and known issues).

- [Top Feature Requests](https://github.com/starknet-exploration/beerus/issues?q=label%3Aenhancement+is%3Aopen+sort%3Areactions-%2B1-desc) (Add your votes using the 👍 reaction)
- [Top Bugs](https://github.com/starknet-exploration/beerus/issues?q=is%3Aissue+is%3Aopen+label%3Abug+sort%3Areactions-%2B1-desc) (Add your votes using the 👍 reaction)
- [Newest Bugs](https://github.com/starknet-exploration/beerus/issues?q=is%3Aopen+is%3Aissue+label%3Abug)

## Support

Reach out to the maintainer at one of the following places:

- [GitHub Discussions](https://github.com/starknet-exploration/beerus/discussions)
- Contact options listed on [this GitHub profile](https://github.com/starknet-exploration)

## Project assistance

If you want to say **thank you** or/and support active development of Beerus:

- Add a [GitHub Star](https://github.com/starknet-exploration/beerus) to the project.
- Tweet about the Beerus.
- Write interesting articles about the project on [Dev.to](https://dev.to/), [Medium](https://medium.com/) or your personal blog.

Together, we can make Beerus **better**!

## Contributing

First off, thanks for taking the time to contribute! Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make will benefit everybody else and are **greatly appreciated**.

Please read [our contribution guidelines](docs/CONTRIBUTING.md), and thank you for being involved!

## Authors & contributors

For a full list of all authors and contributors, see [the contributors page](https://github.com/starknet-exploration/beerus/contributors).

## Security

Beerus follows good practices of security, but 100% security cannot be assured.
Beerus is provided **"as is"** without any **warranty**. Use at your own risk.

_For more information and to report security issues, please refer to our [security documentation](docs/SECURITY.md)._

## License

This project is licensed under the **MIT license**.

See [LICENSE](LICENSE) for more information.

## Acknowledgements

- Huge props to A16z for their work on [helios](https://github.com/a16z/helios/).

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/abdelhamidbakhta"><img src="https://avatars.githubusercontent.com/u/45264458?v=4?s=100" width="100px;" alt="Abdel @ StarkWare "/><br /><sub><b>Abdel @ StarkWare </b></sub></a><br /><a href="https://github.com/keep-starknet-strange/beerus/commits?author=abdelhamidbakhta" title="Code">💻</a></td>
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

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!