<div align="center">
  <h1>Beerus</h1>
  <br />
  <a href="#about"><strong>Explore the screenshots »</strong></a>
  <br />
  <br />
  <a href="https://github.com/abdelhamidbakhta/beerus/issues/new?assignees=&labels=bug&template=01_BUG_REPORT.md&title=bug%3A+">Report a Bug</a>
  ·
  <a href="https://github.com/abdelhamidbakhta/beerus/issues/new?assignees=&labels=enhancement&template=02_FEATURE_REQUEST.md&title=feat%3A+">Request a Feature</a>
  .<a href="https://github.com/abdelhamidbakhta/beerus/discussions">Ask a Question</a>
</div>

<div align="center">
<br />

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/abdelhamidbakhta/beerus/test?style=flat-square&logo=github)
[![Project license](https://img.shields.io/github/license/abdelhamidbakhta/beerus.svg?style=flat-square)](LICENSE)
[![Pull Requests welcome](https://img.shields.io/badge/PRs-welcome-ff69b4.svg?style=flat-square)](https://github.com/abdelhamidbakhta/beerus/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![code with love by abdelhamidbakhta](https://img.shields.io/badge/%3C%2F%3E%20with%20%E2%99%A5%20by-abdelhamidbakhta-ff1414.svg?style=flat-square)](https://github.com/abdelhamidbakhta)

</div>

<details open="open">
<summary>Table of Contents</summary>

- [About](#about)
  - [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [Support](#support)
- [Project assistance](#project-assistance)
- [Contributing](#contributing)
- [Authors & contributors](#authors--contributors)
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

|                             Screenshot 1                              |                           Screenshot 2 Page                            |
| :-------------------------------------------------------------------: | :--------------------------------------------------------------------: |
| <img src="docs/images/screenshot.png" title="Home Page" width="100%"> | <img src="docs/images/screenshot.png" title="Login Page" width="100%"> |

</details>

### Built With

- [Rust](https://www.rust-lang.org/)
- [helios](https://github.com/a16z/helios)
- [ethers-rs](https://github.com/gakonst/ethers-rs)

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Installation

```bash
cargo build
```

### Configuration

Beerus is configurable through environment variables.

Here is the list of all the available environment variables:

| Name                       | Default value | Description                                                                                               |
| -------------------------- | ------------- | --------------------------------------------------------------------------------------------------------- |
| ETHEREUM_EXECUTION_RPC_URL | No            | Ethereum execution layer RPC URL (must be an Ethereum provider that supports the eth_getProof endpoint)   |
| ETHEREUM_CONSENSUS_RPC_URL | No            | Ethereum consensus layer RPC URL (must be a consenus node that supports the light client beaconchain api) |

## Usage

> **[TODO]**

## Roadmap

See the [open issues](https://github.com/abdelhamidbakhta/beerus/issues) for a list of proposed features (and known issues).

- [Top Feature Requests](https://github.com/abdelhamidbakhta/beerus/issues?q=label%3Aenhancement+is%3Aopen+sort%3Areactions-%2B1-desc) (Add your votes using the 👍 reaction)
- [Top Bugs](https://github.com/abdelhamidbakhta/beerus/issues?q=is%3Aissue+is%3Aopen+label%3Abug+sort%3Areactions-%2B1-desc) (Add your votes using the 👍 reaction)
- [Newest Bugs](https://github.com/abdelhamidbakhta/beerus/issues?q=is%3Aopen+is%3Aissue+label%3Abug)

## Support

> **[?]**
> Provide additional ways to contact the project maintainer/maintainers.

Reach out to the maintainer at one of the following places:

- [GitHub Discussions](https://github.com/abdelhamidbakhta/beerus/discussions)
- Contact options listed on [this GitHub profile](https://github.com/abdelhamidbakhta)

## Project assistance

If you want to say **thank you** or/and support active development of Beerus:

- Add a [GitHub Star](https://github.com/abdelhamidbakhta/beerus) to the project.
- Tweet about the Beerus.
- Write interesting articles about the project on [Dev.to](https://dev.to/), [Medium](https://medium.com/) or your personal blog.

Together, we can make Beerus **better**!

## Contributing

First off, thanks for taking the time to contribute! Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make will benefit everybody else and are **greatly appreciated**.

Please read [our contribution guidelines](docs/CONTRIBUTING.md), and thank you for being involved!

## Authors & contributors

The original setup of this repository is by [Abdelhamid Bakhta](https://github.com/abdelhamidbakhta).

For a full list of all authors and contributors, see [the contributors page](https://github.com/abdelhamidbakhta/beerus/contributors).

## Security

Beerus follows good practices of security, but 100% security cannot be assured.
Beerus is provided **"as is"** without any **warranty**. Use at your own risk.

_For more information and to report security issues, please refer to our [security documentation](docs/SECURITY.md)._

## License

This project is licensed under the **MIT license**.

See [LICENSE](LICENSE) for more information.

## Acknowledgements

> **[?]**
> If your work was funded by any organization or institution, acknowledge their support here.
> In addition, if your work relies on other software libraries, or was inspired by looking at other work, it is appropriate to acknowledge this intellectual debt too.
