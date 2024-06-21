use std::ffi::OsStr;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use eyre::{eyre, Result};

use helios::config::networks::Network;
use serde::Deserialize;
use validator::Validate;

const DEFAULT_DATA_DIR: &str = "tmp";
const DEFAULT_POLL_SECS: u64 = 5;

const MAINNET_ETHEREUM_CHAINID: &str = "0x1";
const SEPOLIA_ETHEREUM_CHAINID: &str = "0xaa36a7";

const MAINNET_STARKNET_CHAINID: &str = "0x534e5f4d41494e";
const SEPOLIA_STARKNET_CHAINID: &str = "0x534e5f5345504f4c4941";

enum Blockchain {
    Ethereum,
    Starknet,
}

#[derive(Clone, Deserialize, Debug, Validate)]
pub struct Config {
    pub network: Network,
    #[validate(url)]
    pub eth_execution_rpc: String,
    #[validate(url)]
    pub starknet_rpc: String,
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
    #[serde(default = "default_poll_secs")]
    #[validate(range(min = 1, max = 3600))]
    pub poll_secs: u64,
    #[serde(default = "default_rpc_addr")]
    pub rpc_addr: SocketAddr,
}

fn default_data_dir() -> PathBuf {
    PathBuf::from(DEFAULT_DATA_DIR)
}

fn default_poll_secs() -> u64 {
    DEFAULT_POLL_SECS
}

