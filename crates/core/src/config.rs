use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use ethers::types::Address;
use eyre::{eyre, Result};
use helios::client::{Client, ClientBuilder};
use helios::config::checkpoints;
use helios::config::networks::Network;
use helios::prelude::FileDB;
use serde::Deserialize;
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use url::Url;

// defaults
pub const DEFAULT_DATA_DIR: &str = "tmp";
pub const DEFAULT_POLL_SECS: u64 = 5;

// mainnet constants
pub const MAINNET_CC_ADDRESS: &str = "c662c410C0ECf747543f5bA90660f6ABeBD9C8c4";
pub const MAINNET_CONSENSUS_RPC: &str = "https://www.lightclientdata.org";
pub const MAINNET_FALLBACK_RPC: &str = "https://sync-mainnet.beaconcha.in";

// testnet constants
pub const TESTNET_CC_ADDRESS: &str = "de29d060D45901Fb19ED6C6e959EB22d8626708e";
pub const TESTNET_CONSENSUS_RPC: &str = "http://testing.prater.beacon-api.nimbus.team";
pub const TESTNET_FALLBACK_RPC: &str = "https://sync-goerli.beaconcha.in";

/// global config
#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub network: Network,
    pub eth_execution_rpc: String,
    pub starknet_rpc: String,
    pub data_dir: PathBuf,
    pub poll_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: Network::MAINNET,
            eth_execution_rpc: "http://localhost:5054".to_string(),
            starknet_rpc: "http://localhost:9545".to_string(),
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
            poll_secs: DEFAULT_POLL_SECS,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            network: Network::from_str(&std::env::var("NETWORK").unwrap_or_default()).unwrap_or(Network::MAINNET),
            eth_execution_rpc: std::env::var("ETH_EXECUTION_RPC").unwrap_or_default(),
            starknet_rpc: std::env::var("STARKNET_RPC").unwrap_or_default(),
            data_dir: PathBuf::from(std::env::var("DATA_DIR").unwrap_or_default()),
            poll_secs: u64::from_str(&std::env::var("POLL_SECS").unwrap_or_default()).unwrap_or(DEFAULT_POLL_SECS),
        }
    }

    pub fn from_file(path: &str) -> Self {
        let raw_conf = fs::read_to_string(path).expect(&format!("unable to read file: {path}"));

        if path.contains(".toml") {
            toml::from_str(&raw_conf).unwrap()
        } else if path.contains(".json") {
            serde_json::from_str(&raw_conf).unwrap()
        } else {
            Self::default()
        }
    }

    pub fn get_core_contract_address(&self) -> Address {
        match self.network {
            Network::MAINNET => Address::from_str(MAINNET_CC_ADDRESS).expect("should not fail mainnet addr"),
            Network::GOERLI => Address::from_str(TESTNET_CC_ADDRESS).expect("should not fail testnet addr"),
            Network::SEPOLIA => todo!(),
        }
    }

    pub fn get_consensus_rpc(&self) -> String {
        match self.network {
            Network::MAINNET => MAINNET_CONSENSUS_RPC.to_string(),
            Network::GOERLI => TESTNET_CC_ADDRESS.to_string(),
            Network::SEPOLIA => todo!(),
        }
    }

    pub fn get_fallback_address(&self) -> String {
        match self.network {
            Network::MAINNET => MAINNET_FALLBACK_RPC.to_string(),
            Network::GOERLI => TESTNET_FALLBACK_RPC.to_string(),
            Network::SEPOLIA => todo!(),
        }
    }

    pub async fn get_checkpoint(&self) -> Result<String> {
        let cf = checkpoints::CheckpointFallback::new().build().await?;

        match self.network {
            Network::MAINNET => {
                let checkpoint = cf.fetch_latest_checkpoint(&Network::MAINNET).await?;
                Ok(format!("{checkpoint:x}"))
            }
            Network::GOERLI => {
                let checkpoint = cf.fetch_latest_checkpoint(&Network::GOERLI).await?;
                Ok(format!("{checkpoint:x}"))
            }
            _ => Err(eyre!("Invalid network")),
        }
    }

    pub fn to_starknet_client(&self) -> JsonRpcClient<HttpTransport> {
        JsonRpcClient::new(HttpTransport::new(Url::parse(&self.starknet_rpc.clone()).unwrap()))
    }

    pub async fn to_helios_client(&self) -> Client<FileDB> {
        ClientBuilder::new()
            .network(self.network)
            .data_dir(self.data_dir.clone())
            .consensus_rpc(&self.get_consensus_rpc())
            .execution_rpc(&self.eth_execution_rpc)
            .checkpoint(&self.get_checkpoint().await.expect("unable to retrieve checkpoint"))
            .load_external_fallback()
            .fallback(&self.get_consensus_rpc())
            .build()
            .expect("incorrect helios client config")
    }
}
