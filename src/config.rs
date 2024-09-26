use std::fs;
use std::net::SocketAddr;
use std::path::Path;

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

#[cfg(feature = "testing")]
const KATANA_STARKNET_CHAINID: &str = "0x4b4154414e41";

#[derive(Clone, Deserialize, Debug, Validate)]
pub struct ServerConfig {
    #[serde(flatten)]
    pub client: Config,
    #[serde(default = "default_poll_secs")]
    #[validate(range(min = 1, max = 3600))]
    pub poll_secs: u64,
    #[serde(default = "default_rpc_addr")]
    pub rpc_addr: SocketAddr,
}

#[derive(Clone, Deserialize, Debug, Validate)]
pub struct Config {
    #[validate(url)]
    pub ethereum_rpc: String,
    #[validate(url)]
    pub starknet_rpc: String,
    #[serde(default = "default_data_dir")]
    pub data_dir: String,
}

fn default_data_dir() -> String {
    DEFAULT_DATA_DIR.to_owned()
}

fn default_poll_secs() -> u64 {
    DEFAULT_POLL_SECS
}

fn default_rpc_addr() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 3030))
}

impl ServerConfig {
    pub fn from_env() -> Result<Self> {
        let poll_secs = if let Ok(poll_secs) = std::env::var("POLL_SECS") {
            poll_secs.parse()?
        } else {
            DEFAULT_POLL_SECS
        };
        let rpc_addr = if let Ok(rpc_addr) = std::env::var("RPC_ADDR") {
            rpc_addr.parse()?
        } else {
            default_rpc_addr()
        };
        Ok(Self {
            client: Config {
                ethereum_rpc: std::env::var("ETHEREUM_RPC")
                    .context("ETHEREUM_RPC env var missing")?,
                starknet_rpc: std::env::var("STARKNET_RPC")
                    .context("STARKNET_RPC env var missing")?,
                data_dir: std::env::var("DATA_DIR")
                    .unwrap_or_else(|_| default_data_dir()),
            },
            poll_secs,
            rpc_addr,
        })
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}

pub async fn check_chain_id(
    ethereum_rpc: &str,
    starknet_rpc: &str,
) -> Result<Network> {
    let ethereum_chain_id = call_method(ethereum_rpc, "eth_chainId").await?;
    let starknet_chain_id =
        call_method(starknet_rpc, "starknet_chainId").await?;

    if ethereum_chain_id == MAINNET_ETHEREUM_CHAINID
        && starknet_chain_id == MAINNET_STARKNET_CHAINID
    {
        return Ok(Network::MAINNET);
    }

    if ethereum_chain_id == SEPOLIA_ETHEREUM_CHAINID
        && starknet_chain_id == SEPOLIA_STARKNET_CHAINID
    {
        return Ok(Network::SEPOLIA);
    }

    #[cfg(feature = "testing")]
    if starknet_chain_id == KATANA_STARKNET_CHAINID {
        return match ethereum_chain_id.as_str() {
            MAINNET_ETHEREUM_CHAINID => Ok(Network::MAINNET),
            SEPOLIA_ETHEREUM_CHAINID => Ok(Network::SEPOLIA),
            _ => {
                eyre::bail!(
                    "Unexpected Ethereum chain_id: {ethereum_chain_id}"
                );
            }
        };
    }

    eyre::bail!("chain_id mismatch: ethereum={ethereum_chain_id}, starknet={starknet_chain_id}")
}

pub fn check_data_dir<P: AsRef<Path>>(path: &P) -> Result<()> {
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

    if let Some(error) = response["error"].as_str() {
        eyre::bail!("rpc error: {error}");
    }
    if let Some(error) = response["error"].as_object() {
        let error = serde_json::to_string(error)?;
        eyre::bail!("rpc error: {error}");
    }

    response["result"]
        .as_str()
        .map(|result| result.to_owned())
        .ok_or_else(|| eyre!("Result missing for method={method}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        matchers::body_partial_json, Mock, MockServer, ResponseTemplate,
    };

    async fn mock(
        patterns: &[(serde_json::Value, serde_json::Value)],
    ) -> MockServer {
        let server = MockServer::start().await;
        for (request, response) in patterns {
            Mock::given(body_partial_json(&request))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json(response),
                )
                .mount(&server)
                .await;
        }
        server
    }

    #[tokio::test]
    async fn wrong_urls() {
        let config = ServerConfig {
            client: Config {
                ethereum_rpc: "foo".to_string(),
                starknet_rpc: "bar".to_string(),
                data_dir: Default::default(),
            },
            poll_secs: 300,
            rpc_addr: SocketAddr::from(([0, 0, 0, 0], 3030)),
        };
        let response = config.client.validate();

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("ethereum_rpc"));
    }

