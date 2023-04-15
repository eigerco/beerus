#[cfg(not(feature = "std"))]
use core::str::FromStr;
use ethers::types::Address;
use eyre::{eyre, Result};
use helios::config::{checkpoints, networks::Network};
#[cfg(feature = "std")]
use log::{error, info};
use serde::Deserialize;
use shellexpand::tilde;
use std::{env, fs, net::SocketAddr, path::PathBuf, str::FromStr};

use crate::stdlib::string::{String, ToString};

pub const STARKNET_MAINNET_CC_ADDRESS: &str = "0xc662c410C0ECf747543f5bA90660f6ABeBD9C8c4";
pub const STARKNET_GOERLI_CC_ADDRESS: &str = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";
pub const DEFAULT_ETHEREUM_NETWORK: &str = "goerli";
pub const DEFAULT_DATA_DIR: &str = "~/.beerus/tmp";
pub const DEFAULT_POLL_INTERVAL_SECS: u64 = 5;
pub const DEFAULT_BEERUS_RPC_ADDR: &str = "0.0.0.0:3030";
pub const DEFAULT_HELIOS_RPC_ADDR: u16 = 3031;

/// Global configuration.
#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Config {
    pub ethereum_network: String,
    pub ethereum_consensus_rpc: String,
    pub ethereum_execution_rpc: String,
    pub starknet_rpc: String,
    #[serde(skip_deserializing)]
    pub starknet_core_contract_address: Address,
    #[cfg(feature = "std")]
    pub data_dir: PathBuf,
    pub poll_interval_secs: Option<u64>,
    #[cfg(feature = "std")]
    pub beerus_rpc_address: Option<SocketAddr>,
    #[cfg(feature = "std")]
    pub helios_rpc_address: Option<u16>,
}

impl Config {
    #[cfg(feature = "std")]
    pub fn from_env() -> Self {
        // if BEERUS_CONFIG environment variable is set -> use config file
        if let Ok(path) = std::env::var("BEERUS_CONFIG") {
            let buf = PathBuf::from(path);
            return Self::from_file(&buf);
        }

        let mut config = Self::default();
        if let Ok(network) = std::env::var("ETHEREUM_NETWORK") {
            match network.as_str() {
                "goerli" => {
                    info!("ethereum network: goerli(default)");
                }
                "mainnet" => {
                    info!("ethereum network: mainnet");
                    config.ethereum_network = network.to_string();
                    config.starknet_core_contract_address =
                        Address::from_str(STARKNET_MAINNET_CC_ADDRESS).unwrap();
                }
                _ => {
                    error! {"invalid network"};
                    panic!();
                }
            };
        };

        config.ethereum_consensus_rpc = string_env_or_die("ETHEREUM_CONSENSUS_RPC_URL");
        config.ethereum_execution_rpc = string_env_or_die("ETHEREUM_EXECUTION_RPC_URL");
        config.starknet_rpc = string_env_or_die("STARKNET_RPC_URL");
        if let Ok(dir) = std::env::var("DATA_DIR") {
            config.data_dir = PathBuf::from(tilde(&dir).to_string());
        }

        if let Ok(raw_addr) = std::env::var("BEERUS_RPC_ADDR") {
            if let Ok(addr) = SocketAddr::from_str(raw_addr.as_str()) {
                config.beerus_rpc_address = Some(addr);
            }
        }

        if let Ok(raw_addr) = std::env::var("HELIOS_RPC_ADDR") {
            config.helios_rpc_address = Some(raw_addr.parse().unwrap());
        }

        config
    }

