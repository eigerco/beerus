use crate::{config::Config, ethers_helper};
use ethers::{abi::Abi, types::U256};

use eyre::Result;
use helios::types::BlockTag;
use helios::types::CallOpts;
use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::models::FunctionCall;

use super::{ethereum::EthereumLightClient, starknet::StarkNetLightClient};

/// Enum representing the different synchrnization status of the light client.
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
    ) -> Result<Self> {
        Ok(Self {
            config,
            ethereum_lightclient,
            starknet_lightclient,
            sync_status: SyncStatus::NotSynced,
        })
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
        // Call the StarkNet light client.
        let result = self.starknet_lightclient.call(opts).await?;

        Ok(result)
    }
}