fn default_rpc_addr() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 3030))
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            network: Network::from_str(
                &std::env::var("NETWORK").unwrap_or_default(),
            )
            .unwrap_or(Network::MAINNET),
            eth_execution_rpc: std::env::var("ETH_EXECUTION_RPC")
                .unwrap_or_default(),
            starknet_rpc: std::env::var("STARKNET_RPC").unwrap_or_default(),
            data_dir: PathBuf::from(
                std::env::var("DATA_DIR").unwrap_or_default(),
            ),
            poll_secs: u64::from_str(
                &std::env::var("POLL_SECS").unwrap_or_default(),
            )
            .unwrap_or(DEFAULT_POLL_SECS),
            rpc_addr: std::env::var("RPC_ADDR")
                .ok()
                .and_then(|rpc_addr| rpc_addr.parse::<SocketAddr>().ok())
                .unwrap_or_else(default_rpc_addr),
        }
    }

    pub fn from_file(path: &str) -> Result<Self> {
        match Path::new(path).extension().and_then(OsStr::to_str) {
            Some("toml") => {
                let content = fs::read_to_string(path)?;
                Ok(toml::from_str(&content)?)
            }
            Some("json") => {
                let content = fs::read_to_string(path)?;
                Ok(serde_json::from_str(&content)?)
            }
            Some(x) => Err(eyre!("Unexpected config file format: {x}")),
            None => Err(eyre!("Config file extension missing")),
        }
    }

    pub async fn validate_params(&self) -> Result<()> {
        self.validate()?;
        self.check_url(Blockchain::Ethereum).await?;
        self.check_url(Blockchain::Starknet).await?;
        self.validate_data_dir()
    }

    async fn check_url(&self, blockchain: Blockchain) -> Result<()> {
        let (blockchain_name, url) = match blockchain {
            Blockchain::Ethereum => ("eth", &self.eth_execution_rpc),
            Blockchain::Starknet => ("starknet", &self.starknet_rpc),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .body(format!(
                r#"{{"jsonrpc":"2.0","method":"{blockchain_name}_chainId","params":[],"id":0}}"#
            ))
            .send()
            .await
            .expect("Failed to execute request");

        if !response.status().is_success() {
            return Err(eyre!(
                "Wrong response, check {blockchain_name} url and api key"
            ));
        };

        let chain_id = match self.network {
            Network::MAINNET => match blockchain {
                Blockchain::Ethereum => MAINNET_ETHEREUM_CHAINID,
                Blockchain::Starknet => MAINNET_STARKNET_CHAINID,
            },
            Network::SEPOLIA => match blockchain {
                Blockchain::Ethereum => SEPOLIA_ETHEREUM_CHAINID,
                Blockchain::Starknet => SEPOLIA_STARKNET_CHAINID,
            },
            network => {
                return Err(eyre!(
                    "Unsupported {network}, has to be MAINNET or SEPOLIA"
                ))
            }
        };

        if response.text().await?.contains(&format!(r#"result":"{chain_id}"#)) {
            Ok(())
        } else {
            Err(eyre!("Unverified chainId for {blockchain_name} url"))
        }
    }

    fn validate_data_dir(&self) -> Result<()> {
        if !self.data_dir.exists() {
            tracing::warn!("Provided data dir does not exist");
            return Ok(());
        };

        match self.data_dir.metadata() {
            Ok(metadata) if metadata.permissions().readonly() => {
                Err(eyre!("Unable to write, read only folder"))
            }
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use helios::config::networks::Network;
    use wiremock::{matchers::any, Mock, MockServer, ResponseTemplate};

    use crate::config::{Blockchain, Config, MAINNET_ETHEREUM_CHAINID};

    impl Default for Config {
        fn default() -> Self {
            Self {
                network: Network::MAINNET,
                eth_execution_rpc: "".to_string(),
                starknet_rpc: "".to_string(),
                data_dir: Default::default(),
                poll_secs: 300,
                rpc_addr: SocketAddr::from(([127, 0, 0, 1], 3030)),
            }
        }
    }

    #[tokio::test]
    async fn wrong_urls() {
        let config = Config {
            eth_execution_rpc: "foo".to_string(),
            starknet_rpc: "bar".to_string(),
            ..Default::default()
        };

        let response = config.validate_params().await;

        assert!(response.is_err());
        assert!(response
            .unwrap_err()
            .to_string()
            .contains("eth_execution_rpc"));
    }

    #[tokio::test]
    async fn correct_eth_url() {
        let eth_mock_server = MockServer::start().await;
        let config = Config {
            eth_execution_rpc: eth_mock_server.uri(),
            ..Default::default()
        };

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_body_string(format!(
                r#"{{"jsonrpc":"2.0","id":0,"result":"{MAINNET_ETHEREUM_CHAINID}"}}"#
            )))
            .expect(1)
            .mount(&eth_mock_server)
            .await;

        assert!(config.check_url(Blockchain::Ethereum).await.is_ok());
    }

    #[tokio::test]
    async fn wrong_eth_url() {
        let eth_mock_server = MockServer::start().await;
        let config = Config {
            eth_execution_rpc: eth_mock_server.uri(),
            ..Default::default()
        };

        Mock::given(any())
            .respond_with(ResponseTemplate::new(400))
            .expect(1)
            .mount(&eth_mock_server)
            .await;

        let response = config.check_url(Blockchain::Ethereum).await;

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("eth"));
    }

    #[tokio::test]
    async fn wrong_starknet_url() {
        let starknet_rpc_server = MockServer::start().await;
        let config = Config {
            starknet_rpc: starknet_rpc_server.uri(),
            ..Default::default()
        };

        Mock::given(any())
            .respond_with(ResponseTemplate::new(400))
            .expect(1)
            .mount(&starknet_rpc_server)
            .await;

        let response = config.check_url(Blockchain::Starknet).await;

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("starknet"));
    }

    #[tokio::test]
    async fn correct_eth_url_wrong_chain_id() {
        let eth_rpc_server = MockServer::start().await;
        let config = Config {
            eth_execution_rpc: eth_rpc_server.uri(),
            ..Default::default()
        };

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{{"jsonrpc":"2.0","id":0,"result":"foo"}}"#,
            ))
            .expect(1)
            .mount(&eth_rpc_server)
            .await;

        let response = config.check_url(Blockchain::Ethereum).await;

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("chainId for eth"));
    }

    #[tokio::test]
    async fn correct_starknet_url_wrong_chain_id() {
        let starknet_rpc_server = MockServer::start().await;
        let config = Config {
            starknet_rpc: starknet_rpc_server.uri(),
            ..Default::default()
        };

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{{"jsonrpc":"2.0","id":0,"result":"foo"}}"#,
            ))
            .expect(1)
            .mount(&starknet_rpc_server)
            .await;

        let response = config.check_url(Blockchain::Starknet).await;

        assert!(response.is_err());
        assert!(response
            .unwrap_err()
            .to_string()
            .contains("chainId for starknet"));
    }

    #[tokio::test]
    async fn wrong_poll_secs() {
        let config = Config { poll_secs: 9999, ..Default::default() };

        let response = config.validate_params().await;

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("poll_secs"));
    }
}
