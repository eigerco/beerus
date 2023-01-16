use super::{ethereum::EthereumLightClient, starknet::StarkNetLightClient};
use crate::{config::Config, ethers_helper};
use ethers::{
    abi::Abi,
    types::{H160, U256},
};
use eyre::Result;
use helios::types::BlockTag;
use helios::types::CallOpts;
use starknet::{core::types::FieldElement, providers::jsonrpc::models::FunctionCall};

/// Enum representing the different synchronization status of the light client.
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
    /// StarkNet core ABI.
    pub starknet_core_abi: Abi,
    /// StarkNet core contract address.
    pub starknet_core_contract_address: H160,
}

impl BeerusLightClient {
    /// Create a new Beerus Light Client service.
    pub fn new(
        config: Config,
        ethereum_lightclient: Box<dyn EthereumLightClient>,
        starknet_lightclient: Box<dyn StarkNetLightClient>,
    ) -> Self {
        let starknet_core_abi = include_str!("../resources/starknet_core_abi.json");
        // Deserialize the StarkNet core ABI.
        // For now we assume that the ABI is valid and that the deserialization will never fail.
        let starknet_core_abi: Abi = serde_json::from_str(starknet_core_abi).unwrap();
        let starknet_core_contract_address = config.starknet_core_contract_address;

        Self {
            config,
            ethereum_lightclient,
            starknet_lightclient,
            sync_status: SyncStatus::NotSynced,
            starknet_core_abi,
            starknet_core_contract_address,
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

    /// Return the current synchronization status.
    pub fn sync_status(&self) -> &SyncStatus {
        &self.sync_status
    }

    /// Get the StarkNet state root.
    pub async fn starknet_state_root(&self) -> Result<U256> {
        // Get the StarkNet core contract address.
        let starknet_core_contract_address = &self.config.starknet_core_contract_address;

        // Corresponds to the StarkNet core contract function `stateRoot`.
        // The function signature is `stateRoot() -> (uint256)`.
        // The function selector is `0x95d8ecA2`.
        let data = vec![0x95, 0x88, 0xec, 0xa2];

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

    /// Get the nonce at a given address.
    /// This function is used to get the nonce at a given address.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The StarkNet contract address.
    ///
    /// # Returns
    ///
    /// `Ok(FieldElement)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_get_nonce(&self, address: FieldElement) -> Result<FieldElement> {
        let last_block = self.starknet_last_proven_block().await?.as_u64();

        self.starknet_lightclient
            .get_nonce(last_block, address)
            .await
    }

    /// Return the timestamp at the time cancelL1ToL2Message was called with a message matching 'msg_hash'.
    /// The function returns 0 if cancelL1ToL2Message was never called.
    /// See https://github.com/starknet-io/starknet-addresses for the StarkNet core contract address on different networks.
    /// # Arguments
    /// * `msg_hash` - The message hash as bytes32.
    /// # Returns
    /// `Ok(U256)` if the operation was successful - The timestamp at the time cancelL1ToL2Message was called with a message matching 'msg_hash'.
    /// `Ok(U256::zero())` if the operation was successful - The function returns 0 if cancelL1ToL2Message was never called.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256> {
        // Convert the message hash to bytes32.
        let msg_hash_bytes32 = ethers_helper::u256_to_bytes32_type(msg_hash);
        // Encode the function data.
        let data = ethers_helper::encode_function_data(
            msg_hash_bytes32,
            self.starknet_core_abi.clone(),
            "l1ToL2MessageCancellations",
        )?;
        let data = data.to_vec();

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: self.starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let call_response = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;
        Ok(U256::from_big_endian(&call_response))
    }

    /// Return the msg_fee + 1 from the L1ToL2Message hash'. 0 if there is no matching msg_hash
    /// The function returns 0 if L1ToL2Message was never called.
    /// See https://github.com/starknet-io/starknet-addresses for the StarkNet core contract address on different networks.
    /// # Arguments
    /// * `msg_hash` - The message hash as bytes32.
    /// # Returns
    /// `Ok(U256)` if the operation was successful - The msg_fee + 1 from the L1ToL2Message hash'.
    /// `Ok(U256::zero())` if the operation was successful - The function returns 0 if there is no match on the message hash
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_l1_to_l2_messages(&self, msg_hash: ethers::types::U256) -> Result<U256> {
        // Convert the message hash to bytes32.
        let msg_hash_bytes32 = ethers_helper::u256_to_bytes32_type(msg_hash);
        // Encode the function data.
        let data = ethers_helper::encode_function_data(
            msg_hash_bytes32,
            self.starknet_core_abi.clone(),
            "l1ToL2Messages",
        )?;
        let data = data.to_vec();

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: self.starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let call_response = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;
        Ok(U256::from_big_endian(&call_response))
    }

    ///  Returns the msg_fee + 1 for the message with the given 'msgHash', or 0 if no message with such a hash is pending.
    /// The function returns 0 if L2ToL1Message was never called.
    /// See https://github.com/starknet-io/starknet-addresses for the StarkNet core contract address on different networks.
    /// # Arguments
    /// * `msg_hash` - The message hash as bytes32.
    /// # Returns
    /// `Ok(U256)` if the operation was successful - The msg_fee + 1 from the L2ToL1Message hash'.
    /// `Ok(U256::zero())` if the operation was successful - The function returns 0 if there is no matching message hash
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256> {
        // Convert the message hash to bytes32.
        let msg_hash_bytes32 = ethers_helper::u256_to_bytes32_type(msg_hash);
        // Encode the function data.
        let data = ethers_helper::encode_function_data(
            msg_hash_bytes32,
            self.starknet_core_abi.clone(),
            "l2ToL1Messages",
        )?;
        let data = data.to_vec();

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: self.starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let call_response = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;
        Ok(U256::from_big_endian(&call_response))
    }

    /// Return the nonce for the L1ToL2Message bridge.
    /// See https://github.com/starknet-io/starknet-addresses for the StarkNet core contract address on different networks.
    /// # Arguments
    /// # Returns
    /// `Ok(U256)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256> {
        // Encode the function data.
        let data = ethers_helper::encode_function_data(
            (),
            self.starknet_core_abi.clone(),
            "l1ToL2MessageNonce",
        )?;
        let data = data.to_vec();

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: self.starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let call_response = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;
        Ok(U256::from_big_endian(&call_response))
    }
}
