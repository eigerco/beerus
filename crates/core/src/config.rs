use std::ffi::OsStr;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use ethers::types::Address;
use eyre::{eyre, Result};
use helios::client::{Client, ClientBuilder};
use helios::config::checkpoints;
use helios::config::networks::Network;
#[cfg(target_arch = "wasm32")]
use helios::prelude::ConfigDB;
#[cfg(not(target_arch = "wasm32"))]
use helios::prelude::FileDB;
use serde::Deserialize;
use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use url::Url;
use validator::Validate;

// defaults
pub const DEFAULT_DATA_DIR: &str = "tmp";
pub const DEFAULT_POLL_SECS: u64 = 5;
pub const DEFAULT_FEE_TOKEN_ADDR: &str =
    "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
const DEFAULT_IP_V4_ADDR: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 3030;

// mainnet constants
pub const MAINNET_CC_ADDRESS: &str = "c662c410C0ECf747543f5bA90660f6ABeBD9C8c4";
pub const MAINNET_CONSENSUS_RPC: &str = "https://www.lightclientdata.org";
pub const MAINNET_FALLBACK_RPC: &str = "https://sync-mainnet.beaconcha.in";

// sepolia testnet constants
pub const SEPOLIA_CC_ADDRESS: &str = "E2Bb56ee936fd6433DC0F6e7e3b8365C906AA057";
pub const SEPOLIA_CONSENSUS_RPC: &str =
    "http://unstable.sepolia.beacon-api.nimbus.team";
pub const SEPOLIA_FALLBACK_RPC: &str = "https://sync-sepolia.beaconcha.in";

enum Blockchain {
    Ethereum,
    Starknet,
}

/// global config
#[derive(Clone, Deserialize, Debug, Validate)]
pub struct Config {
    pub network: Network,
    #[validate(url)]
    pub eth_execution_rpc: String,
    #[validate(url)]
    pub starknet_rpc: String,
    #[serde(default = "data_dir")]
    pub data_dir: PathBuf,
    #[serde(default = "poll_secs")]
    #[validate(range(min = 1, max = 10))]
    pub poll_secs: u64,
    #[serde(default = "rpc_addr")]
    pub rpc_addr: SocketAddr,
    #[serde(default = "fee_token_addr")]
    pub fee_token_addr: FieldElement,
}

fn data_dir() -> PathBuf {
    PathBuf::from(DEFAULT_DATA_DIR)
}

fn poll_secs() -> u64 {
    DEFAULT_POLL_SECS
}

fn rpc_addr() -> SocketAddr {
    SocketAddr::from_str(&format!("{DEFAULT_IP_V4_ADDR}:{DEFAULT_PORT}"))
        .unwrap()
}

