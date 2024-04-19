use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use ethers::types::Address;
use eyre::{eyre, Context, Result};
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
    pub fn from_env() -> Result<Self> {
        Self::from_vars(|key| std::env::var(key).ok())
    }

    fn from_vars<F>(get: F) -> Result<Self>
    where
        F: Fn(&'static str) -> Option<String> + 'static,
    {
        let require = |var_key: &'static str| {
            get(var_key).ok_or_else(|| eyre!("The \"{}\" env var must be set or a configuration file must be specified", var_key))
        };

        Ok(Self {
            network: Network::from_str(&get("NETWORK").unwrap_or_default())
                .unwrap_or(Network::MAINNET),
            eth_execution_rpc: require("ETH_EXECUTION_RPC")?,
            starknet_rpc: require("STARKNET_RPC")?,
            data_dir: PathBuf::from(get("DATA_DIR").unwrap_or_default()),
            poll_secs: u64::from_str(&get("POLL_SECS").unwrap_or_default())
                .unwrap_or(DEFAULT_POLL_SECS),
            rpc_addr: match get("RPC_ADDR") {
                Some(addr) => SocketAddr::from_str(&addr)
                    .context("Invalid value for `RPC_ADDR`")?,
                None => rpc_addr(),
            },
            fee_token_addr: match get("FEE_TOKEN_ADDR") {
                Some(addr) => FieldElement::from_hex_be(&addr)
                    .context("Invalid value for `FEE_TOKEN_ADDR`")?,
                None => fee_token_addr(),
            },
        })
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
    use std::collections::HashMap;

    use super::*;

    fn case(vars: &[(&'static str, &'static str)]) -> Result<Config> {
        let vars: HashMap<&str, &str> = HashMap::from_iter(vars.to_vec());
        Config::from_vars(move |s| vars.get(s).map(|s| s.to_string()))
    }

    static MIN_CONFIG_VARS: &[(&'static str, &'static str)] =
        &[("ETH_EXECUTION_RPC", "url"), ("STARKNET_RPC", "url")];

    #[test]
    fn test_min_config_requirements() {
        assert!(case(&[("ETH_EXECUTION_RPC", "url"),]).is_err());
        assert!(case(&[("STARKNET_RPC", "url"),]).is_err());

        assert!(case(&MIN_CONFIG_VARS).is_ok());
    }

    #[test]
    fn test_unspecified_network_is_mainnet() {
        let config = case(&MIN_CONFIG_VARS).unwrap();
        assert_eq!(config.network, Network::MAINNET);
    }

    #[test]
    fn test_rpc_address_is_validated() {
        let result = case(&[
            ("ETH_EXECUTION_RPC", "url"),
            ("STARKNET_RPC", "url"),
            ("RPC_ADDR", "invalid_value"),
        ]);
        assert!(result.is_err());

        let result = case(&[
            ("ETH_EXECUTION_RPC", "url"),
            ("STARKNET_RPC", "url"),
            ("RPC_ADDR", "127.0.0.1:3333"),
        ]);
        assert!(result.is_ok());
        assert_eq!(
            SocketAddr::from_str("127.0.0.1:3333").unwrap(),
            result.unwrap().rpc_addr
        );

        // Default test case
        let config = case(&MIN_CONFIG_VARS).unwrap();
        assert_eq!(config.rpc_addr, rpc_addr());
    }

    #[test]
    fn test_fee_token_addr_is_validated() {
        let result = case(&[
            ("ETH_EXECUTION_RPC", "url"),
            ("STARKNET_RPC", "url"),
            ("FEE_TOKEN_ADDR", "invalid_value"),
        ]);
        assert!(result.is_err());

        let result = case(&[
            ("ETH_EXECUTION_RPC", "url"),
            ("STARKNET_RPC", "url"),
            ("FEE_TOKEN_ADDR", "1"),
        ]);
        assert!(result.is_ok());
        assert_eq!(FieldElement::ONE, result.unwrap().fee_token_addr);

        // Default test case
        let config = case(&MIN_CONFIG_VARS).unwrap();
        assert_eq!(config.fee_token_addr, fee_token_addr());
    }
}