    #[cfg(feature = "std")]
    pub fn from_file(path: &PathBuf) -> Self {
        info!("Config file: {:?}", path);
        let raw_config = match fs::read_to_string(path) {
            Ok(r) => r,
            Err(e) => {
                error! {"Config file read error: {e}"};
                panic!();
            }
        };

        let mut config: Config = match toml::from_str(&raw_config) {
            Ok(c) => c,
            Err(e) => {
                error! {"Config file read error: {e}"};
                panic!();
            }
        };

        config.starknet_core_contract_address = match config.ethereum_network.as_str() {
            "mainnet" => Address::from_str(STARKNET_MAINNET_CC_ADDRESS).unwrap(),
            _ => Address::from_str(STARKNET_GOERLI_CC_ADDRESS).unwrap(),
        };

        if config.poll_interval_secs.is_none() {
            config.poll_interval_secs = Some(DEFAULT_POLL_INTERVAL_SECS);
        }

        if config.beerus_rpc_address.is_none() {
            config.beerus_rpc_address =
                Some(SocketAddr::from_str(DEFAULT_BEERUS_RPC_ADDR).unwrap());
        }

        if config.helios_rpc_address.is_none() {
            config.helios_rpc_address = Some(DEFAULT_HELIOS_RPC_ADDR);
        }

        config
    }

    #[cfg(not(feature = "std"))]
    pub fn from_args(
        network: String,
        consensus_rpc: String,
        execution_rpc: String,
        starknet_rpc: String,
    ) -> Self {
        let starknet_cc = match network.as_str() {
            "mainnet" => Address::from_str(STARKNET_MAINNET_CC_ADDRESS).unwrap(),
            _ => Address::from_str(STARKNET_GOERLI_CC_ADDRESS).unwrap(),
        };

        Self {
            ethereum_network: network.to_string(),
            ethereum_consensus_rpc: consensus_rpc.to_string(),
            ethereum_execution_rpc: execution_rpc.to_string(),
            starknet_rpc: starknet_rpc.to_string(),
            starknet_core_contract_address: starknet_cc,
            poll_interval_secs: Some(DEFAULT_POLL_INTERVAL_SECS),
        }
    }

    /// Return the Ethereum network.
    pub fn ethereum_network(&self) -> Result<Network> {
        match self.ethereum_network.to_lowercase().as_str() {
            "goerli" => Ok(Network::GOERLI),
            "mainnet" => Ok(Network::MAINNET),
            _ => Err(eyre!("Invalid network")),
        }
    }

    pub fn get_poll_interval(&self) -> u64 {
        match self.poll_interval_secs {
            Some(s) => s,
            None => DEFAULT_POLL_INTERVAL_SECS,
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

    #[cfg(feature = "std")]
    pub fn clean_env() {
        env::remove_var("BEERUS_CONFIG");
        env::remove_var("ETHEREUM_NETWORK");
        env::remove_var("ETHEREUM_CONSENSUS_RPC_URL");
        env::remove_var("ETHEREUM_EXECUTION_RPC_URL");
        env::remove_var("STARKNET_RPC_URL");
        env::remove_var("DATA_DIR");
        env::remove_var("BEERUS_RPC_ADDR");
        env::remove_var("HELIOS_RPC_ADDR");
    }
}

#[cfg(feature = "std")]
fn string_env_or_die(env_var: &str) -> String {
    match std::env::var(env_var) {
        Ok(res) => res,
        Err(e) => {
            error! {"{env_var}: {e}"};
            panic!();
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ethereum_network: DEFAULT_ETHEREUM_NETWORK.to_string(),
            ethereum_consensus_rpc: "http://localhost:8545".to_string(),
            ethereum_execution_rpc: "http://localhost:5054".to_string(),
            starknet_rpc: "http://localhost:9545".to_string(),
            starknet_core_contract_address: Address::from_str(STARKNET_GOERLI_CC_ADDRESS).unwrap(),
            #[cfg(feature = "std")]
            data_dir: PathBuf::from(tilde(DEFAULT_DATA_DIR).to_string()),
            poll_interval_secs: Some(DEFAULT_POLL_INTERVAL_SECS),
            #[cfg(feature = "std")]
            beerus_rpc_address: Some(SocketAddr::from_str(DEFAULT_BEERUS_RPC_ADDR).unwrap()),
            #[cfg(feature = "std")]
            helios_rpc_address: Some(DEFAULT_HELIOS_RPC_ADDR),
        }
    }
}
