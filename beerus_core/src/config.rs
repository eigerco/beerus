use std::str::FromStr;

use ethers::types::Address;
use eyre::{eyre, Result};
use helios::config::networks::Network;
use serde::{Deserialize, Serialize};

const DEFAULT_ETHEREUM_NETWORK: &str = "goerli";
// By default, we use the Ethereum Mainnet value.
//const DEFAULT_STARKNET_CORE_CONTRACT_ADDRESS: &str = "0xc662c410C0ECf747543f5bA90660f6ABeBD9C8c4";
// For testing purpose use Goerli address until we make it configurable
const DEFAULT_STARKNET_CORE_CONTRACT_ADDRESS: &str = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";

/// Global configuration.
#[derive(Serialize, Deserialize)]
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
}

impl Config {
    /// Create a new global configuration from environment variables.
    pub fn new_from_env() -> Result<Self> {
        let ethereum_network = std::env::var("ETHEREUM_NETWORK")
            .unwrap_or_else(|_| DEFAULT_ETHEREUM_NETWORK.to_string());
        let ethereum_consensus_rpc = std::env::var("ETHEREUM_CONSENSUS_RPC_URL")?;
        let ethereum_execution_rpc = std::env::var("ETHEREUM_EXECUTION_RPC_URL")?;
        let starknet_rpc = std::env::var("STARKNET_RPC_URL")?;
        let starknet_core_contract_address = std::env::var("STARKNET_CORE_CONTRACT_ADDRESS")
            .unwrap_or_else(|_| DEFAULT_STARKNET_CORE_CONTRACT_ADDRESS.to_string());
        let starknet_core_contract_address = Address::from_str(&starknet_core_contract_address)?;

        Ok(Self {
            ethereum_network,
            ethereum_consensus_rpc,
            ethereum_execution_rpc,
            starknet_rpc,
            starknet_core_contract_address,
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
}

impl Default for Config {
    fn default() -> Self {
        Self::new_from_env().unwrap()
    }
}
