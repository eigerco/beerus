use async_trait::async_trait;

// #[cfg(not(feature = "std"))]
// #[allow(unused_imports)]
// #[macro_use]
// extern crate alloc;

use crate::stdlib::boxed::Box;

use crate::stdlib::vec::Vec;

use crate::stdlib::primitive::u64;

use ethers::types::{
    Address, Filter, Log, SyncingStatus, Transaction, TransactionReceipt, H256, U256,
};
use eyre::Result;

use crate::config::Config;

use helios::types::{BlockTag, CallOpts, ExecutionBlock};

use super::EthereumLightClient;

#[cfg(not(feature = "std"))]
use helios::client::{Client, ClientBuilder, ConfigDB};
#[cfg(feature = "std")]
use helios::client::{Client, ClientBuilder, FileDB};

/// Helios implementation of `EthereumLightClient`.
pub struct HeliosLightClient {
    /// The wrapped Helios client.
    #[cfg(feature = "std")]
    pub helios_light_client: Client<FileDB>,

    #[cfg(not(feature = "std"))]
    pub helios_light_client: Client<ConfigDB>,

    pub starknet_core_contract_address: Address,
}

/// Implementation of `EthereumLightClient` for Helios.
#[cfg_attr(feature = "std", async_trait)]
#[cfg_attr(not(feature = "std"), async_trait(?Send))]
impl EthereumLightClient for HeliosLightClient {
    async fn start(&mut self) -> eyre::Result<()> {
        // Start the Helios light client.
        self.helios_light_client.start().await
    }

    async fn call(&self, opts: &CallOpts, block: BlockTag) -> eyre::Result<Vec<u8>> {
        // Wrap the Helios call.
        self.helios_light_client.call(opts, block).await
    }

    async fn send_raw_transaction(&self, bytes: &[u8]) -> eyre::Result<ethers::types::H256> {
        self.helios_light_client.send_raw_transaction(bytes).await
    }

    async fn get_balance(&self, address: &Address, block: BlockTag) -> eyre::Result<U256> {
        self.helios_light_client.get_balance(address, block).await
    }

    async fn get_nonce(&self, address: &Address, block: BlockTag) -> Result<u64> {
        self.helios_light_client.get_nonce(address, block).await
    }

    async fn get_block_number(&self) -> Result<u64> {
        self.helios_light_client.get_block_number().await
    }

    async fn get_chain_id(&self) -> Result<u64> {
        Ok(self.helios_light_client.chain_id().await)
    }

    async fn get_code(&self, address: &Address, block: BlockTag) -> Result<Vec<u8>> {
        self.helios_light_client.get_code(address, block).await
    }

    async fn get_transaction_count(&self, address: &Address, block: BlockTag) -> Result<u64> {
        // TODO: Rename after it has been renamed https://github.com/a16z/helios/pull/166#issuecomment-1379587761
        self.helios_light_client.get_nonce(address, block).await
    }

    async fn get_block_transaction_count_by_number(&self, block: BlockTag) -> Result<u64> {
        self.helios_light_client
            .get_block_transaction_count_by_number(block)
            .await
    }

    async fn get_transaction_by_block_hash_and_index(
        &self,
        hash: &[u8],
        index: usize,
    ) -> Result<Option<Transaction>> {
        let hash = hash.to_vec();
        self.helios_light_client
            .get_transaction_by_block_hash_and_index(&hash, index)
            .await
    }

    async fn get_transaction_receipt(&self, hash: &H256) -> Result<Option<TransactionReceipt>> {
        self.helios_light_client.get_transaction_receipt(hash).await
    }

    async fn coinbase(&self) -> Result<Address> {
        self.helios_light_client.get_coinbase().await
    }

    async fn syncing(&self) -> Result<SyncingStatus> {
        self.helios_light_client.syncing().await
    }

    async fn get_block_transaction_count_by_hash(&self, hash: &[u8]) -> Result<u64> {
        let hash = hash.to_vec();
        self.helios_light_client
            .get_block_transaction_count_by_hash(&hash)
            .await
    }

