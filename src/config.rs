use std::fs;
use std::net::SocketAddr;
use std::path::Path;

use eyre::{eyre, Context, Result};

use serde::Deserialize;
use validator::Validate;

#[cfg(not(target_arch = "wasm32"))]
const DEFAULT_DATA_DIR: &str = "tmp";
const DEFAULT_POLL_SECS: u64 = 30;

pub const MAINNET_STARKNET_CHAINID: &str = "0x534e5f4d41494e";
pub const SEPOLIA_STARKNET_CHAINID: &str = "0x534e5f5345504f4c4941";

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
    pub starknet_rpc: String,
    #[cfg(not(target_arch = "wasm32"))]
    #[serde(default = "default_data_dir")]
    pub data_dir: String,
}

#[cfg(not(target_arch = "wasm32"))]
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
                starknet_rpc: std::env::var("STARKNET_RPC")
                    .context("STARKNET_RPC env var missing")?,
                #[cfg(not(target_arch = "wasm32"))]
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

pub async fn get_gateway_url(starknet_rpc: &str) -> Result<&'static str> {
    let chain_id = call_method(starknet_rpc, "starknet_chainId").await?;
    match chain_id.as_str() {
        MAINNET_STARKNET_CHAINID => Ok("https://alpha-mainnet.starknet.io"),
        SEPOLIA_STARKNET_CHAINID => Ok("https://alpha-sepolia.starknet.io"),
        _ => eyre::bail!("Unexpected chain id: {}", chain_id),
    }
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

    #[tokio::test]
    async fn wrong_urls() {
        let config = ServerConfig {
            client: Config {
                starknet_rpc: "bar".to_string(),
                data_dir: Default::default(),
            },
            poll_secs: 300,
            rpc_addr: SocketAddr::from(([0, 0, 0, 0], 3030)),
        };
        let response = config.client.validate();

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("starknet_rpc"));
    }

    #[tokio::test]
    async fn wrong_poll_secs() {
        let config = ServerConfig {
            client: Config {
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