fn fee_token_addr() -> FieldElement {
    FieldElement::from_hex_be(DEFAULT_FEE_TOKEN_ADDR).unwrap()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: Network::MAINNET,
            eth_execution_rpc: "http://localhost:5054".to_string(),
            starknet_rpc: "http://localhost:9545".to_string(),
            data_dir: data_dir(),
            poll_secs: poll_secs(),
            rpc_addr: rpc_addr(),
            fee_token_addr: fee_token_addr(),
        }
    }
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
            rpc_addr: rpc_addr(),
            fee_token_addr: fee_token_addr(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let data_type =
            match Path::new(path).extension().and_then(OsStr::to_str) {
                Some(file_extension) => file_extension,
                None => {
                    return Err(eyre!(
                        "No file extension found in provided config file"
                    ))
                }
            };
        Self::extract_from_file(data_type, content)
    }

    fn extract_from_file(data_type: &str, content: String) -> Result<Self> {
        let config_data: Self = match data_type {
            "toml" => toml::from_str(&content)?,
            "json" => serde_json::from_str(&content)?,
            _ => return Err(eyre!("Unsupported config file format")),
        };

        Ok(config_data)
    }

    pub async fn validate_params(&self) -> Result<()> {
        self.validate()?;
        Self::check_url(&self.eth_execution_rpc, Blockchain::Ethereum).await?;
        Self::check_url(&self.starknet_rpc, Blockchain::Starknet).await?;
        self.validate_data_dir()
    }

    async fn check_url(url: &str, blockchain: Blockchain) -> Result<()> {
        let name = match blockchain {
            Blockchain::Ethereum => "eth",
            Blockchain::Starknet => "starknet",
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .body(format!(
                r#"{{"jsonrpc":"2.0","method":"{name}_blockNumber","params":[],"id":0}}"#
            ))
            .send()
            .await
            .expect("Failed to execute request");

        match response.status().is_success() {
            true => Ok(()),
            false => Err(eyre!("Wrong response, check {name} url and api key")),
        }
    }

    fn validate_data_dir(&self) -> Result<()> {
        if !self.data_dir.exists() {
            tracing::warn!("Provided data dir does not exist");
            return Ok(());
        };

        let permissions = match self.data_dir.metadata() {
            Ok(val) => val.permissions(),
            Err(e) => return Err(e.into()),
        };

        match permissions.readonly() {
            false => Ok(()),
            true => Err(eyre!("Unable to write, read only folder")),
        }
    }

    pub fn get_core_contract_address(&self) -> Result<Address> {
        match self.network {
            Network::MAINNET => Ok(Address::from_str(MAINNET_CC_ADDRESS)?),
            Network::SEPOLIA => Ok(Address::from_str(SEPOLIA_CC_ADDRESS)?),
            network => eyre::bail!("unsupported network: {network:?}"),
        }
    }

    pub fn get_consensus_rpc(&self) -> Result<String> {
        match self.network {
            Network::MAINNET => Ok(MAINNET_CONSENSUS_RPC.to_owned()),
            Network::SEPOLIA => Ok(SEPOLIA_CONSENSUS_RPC.to_owned()),
            network => eyre::bail!("unsupported network: {network:?}"),
        }
    }

    pub fn get_fallback_address(&self) -> Result<String> {
        match self.network {
            Network::MAINNET => Ok(MAINNET_FALLBACK_RPC.to_owned()),
            Network::SEPOLIA => Ok(SEPOLIA_FALLBACK_RPC.to_owned()),
            network => eyre::bail!("unsupported network: {network:?}"),
        }
    }

    pub async fn get_checkpoint(&self) -> Result<String> {
        let cf = checkpoints::CheckpointFallback::new().build().await?;

        match self.network {
            Network::MAINNET => {
                let checkpoint =
                    cf.fetch_latest_checkpoint(&Network::MAINNET).await?;
                Ok(format!("{checkpoint:x}"))
            }
            Network::SEPOLIA => {
                let checkpoint =
                    cf.fetch_latest_checkpoint(&Network::SEPOLIA).await?;
                Ok(format!("{checkpoint:x}"))
            }
            network => Err(eyre!("unsupported network: {network:?}")),
        }
    }

    pub fn to_starknet_client(&self) -> JsonRpcClient<HttpTransport> {
        let url: Url = self.starknet_rpc.as_str().try_into().unwrap();
        let transport = HttpTransport::new(url);
        JsonRpcClient::new(transport)
    }

    async fn to_helios_client_builder(&self) -> ClientBuilder {
        let consensus_rpc =
            self.get_consensus_rpc().expect("unable to retrieve consensus url");
        ClientBuilder::new()
            .network(self.network)
            .consensus_rpc(&consensus_rpc)
            .execution_rpc(&self.eth_execution_rpc)
            .checkpoint(
                &self
                    .get_checkpoint()
                    .await
                    .expect("unable to retrieve checkpoint"),
            )
            .load_external_fallback()
            .fallback(&consensus_rpc)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn to_helios_client(&self) -> Client<FileDB> {
        self.to_helios_client_builder()
            .await
            .data_dir(self.data_dir.clone())
            .build()
            .expect("incorrect helios client config")
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn to_helios_client(&self) -> Client<ConfigDB> {
        self.to_helios_client_builder()
            .await
            .build()
            .expect("incorrect helios client config")
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{matchers::any, Mock, MockServer, ResponseTemplate};

    use crate::config::{Blockchain, Config};

    #[tokio::test]
    async fn wrong_data_type() {
        let data_type = "yml";
        let content = "".to_string();
        assert!(Config::extract_from_file(data_type, content).is_err());
    }

    #[tokio::test]
    async fn wrong_content() {
        let data_type = "toml";
        let content = "".to_string();
        assert!(Config::extract_from_file(data_type, content).is_err());
    }

    #[tokio::test]
    async fn wrong_urls() {
        let mut config = Config::default();
        config.eth_execution_rpc = "foo".to_string();
        config.starknet_rpc = "bar".to_string();

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

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&eth_mock_server)
            .await;

        assert!(Config::check_url(
            eth_mock_server.uri().as_str(),
            Blockchain::Ethereum
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn wrong_eth_url() {
        let eth_mock_server = MockServer::start().await;

        Mock::given(any())
            .respond_with(ResponseTemplate::new(400))
            .expect(1)
            .mount(&eth_mock_server)
            .await;

        let response = Config::check_url(
            eth_mock_server.uri().as_str(),
            Blockchain::Ethereum,
        )
        .await;

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("eth"));
    }

    #[tokio::test]
    async fn wrong_starknet_url() {
        let starknet_rpc_server = MockServer::start().await;

        Mock::given(any())
            .respond_with(ResponseTemplate::new(400))
            .expect(1)
            .mount(&starknet_rpc_server)
            .await;

        let response = Config::check_url(
            starknet_rpc_server.uri().as_str(),
            Blockchain::Starknet,
        )
        .await;

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("starknet"));
    }

    #[tokio::test]
    async fn wrong_poll_secs() {
        let mut config = Config::default();
        config.poll_secs = 99;

        let response = config.validate_params().await;

        assert!(response.is_err());
        assert!(response.unwrap_err().to_string().contains("poll_secs"));
    }
}
