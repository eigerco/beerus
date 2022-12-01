use crate::config::Config;
use async_trait::async_trait;
use ethers::{
    abi::{Abi, AbiError, Tokenize},
    types::{Bytes, U256},
};

use eyre::Result;
use helios::types::BlockTag;
use helios::types::CallOpts;

use super::{
    ethereum::{ethereum::EthereumLightClient, helios::HeliosLightClient},
    starknet::StarkNetLightClient,
};

pub enum SyncStatus {
    NotSynced,
    Syncing,
    Synced,
}

#[async_trait]
pub trait Beerus {
    async fn start(&mut self) -> Result<()>;
    fn sync_status(&self) -> SyncStatus;
    async fn starknet_state_root(&self) -> Result<U256>;
}

/// Beerus Light Client service.
pub struct BeerusLightClient<T: EthereumLightClient> {
    /// Global configuration.
    pub config: Config,
    /// Ethereum light client.
    pub ethereum_lightclient: T,
    /// StarkNet light client.
    pub starknet_lightclient: StarkNetLightClient,
}

impl BeerusLightClient<HeliosLightClient> {
    /// Create a new Beerus Light Client service.
    pub fn new(config: Config) -> Result<Self> {
        // Build the Ethereum light client.
        let ethereum_lightclient = HeliosLightClient::new(&config)?;
        // Build the StarkNet light client.
        let starknet_lightclient = StarkNetLightClient::new(&config)?;
        Ok(Self {
            config,
            ethereum_lightclient,
            starknet_lightclient,
        })
    }
}
/// Helper for ABI encoding arguments for a specific function
fn encode_function_data<T: Tokenize>(args: T, abi: Abi, name: &str) -> Result<Bytes, AbiError> {
    let function = abi.function(name)?;
    let tokens = args.into_tokens();
    Ok(function.encode_input(&tokens).map(Into::into)?)
}

#[async_trait]
impl Beerus for BeerusLightClient<HeliosLightClient> {
    /// Start Beerus light client and synchronize with Ethereum and StarkNet.
    async fn start(&mut self) -> Result<()> {
        // Start the Ethereum light client.
        self.ethereum_lightclient.start().await?;
        // Start the StarkNet light client.
        self.starknet_lightclient.start().await?;
        Ok(())
    }

    fn sync_status(&self) -> SyncStatus {
        todo!()
    }

    /// Get the StarkNet state root.
    async fn starknet_state_root(&self) -> Result<U256> {
        // Get the StarkNet core contract address.
        let starknet_core_contract_address = &self.config.starknet_core_contract_address;

        let abi: Abi = serde_json::from_str(
            r#"[{"inputs":[],"name":"stateRoot","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"}]"#,
        )?;
        let data = encode_function_data((), abi, "stateRoot")?;
        let data = data.to_vec();

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: *starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let starknet_root = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;

        // Convert the response bytes to a U256.
        let starknet_root = U256::from_big_endian(&starknet_root);

        Ok(starknet_root)
    }
}