    #[tokio::test]
    async fn test_mainnet_detected() {
        let server = mock(&[
            (
                serde_json::json!({
                    "method": "eth_chainId"
                }),
                serde_json::json!({
                    "id": 0,
                    "jsonrpc": "2.0",
                    "result": MAINNET_ETHEREUM_CHAINID
                }),
            ),
            (
                serde_json::json!({
                    "method": "starknet_chainId"
                }),
                serde_json::json!({
                    "id": 0,
                    "jsonrpc": "2.0",
                    "result": MAINNET_STARKNET_CHAINID
                }),
            ),
        ])
        .await;

        let rpc = format!("http://{}/", server.address());
        let network = check_chain_id(&rpc, &rpc).await.expect("check_chain_id");
        assert_eq!(network, Network::MAINNET);
    }

    #[tokio::test]
    async fn test_testnet_detected() {
        let server = mock(&[
            (
                serde_json::json!({
                    "method": "eth_chainId"
                }),
                serde_json::json!({
                    "id": 0,
                    "jsonrpc": "2.0",
                    "result": SEPOLIA_ETHEREUM_CHAINID
                }),
            ),
            (
                serde_json::json!({
                    "method": "starknet_chainId"
                }),
                serde_json::json!({
                    "id": 0,
                    "jsonrpc": "2.0",
                    "result": SEPOLIA_STARKNET_CHAINID
                }),
            ),
        ])
        .await;

        let rpc = format!("http://{}/", server.address());
        let network = check_chain_id(&rpc, &rpc).await.expect("check_chain_id");
        assert_eq!(network, Network::SEPOLIA);
    }

    #[tokio::test]
    async fn test_chain_mismatch() {
        let server = mock(&[
            (
                serde_json::json!({
                    "method": "eth_chainId"
                }),
                serde_json::json!({
                    "id": 0,
                    "jsonrpc": "2.0",
                    "result": "0xA"
                }),
            ),
            (
                serde_json::json!({
                    "method": "starknet_chainId"
                }),
                serde_json::json!({
                    "id": 0,
                    "jsonrpc": "2.0",
                    "result": "0xB"
                }),
            ),
        ])
        .await;

        let rpc = format!("http://{}/", server.address());
        let result = check_chain_id(&rpc, &rpc).await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "chain_id mismatch: ethereum=0xA, starknet=0xB"
        );
    }

    #[tokio::test]
    async fn test_chain_error() {
        let server = mock(&[
            (
                serde_json::json!({
                    "method": "eth_chainId"
                }),
                serde_json::json!({
                    "id": 0,
                    "jsonrpc": "2.0",
                    "result": "0xcafebabe"
                }),
            ),
            (
                serde_json::json!({
                    "method": "starknet_chainId"
                }),
                serde_json::json!({
                    "id": 0,
                    "jsonrpc": "2.0",
                    "error": "computer says no"
                }),
            ),
        ])
        .await;

        let rpc = format!("http://{}/", server.address());
        let result = check_chain_id(&rpc, &rpc).await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "rpc error: computer says no"
        );
    }

    #[tokio::test]
    async fn wrong_poll_secs() {
        let config = ServerConfig {
            client: Config {
                ethereum_rpc: "foo".to_string(),
                starknet_rpc: "bar".to_string(),
                data_dir: Default::default(),
            },
            poll_secs: 9999,
            rpc_addr: SocketAddr::from(([127, 0, 0, 1], 3030)),
        };
        let response = config.validate();

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("poll_secs"));
    }
}
