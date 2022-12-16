use async_trait::async_trait;
use ethers::types::{Address, H256, Transaction};
use eyre::Result;
use helios::client::{Client, ClientBuilder, FileDB};
use helios::types::{BlockTag, CallOpts};
use std::primitive::u64;

use crate::config::Config;

use super::EthereumLightClient;

/// Helios implementation of `EthereumLightClient`.
pub struct HeliosLightClient {
    /// The wrapped Helios client.
    pub helios_light_client: Client<FileDB>,
}

/// Implementation of `EthereumLightClient` for Helios.
#[async_trait]
impl EthereumLightClient for HeliosLightClient {
    async fn start(&mut self) -> eyre::Result<()> {
        // Start the Helios light client.
        self.helios_light_client.start().await
    }

    async fn call(&self, opts: &CallOpts, block: BlockTag) -> eyre::Result<Vec<u8>> {
        // Wrap the Helios call.
        self.helios_light_client.call(opts, block).await
    }

    async fn get_balance(
        &self,
        address: &Address,
        block: BlockTag,
    ) -> eyre::Result<primitive_types::U256> {
        self.helios_light_client.get_balance(address, block).await
    }

    async fn get_nonce(&self, address: &Address, block: BlockTag) -> Result<u64> {
        self.helios_light_client.get_nonce(address, block).await
    }

    async fn get_block_number(&self) -> Result<u64> {
        self.helios_light_client.get_block_number().await
    }

    async fn chain_id(&self) -> u64 {
        self.helios_light_client.chain_id().await
    }
    async fn get_code(&self, address: &Address, block: BlockTag) -> Result<Vec<u8>> {
        self.helios_light_client.get_code(address, block).await
    }
    async fn get_block_transaction_count_by_number(&self, block: BlockTag) -> Result<u64> {
        self.helios_light_client
            .get_block_transaction_count_by_number(block)
            .await
    }
    async fn get_transaction_by_hash(&self, tx_hash: &H256) -> Result<Option<Transaction>>{
        self.helios_light_client
            .get_transaction_by_hash(tx_hash)
            .await
    }

}

/// HeliosLightClient non-trait functions.
impl HeliosLightClient {
    /// Create a new HeliosLightClient.
    pub async fn new(config: Config) -> eyre::Result<Self> {
        // Fetch the current checkpoint.
        let checkpoint_value = config.get_checkpoint().await.unwrap();

        // Build the Helios wrapped light client.
        let helios_light_client = ClientBuilder::new()
            .network(config.ethereum_network()?)
            .consensus_rpc(config.ethereum_consensus_rpc.as_str())
            .execution_rpc(config.ethereum_execution_rpc.as_str())
            .checkpoint(&checkpoint_value)
            .build()?;

        Ok(Self {
            helios_light_client,
        })
    }
}
