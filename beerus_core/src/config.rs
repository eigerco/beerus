use core::str::FromStr;

use ethers::types::Address;
use eyre::{eyre, Result};
use helios::config::{checkpoints, networks::Network};

#[cfg(feature = "std")]
use std::path::PathBuf;

#[cfg(feature = "std")]
use std::string::String;

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

#[cfg(feature = "std")]
use std::format;

#[cfg(not(feature = "std"))]
use alloc::format;

pub const DEFAULT_ETHEREUM_NETWORK: &str = "goerli";
// By default, we use the Ethereum Mainnet value.
// const DEFAULT_STARKNET_CORE_CONTRACT_ADDRESS: &str = "0xc662c410C0ECf747543f5bA90660f6ABeBD9C8c4";
// For testing purpose use Goerli address until we make it configurable
pub const DEFAULT_STARKNET_CORE_CONTRACT_ADDRESS: &str =
    "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";
pub const DEFAULT_DATA_DIR: &str = "/tmp";
/// Global configuration.
#[derive(Clone, PartialEq)]
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
    pub starknet_core_contract_address: Address,
    // Path to storage directory
    #[cfg(feature = "std")]
    pub data_dir: Option<PathBuf>,
}

impl Config {
    /// Create a new global configuration from environment variables.
    #[cfg(feature = "std")]
    pub fn new_from_env() -> Result<Self> {
        let ethereum_network = std::env::var("ETHEREUM_NETWORK")
            .unwrap_or_else(|_| DEFAULT_ETHEREUM_NETWORK.to_string());
        let ethereum_consensus_rpc = std::env::var("ETHEREUM_CONSENSUS_RPC_URL").map_err(|_| {
            eyre!("Missing mandatory environment variable: ETHEREUM_CONSENSUS_RPC_URL")
        })?;
        let ethereum_execution_rpc = std::env::var("ETHEREUM_EXECUTION_RPC_URL").map_err(|_| {
            eyre!("Missing mandatory environment variable: ETHEREUM_EXECUTION_RPC_URL")
        })?;
        let starknet_rpc = std::env::var("STARKNET_RPC_URL")
            .map_err(|_| eyre!("Missing mandatory environment variable: STARKNET_RPC_URL"))?;
        let starknet_core_contract_address = std::env::var("STARKNET_CORE_CONTRACT_ADDRESS")
            .unwrap_or_else(|_| DEFAULT_STARKNET_CORE_CONTRACT_ADDRESS.to_string());
        let starknet_core_contract_address = Address::from_str(&starknet_core_contract_address)?;
        let data_dir_str =
            std::env::var("DATA_DIR").unwrap_or_else(|_| DEFAULT_DATA_DIR.to_string());
        #[cfg(feature = "std")]
        let data_dir = PathBuf::from(data_dir_str);

        Ok(Self {
            ethereum_network,
            ethereum_consensus_rpc,
            ethereum_execution_rpc,
            starknet_rpc,
            starknet_core_contract_address,
            #[cfg(feature = "std")]
            data_dir: Some(data_dir),
        })
    }

    #[cfg(not(feature = "std"))]
    pub fn new_from_env() -> Result<Self> {
        let ethereum_network = "Ethereum_network".to_string();
        let ethereum_consensus_rpc = "Ethereum_consensus_rpc".to_string();
        let ethereum_execution_rpc = "Ethereum_execution_rpc".to_string();
        let starknet_rpc = "Starknet_rpc".to_string();
        let starknet_core_contract_address = "Starknet_core_contract_address".to_string();
        let starknet_core_contract_address = Address::from_str(&starknet_core_contract_address)?;
        #[cfg(feature = "std")]
        let data_dir_str = "data_dir";
        #[cfg(feature = "std")]
        let data_dir = PathBuf::from(data_dir_str);

        Ok(Self {
            ethereum_network,
            ethereum_consensus_rpc,
            ethereum_execution_rpc,
            starknet_rpc,
            starknet_core_contract_address,
            #[cfg(feature = "std")]
            data_dir: Some(data_dir),
        })
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
                return Ok(format!("{_checkpoint:x}"));
            }
            "goerli" => {
                let _checkpoint = cf.fetch_latest_checkpoint(&Network::GOERLI).await?;
                return Ok(format!("{_checkpoint:x}"));
            }
            _ => return Err(eyre!("Invalid network")),
        };
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new_from_env().unwrap()
    }
}
