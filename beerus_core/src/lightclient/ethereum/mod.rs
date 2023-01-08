pub mod helios_lightclient;

use async_trait::async_trait;
use ethers::types::{Address, Transaction, H256, U256};
use eyre::Result;
use helios::types::{BlockTag, CallOpts, ExecutionBlock};
use mockall::automock;
use std::u8;

/// Ethereum light client trait.
/// This trait is used to abstract the Ethereum light client implementation.
// TODO: For now there is a dependency on Helios types, we should abstract this away eventually.
// TODO: Maybe we can let the possibility to get access to the underlying light client anyway.
#[automock]
#[async_trait]
pub trait EthereumLightClient: Send + Sync {
    /// Start and synchronize the Ethereum light client.
    /// This function should be called before any other function.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn start(&mut self) -> Result<()>;
    /// Call a contract function.
    /// This function should be called after `start`.
    /// # Arguments
    /// * `opts` - Call options.
    /// * `block` - Block tag.
    /// # Returns
    /// The result of the call.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn call(&self, opts: &CallOpts, block: BlockTag) -> Result<Vec<u8>>;

    /// Send Raw Transaction
    /// This function should be called after `start`.
    /// # Arguments
    /// * `bytes` - Transaction Bytes.
    /// # Returns
    /// The balance of the account.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn send_raw_transaction(&self, bytes: &[u8]) -> Result<H256>;

    /// Get the balance of an account.
    /// This function should be called after `start`.
    /// # Arguments
    /// * `address` - Address of the account.
    /// * `block` - Block tag.
    /// # Returns
    /// The balance of the account.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn get_balance(&self, address: &Address, block: BlockTag) -> Result<U256>;

    /// Get the Nonce of an account.
    /// This function should be called after `start`.
    /// # Arguments
    /// * `address` - Address of the account.
    /// * `block` - Block tag.
    /// # Returns
    /// The balance of the account.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn get_nonce(&self, address: &Address, block: BlockTag) -> Result<u64>;

    /// Get the current block number.
    /// This function should be called after `start`.
    /// # Returns
    /// The current block number.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn get_block_number(&self) -> Result<u64>;

    /// Get the chain ID.
    /// This function should be called after `start`.
    /// # Returns
    /// The chain ID.
    /// # Errors
    /// Cannot fail.
    async fn chain_id(&self) -> u64;

    /// Get the code of a given address.
    /// This function should be called after `start`.
    /// # Returns
    /// The code of the contract.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn get_code(&self, address: &Address, block: BlockTag) -> Result<Vec<u8>>;

    /// Get the txs counts of a given block number.
    /// This function should be called after `start`.
    /// # Returns
    /// The code of the contract.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn get_block_transaction_count_by_number(&self, block: BlockTag) -> Result<u64>;

    /// Get the txs counts of a given block hash.
    /// This function should be called after `start`.
    /// # Returns
    /// The code of the contract.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn get_block_transaction_count_by_hash(&self, hash: &[u8]) -> Result<u64>;

    /// Get the tx data of a given tx hash.
    /// This function should be called after `start`.
    /// # Returns
    /// The code of the contract.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn get_transaction_by_hash(&self, tx_hash: &H256) -> Result<Option<Transaction>>;

    /// Get gas price.
    /// This function should be called after `start`.
    /// # Returns
    /// The gas price from the Ethereum network.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn get_gas_price(&self) -> Result<U256>;

    /// Generates and returns an estimate of how much gas is necessary to allow the transaction to complete.
    /// # Arguments
    /// * `opts` - Call options.
    /// # Returns
    /// The gas estimate.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn estimate_gas(&self, opts: &CallOpts) -> Result<u64>;

    /// Get information about a block by block hash.
    /// This function should be called after `start`.
    /// # Arguments
    /// * `hash` - block hash
    /// * `full_tx` - If true it returns the full transaction objects, if false only the hashes of the transactions.
    /// # Returns
    /// A block object, or null when no block was found.
    /// # Errors
    /// If the call fails.
    async fn get_block_by_hash(&self, hash: &[u8], full_tx: bool)
        -> Result<Option<ExecutionBlock>>;

    /// Get max priority fee_per_gas.
    /// This function should be called after `start`.
    /// # Returns
    /// The gas price from the Ethereum network.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn get_priority_fee(&self) -> Result<U256>;

    /// Get information about a block by block number.
    /// This function should be called after `start`.
    /// # Arguments
    /// * `block` - integer of a block number, or the string "earliest", "latest" or "pending"
    /// * `full_tx` - If true it returns the full transaction objects, if false only the hashes of the transactions.
    /// # Returns
    /// A block object, or null when no block was found.
    /// # Errors
    /// If the call fails.
    async fn get_block_by_number(
        &self,
        block: BlockTag,
        full_tx: bool,
    ) -> Result<Option<ExecutionBlock>>;
}
