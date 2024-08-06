use std::ffi::OsStr;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use eyre::{eyre, Context, Result};

use helios::config::networks::Network;
use serde::Deserialize;
use validator::Validate;

const DEFAULT_DATA_DIR: &str = "tmp";
const DEFAULT_POLL_SECS: u64 = 5;

const MAINNET_ETHEREUM_CHAINID: &str = "0x1";
const SEPOLIA_ETHEREUM_CHAINID: &str = "0xaa36a7";

const MAINNET_STARKNET_CHAINID: &str = "0x534e5f4d41494e";
const SEPOLIA_STARKNET_CHAINID: &str = "0x534e5f5345504f4c4941";

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

    pub async fn check(&self, skip_chain_id_validation: bool) -> Result<()> {
        self.validate()?;
        if skip_chain_id_validation {
            return Ok(());
        }

        let expected_chain_id = match self.network {
            Network::MAINNET => MAINNET_ETHEREUM_CHAINID,
            Network::SEPOLIA => SEPOLIA_ETHEREUM_CHAINID,
            _ => {
                eyre::bail!(
                    "Ethereum chain id check failed: unsupported network"
                );
            }
        };
        check_chain_id(
            expected_chain_id,
            &self.eth_execution_rpc,
            "eth_chainId",
        )
        .await?;

        let expected_chain_id = match self.network {
            Network::MAINNET => MAINNET_STARKNET_CHAINID,
            Network::SEPOLIA => SEPOLIA_STARKNET_CHAINID,
            _ => {
                eyre::bail!(
                    "Starknet chain id check failed: unsupported network"
                );
            }
        };
        check_chain_id(
            expected_chain_id,
            &self.starknet_rpc,
            "starknet_chainId",
        )
        .await?;

        check_data_dir(&self.data_dir)
    }
}

fn check_data_dir<P: AsRef<Path>>(path: &P) -> Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        eyre::bail!("path does not exist");
    };

    let meta = path.metadata().context("path metadata is missing")?;

    if meta.permissions().readonly() {
        eyre::bail!("path is readonly");
    }

    Ok(())
}

async fn check_chain_id(
    expected_chain_id: &str,
    url: &str,
    method: &str,
) -> Result<()> {
    let chain_id = call_method(url, method).await?;
    if chain_id != expected_chain_id {
        eyre::bail!(
            "Invalid chain id: expected {expected_chain_id} but got {chain_id}"
        );
    }
    Ok(())
}

async fn call_method(url: &str, method: &str) -> Result<String> {
    let response: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": [],
            "id": 0
        }))
        .send()
        .await?
        .json()
        .await?;

    response["result"]
        .as_str()
        .map(|result| result.to_owned())
        .ok_or_else(|| eyre!("Result missing for method={method}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{matchers::any, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn wrong_urls() {
        let config = Config {
            network: Network::MAINNET,
            eth_execution_rpc: "foo".to_string(),
            starknet_rpc: "bar".to_string(),
            data_dir: Default::default(),
            poll_secs: 300,
            rpc_addr: SocketAddr::from(([0, 0, 0, 0], 3030)),
        };
        let skip_chain_id_validation = false;
        let response = config.check(skip_chain_id_validation).await;

        assert!(response.is_err());
        assert!(response
            .unwrap_err()
            .to_string()
            .contains("eth_execution_rpc"));
    }

    #[tokio::test]
    async fn correct_eth_url() {
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"jsonrpc":"2.0","id":0,"result":MAINNET_ETHEREUM_CHAINID})))
            .expect(1)
            .mount(&server)
            .await;

        let result = check_chain_id(
            MAINNET_ETHEREUM_CHAINID,
            &server.uri(),
            "eth_chainId",
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn wrong_eth_url() {
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"jsonrpc":"2.0","id":0,"error":"foo"}),
            ))
            .expect(1)
            .mount(&server)
            .await;

        let result = check_chain_id(
            MAINNET_ETHEREUM_CHAINID,
            &server.uri(),
            "eth_chainId",
        )
        .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Result missing for method=eth_chainId"
        );
    }

    #[tokio::test]
    async fn wrong_starknet_url() {
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"jsonrpc":"2.0","id":0,"error":"foo"}),
            ))
            .expect(1)
            .mount(&server)
            .await;

        let result = check_chain_id(
            MAINNET_STARKNET_CHAINID,
            &server.uri(),
            "eth_chainId",
        )
        .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Result missing for method=eth_chainId"
        );
    }

    #[tokio::test]
    async fn correct_eth_url_wrong_chain_id() {
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"jsonrpc":"2.0","id":0,"error":"foo"}),
            ))
            .expect(1)
            .mount(&server)
            .await;

        let result = check_chain_id(
            MAINNET_STARKNET_CHAINID,
            &server.uri(),
            "starknet_chainId",
        )
        .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Result missing for method=starknet_chainId"
        );
    }

    #[tokio::test]
    async fn correct_starknet_url_wrong_chain_id() {
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"jsonrpc":"2.0","id":0,"error":"foo"}),
            ))
            .expect(1)
            .mount(&server)
            .await;

        let result = check_chain_id(
            MAINNET_STARKNET_CHAINID,
            &server.uri(),
            "starknet_chainId",
        )
        .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Result missing for method=starknet_chainId"
        );
    }

    #[tokio::test]
    async fn wrong_poll_secs() {
        let config = Config {
            network: Network::MAINNET,
            eth_execution_rpc: "foo".to_string(),
            starknet_rpc: "bar".to_string(),
            data_dir: Default::default(),
            poll_secs: 9999,
            rpc_addr: SocketAddr::from(([127, 0, 0, 1], 3030)),
        };
        let skip_chain_id_validation = false;
        let response = config.check(skip_chain_id_validation).await;

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("poll_secs"));
    }
}