    async fn get_transaction_by_hash(&self, tx_hash: &H256) -> Result<Option<Transaction>> {
        self.helios_light_client
            .get_transaction_by_hash(tx_hash)
            .await
    }
    async fn get_gas_price(&self) -> Result<U256> {
        self.helios_light_client.get_gas_price().await
    }

    async fn get_storage_at(&self, address: &Address, slot: H256, block: BlockTag) -> Result<U256> {
        self.helios_light_client
            .get_storage_at(address, slot, block)
            .await
    }

    async fn estimate_gas(&self, opts: &CallOpts) -> Result<u64> {
        self.helios_light_client.estimate_gas(opts).await
    }
    async fn get_block_by_hash(
        &self,
        hash: &[u8],
        full_tx: bool,
    ) -> eyre::Result<Option<ExecutionBlock>> {
        let hash: Vec<u8> = Vec::from(hash);
        self.helios_light_client
            .get_block_by_hash(&hash, full_tx)
            .await
    }
    async fn get_priority_fee(&self) -> Result<U256> {
        self.helios_light_client.get_priority_fee().await
    }

    async fn get_block_by_number(
        &self,
        block: BlockTag,
        full_tx: bool,
    ) -> eyre::Result<Option<ExecutionBlock>> {
        self.helios_light_client
            .get_block_by_number(block, full_tx)
            .await
    }

    async fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>> {
        self.helios_light_client.get_logs(filter).await
    }

    /// Get the StarkNet state root.
    async fn starknet_state_root(&self) -> Result<U256> {
        // Corresponds to the StarkNet core contract function `stateRoot`.
        // The function signature is `stateRoot() -> (uint256)`.
        // The function selector is `0x95d8ecA2`.
        let data = vec![0x95, 0x88, 0xec, 0xa2];

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: Some(self.starknet_core_contract_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let starknet_root = self.call(&call_opts, BlockTag::Latest).await?;

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
    async fn starknet_last_proven_block(&self) -> Result<U256> {
        let data = vec![53, 190, 250, 93];

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: Some(self.starknet_core_contract_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let starknet_root = self.call(&call_opts, BlockTag::Latest).await?;

        // Convert the response bytes to a U256.
        let starknet_root = U256::from_big_endian(&starknet_root);

        Ok(starknet_root)
    }
}

/// HeliosLightClient non-trait functions.
impl HeliosLightClient {
    /// Create a new HeliosLightClient.
    pub async fn new(config: Config) -> eyre::Result<Self> {
        // Build the Helios wrapped light client.
        #[cfg(feature = "std")]
        let helios_light_client: Client<FileDB> = ClientBuilder::new()
            .network(config.ethereum_network()?)
            .consensus_rpc(config.ethereum_consensus_rpc.as_str())
            .execution_rpc(config.ethereum_execution_rpc.as_str())
            .load_external_fallback()
            .data_dir(config.data_dir)
            .build()?;

        #[cfg(not(feature = "std"))]
        let helios_light_client: Client<ConfigDB> = ClientBuilder::new()
            .network(config.ethereum_network()?)
            .consensus_rpc(config.ethereum_consensus_rpc.as_str())
            .execution_rpc(config.ethereum_execution_rpc.as_str())
            .load_external_fallback()
            .build()?;

        Ok(Self {
            helios_light_client,
            starknet_core_contract_address: config.starknet_core_contract_address,
        })
    }

    #[cfg(feature = "std")]
    pub async fn new_rpc(config: Config) -> eyre::Result<Self> {
        // Build the Helios wrapped light client.
        let helios_light_client: Client<FileDB> = ClientBuilder::new()
            .network(config.ethereum_network()?)
            .consensus_rpc(config.ethereum_consensus_rpc.as_str())
            .execution_rpc(config.ethereum_execution_rpc.as_str())
            .load_external_fallback()
            .data_dir(config.data_dir)
            .rpc_port(config.helios_rpc_address.unwrap())
            .build()?;

        Ok(Self {
            helios_light_client,
            starknet_core_contract_address: config.starknet_core_contract_address,
        })
    }
}
