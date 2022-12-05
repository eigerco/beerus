use crate::{config::Config, ethers_helper};
use ethers::{abi::Abi, types::U256};

use eyre::Result;
use helios::types::BlockTag;
use helios::types::CallOpts;

use super::{ethereum::ethereum::EthereumLightClient, starknet::StarkNetLightClient};

/// Enum representing the different synchrnization status of the light client.
pub enum SyncStatus {
    NotSynced,
    Syncing,
    Synced,
}

/// Beerus Light Client service.
pub struct BeerusLightClient<'a> {
    /// Global configuration.
    pub config: &'a Config,
    /// Ethereum light client.
    pub ethereum_lightclient: Box<&'a mut dyn EthereumLightClient>,
    /// StarkNet light client.
    pub starknet_lightclient: StarkNetLightClient,
    /// Sync status.
    pub sync_status: SyncStatus,
}

impl<'a> BeerusLightClient<'a> {
    /// Create a new Beerus Light Client service.
    pub fn new(
        config: &'a Config,
        ethereum_lightclient: &'a mut dyn EthereumLightClient,
        starknet_lightclient: StarkNetLightClient,
    ) -> Result<Self> {
        Ok(Self {
            config,
            ethereum_lightclient: Box::new(ethereum_lightclient),
            starknet_lightclient,
            sync_status: SyncStatus::NotSynced,
        })
    }

    /// Start Beerus light client and synchronize with Ethereum and StarkNet.
    pub async fn start(&mut self) -> Result<()> {
        match self.sync_status {
            // If the light client is not synced, start the synchronization.
            SyncStatus::NotSynced => {
                // Start the Ethereum light client.
                self.ethereum_lightclient.start().await?;
                // Start the StarkNet light client.
                self.starknet_lightclient.start().await?;
                self.sync_status = SyncStatus::Synced;
            }
            // If the light client is already syncing or not synced, do nothing.
            _ => (),
        }
        Ok(())
    }

    /// Return the current syncrhonization status.
    pub fn sync_status(&self) -> &SyncStatus {
        &self.sync_status
    }

    /// Get the StarkNet state root.
    pub async fn starknet_state_root(&self) -> Result<U256> {
        // Get the StarkNet core contract address.
        let starknet_core_contract_address = &self.config.starknet_core_contract_address;

        let abi: Abi = serde_json::from_str(
            r#"[{"inputs":[],"name":"stateRoot","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"}]"#,
        )?;
        let data = ethers_helper::encode_function_data((), abi, "stateRoot")?;
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
