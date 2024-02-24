use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
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

// defaults
pub const DEFAULT_DATA_DIR: &str = "tmp";
pub const DEFAULT_POLL_SECS: u64 = 5;
pub const DEFAULT_FEE_TOKEN_ADDR: &str =
    "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
const DEFAULT_IP_V4_ADDR: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3030;

// mainnet constants
pub const MAINNET_CC_ADDRESS: &str = "c662c410C0ECf747543f5bA90660f6ABeBD9C8c4";
pub const MAINNET_CONSENSUS_RPC: &str = "https://www.lightclientdata.org";
pub const MAINNET_FALLBACK_RPC: &str = "https://sync-mainnet.beaconcha.in";

// testnet constants
pub const TESTNET_CC_ADDRESS: &str = "de29d060D45901Fb19ED6C6e959EB22d8626708e";
pub const TESTNET_CONSENSUS_RPC: &str =
    "http://testing.prater.beacon-api.nimbus.team";
pub const TESTNET_FALLBACK_RPC: &str = "https://sync-goerli.beaconcha.in";

/// global config
#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub network: Network,
    pub eth_execution_rpc: String,
    pub starknet_rpc: String,
    #[serde(default = "data_dir")]
    pub data_dir: PathBuf,
    #[serde(default = "poll_secs")]
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

        if path.ends_with(".toml") {
            let ret: Self = toml::from_str(&content)?;
            Ok(ret)
        } else if path.ends_with(".json") {
            let ret: Self = serde_json::from_str(&content)?;
            Ok(ret)
        } else {
            Err(eyre!("Unsupported config file format"))
        }
    }

    pub fn get_core_contract_address(&self) -> Result<Address> {
        match self.network {
            Network::MAINNET => Ok(Address::from_str(MAINNET_CC_ADDRESS)?),
            Network::GOERLI => Ok(Address::from_str(TESTNET_CC_ADDRESS)?),
            network => eyre::bail!("unsupported network: {network:?}")
        }
    }

    pub fn get_consensus_rpc(&self) -> Result<String> {
        match self.network {
            Network::MAINNET => Ok(MAINNET_CONSENSUS_RPC.to_owned()),
            Network::GOERLI => Ok(TESTNET_CONSENSUS_RPC.to_owned()),
            network => eyre::bail!("unsupported network: {network:?}")
        }
    }

    pub fn get_fallback_address(&self) -> Result<String> {
        match self.network {
            Network::MAINNET => Ok(MAINNET_FALLBACK_RPC.to_owned()),
            Network::GOERLI => Ok(TESTNET_FALLBACK_RPC.to_owned()),
            network => eyre::bail!("unsupported network: {network:?}")
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
            Network::GOERLI => {
                let checkpoint =
                    cf.fetch_latest_checkpoint(&Network::GOERLI).await?;
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

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn to_helios_client(&self) -> Client<FileDB> {
        let consensus_rpc = self.get_consensus_rpc().expect("unable to retrieve consensus url");
        ClientBuilder::new()
            .network(self.network)
            .data_dir(self.data_dir.clone())
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
            .build()
            .expect("incorrect helios client config")
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn to_helios_client(&self) -> Client<ConfigDB> {
        ClientBuilder::new()
            .network(self.network)
            .consensus_rpc(&self.get_consensus_rpc())
            .execution_rpc(&self.eth_execution_rpc)
            .checkpoint(
                &self
                    .get_checkpoint()
                    .await
                    .expect("unable to retrieve checkpoint"),
            )
            .load_external_fallback()
            .fallback(&self.get_consensus_rpc())
            .build()
            .expect("incorrect helios client config")
    }
}
