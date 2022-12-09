use crate::config::Config;
use ethers::types::U256;

use eyre::Result;
use helios::types::BlockTag;
use helios::types::CallOpts;
use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::models::FunctionCall;

use super::{ethereum::EthereumLightClient, starknet::StarkNetLightClient};

/// Enum representing the different synchrnization status of the light client.
#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    NotSynced,
    Syncing,
    Synced,
}

/// Beerus Light Client service.
pub struct BeerusLightClient {
    /// Global configuration.
    pub config: Config,
    /// Ethereum light client.
    pub ethereum_lightclient: Box<dyn EthereumLightClient>,
    /// StarkNet light client.
    pub starknet_lightclient: Box<dyn StarkNetLightClient>,
    /// Sync status.
    pub sync_status: SyncStatus,
}

impl BeerusLightClient {
    /// Create a new Beerus Light Client service.
    pub fn new(
        config: Config,
        ethereum_lightclient: Box<dyn EthereumLightClient>,
        starknet_lightclient: Box<dyn StarkNetLightClient>,
    ) -> Self {
        Self {
            config,
            ethereum_lightclient,
            starknet_lightclient,
            sync_status: SyncStatus::NotSynced,
        }
    }

    /// Start Beerus light client and synchronize with Ethereum and StarkNet.
    pub async fn start(&mut self) -> Result<()> {
        if let SyncStatus::NotSynced = self.sync_status {
            // Start the Ethereum light client.
            self.ethereum_lightclient.start().await?;
            // Start the StarkNet light client.
            self.starknet_lightclient.start().await?;
            self.sync_status = SyncStatus::Synced;
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

        let data = vec![149, 136, 236, 162];

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

    /// Get the StarkNet last proven block number.
    /// This function is used to get the last proven block number of the StarkNet network.
    ///
    /// # Returns
    /// `Ok(U256)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_last_proven_block(&self) -> Result<U256> {
        // Get the StarkNet core contract address.
        let starknet_core_contract_address = &self.config.starknet_core_contract_address;

        let data = vec![53, 190, 250, 93];

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

    /// Get the storage at a given address/key.
    /// This function is used to get the storage at a given address and key.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The StarkNet contract address.
    /// * `storage_key` - The storage key.
    ///
    /// # Returns
    ///
    /// `Ok(FieldElement)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_get_storage_at(
        &self,
        contract_address: FieldElement,
        storage_key: FieldElement,
    ) -> Result<FieldElement> {
        let last_block = self.starknet_last_proven_block().await?.as_u64();
        self.starknet_lightclient
            .get_storage_at(contract_address, storage_key, last_block)
            .await
    }

    /// Call starknet contract view.
    /// This function is used to call a view function of a StarkNet contract.
    /// WARNING: This function is untrusted as there's no access list on StarkNet (yet @Avihu).
    ///
    /// # Arguments
    /// * `contract_address` - The StarkNet contract address.
    /// * `entry_point_selector` - The entry point selector.
    /// * `calldata` - The calldata.
    ///
    /// # Returns
    ///
    /// `Ok(Vec<FieldElement>)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_call_contract(
        &self,
        contract_address: FieldElement,
        entry_point_selector: FieldElement,
        calldata: Vec<FieldElement>,
    ) -> Result<Vec<FieldElement>> {
        let opts = FunctionCall {
            contract_address,
            entry_point_selector,
            calldata,
        };

        let last_block = self.starknet_last_proven_block().await?.as_u64();
        // Call the StarkNet light client.
        self.starknet_lightclient.call(opts, last_block).await
    }
}
