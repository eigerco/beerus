use ethers::types::Address;
use eyre::{eyre, Result};
use helios::config::{checkpoints, networks::Network};
use std::path::PathBuf;
use serde::Deserialize;
use std::fs;

pub const STARKNET_MAINNET_CC_ADDRESS: &str = "0xc662c410C0ECf747543f5bA90660f6ABeBD9C8c4";
pub const STARKNET_GOERLI_CC_ADDRESS: &str = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";
pub const DEFAULT_ETHEREUM_NETWORK: &str = "goerli";
pub const DEFAULT_DATA_DIR: &str = "/tmp";

/// Global configuration.
#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Config {
    /// Ethereum network.
    pub ethereum_network: String,
    /// Ethereum consensus RPC endpoint.
    pub ethereum_consensus_rpc: String,
    /// Ethereum execution RPC endpoint.
    pub ethereum_execution_rpc: String,
    /// StarkNet RPC endpoint.
    pub starknet_rpc: String,
    // StarkNet core contract address.
    #[serde(skip_deserializing)]
    pub starknet_core_contract_address: Address,
    // Path to storage directory
    pub data_dir: Option<PathBuf>,
}

impl Config {
    /// Create a new global configuration from environment variables.
    pub fn from_env() -> Result<Self> {
        if let Ok(path) = std::env::var("BEERUS_CONFIG") {
            let buf = PathBuf::from(path);
            return Self::from_file(&buf);
        }

        let ethereum_network = std::env::var("ETHEREUM_NETWORK")
            .unwrap_or_else(|_| DEFAULT_ETHEREUM_NETWORK.to_string());

        let starknet_core_contract_address = match ethereum_network.as_str() {
            DEFAULT_ETHEREUM_NETWORK => Address::from_str(STARKNET_GOERLI_CC_ADDRESS)?,
            _ => Address::from_str(STARKNET_MAINNET_CC_ADDRESS)?,
        };

        let ethereum_consensus_rpc = std::env::var("ETHEREUM_CONSENSUS_RPC_URL").map_err(|_| {
            eyre!("Missing mandatory environment variable: ETHEREUM_CONSENSUS_RPC_URL")
        })?;
        let ethereum_execution_rpc = std::env::var("ETHEREUM_EXECUTION_RPC_URL").map_err(|_| {
            eyre!("Missing mandatory environment variable: ETHEREUM_EXECUTION_RPC_URL")
        })?;
        let starknet_rpc = std::env::var("STARKNET_RPC_URL")
            .map_err(|_| eyre!("Missing mandatory environment variable: STARKNET_RPC_URL"))?;
        let data_dir_str =
            std::env::var("DATA_DIR").unwrap_or_else(|_| DEFAULT_DATA_DIR.to_string());
        let data_dir = PathBuf::from(data_dir_str);

        Ok(Self {
            ethereum_network,
            ethereum_consensus_rpc,
            ethereum_execution_rpc,
            starknet_rpc,
            starknet_core_contract_address,
            data_dir: Some(data_dir),
        })
    }

    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let raw_config = fs::read_to_string(path)?;

        let mut config: Config = toml::from_str(&raw_config)?;

        config.starknet_core_contract_address = match config.ethereum_network.as_str() {
            DEFAULT_ETHEREUM_NETWORK => Address::from_str(STARKNET_GOERLI_CC_ADDRESS)?,
            _ => Address::from_str(STARKNET_MAINNET_CC_ADDRESS)?,
        };

        Ok(config)
    }

    /// Return the Ethereum network.
    pub fn ethereum_network(&self) -> Result<Network> {
        match self.ethereum_network.to_lowercase().as_str() {
            "goerli" => Ok(Network::GOERLI),
            "mainnet" => Ok(Network::MAINNET),
            _ => Err(eyre!("Invalid network")),
        }
    }
    // Return the current checkpoint given the network.
    pub async fn get_checkpoint(&self) -> eyre::Result<String> {
        let cf = checkpoints::CheckpointFallback::new()
            .build()
            .await
            .unwrap();
        match self.ethereum_network.to_lowercase().as_str() {
            "mainnet" => {
                let _checkpoint = cf.fetch_latest_checkpoint(&Network::MAINNET).await?;
                Ok(format!("{_checkpoint:x}"))
            }
            "goerli" => {
                let _checkpoint = cf.fetch_latest_checkpoint(&Network::GOERLI).await?;
                Ok(format!("{_checkpoint:x}"))
            }
            _ => Err(eyre!("Invalid network")),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env().unwrap()
    }
}
