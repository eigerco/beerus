use eyre::{eyre, Result};
use helios::config::networks::Network;
use serde::{Deserialize, Serialize};

const DEFAULT_ETHEREUM_NETWORK: &str = "goerli";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ethereum_network: String,
    pub ethereum_consensus_rpc: String,
    pub ethereum_execution_rpc: String,
    pub starknet_rpc: String,
}

impl Config {
    pub fn new_from_env() -> Result<Self> {
        let ethereum_network = std::env::var("ETHEREUM_NETWORK")
            .unwrap_or_else(|_| DEFAULT_ETHEREUM_NETWORK.to_string());
        let ethereum_consensus_rpc = std::env::var("ETHEREUM_CONSENSUS_RPC_URL")?;
        let ethereum_execution_rpc = std::env::var("ETHEREUM_EXECUTION_RPC_URL")?;
        let starknet_rpc = std::env::var("STARKNET_RPC_URL")?;
        Ok(Self {
            ethereum_network,
            ethereum_consensus_rpc,
            ethereum_execution_rpc,
            starknet_rpc,
        })
    }

    pub fn ethereum_network(&self) -> Result<Network> {
        match self.ethereum_network.to_lowercase().as_str() {
            "goerli" => Ok(Network::GOERLI),
            "mainnet" => Ok(Network::MAINNET),
            _ => Err(eyre!("Invalid network")),
        }
    }
}
