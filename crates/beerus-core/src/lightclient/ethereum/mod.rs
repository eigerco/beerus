pub mod helios_lightclient;

use crate::stdlib::boxed::Box;

use crate::stdlib::vec::Vec;

use async_trait::async_trait;
use core::u8;
use ethers::types::{
    Address, Filter, Log, SyncingStatus, Transaction, TransactionReceipt, H256, U256,
};
use eyre::Result;
use helios::types::{BlockTag, CallOpts, ExecutionBlock};
#[cfg(feature = "std")]
use mockall::automock;

/// Ethereum light client trait.
/// This trait is used to abstract the Ethereum light client implementation.
// TODO: For now there is a dependency on Helios types, we should abstract this away eventually.
// TODO: Maybe we can let the possibility to get access to the underlying light client anyway.

// #[cfg(feature="std")]
#[cfg_attr(feature = "std", automock, async_trait)]
#[cfg_attr(not(feature = "std"), async_trait(?Send))]
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
    async fn call(&self, opts: &CallOpts, block: BlockTag) -> Result<Vec<u8>>;

    /// Send Raw Transaction
    /// This function should be called after `start`.
    /// # Arguments
    /// * `bytes` - Transaction Bytes.
    /// # Returns
    /// The balance of the account.
    /// # Errors
    /// If the call fails.
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
    async fn get_nonce(&self, address: &Address, block: BlockTag) -> Result<u64>;

    /// Get the current block number.
    /// This function should be called after `start`.
    /// # Returns
    /// The current block number.
    /// # Errors
    /// If the call fails.
    async fn get_block_number(&self) -> Result<u64>;

    /// Get the chain ID.
    /// This function should be called after `start`.
    /// # Returns
    /// The chain ID.
    /// # Errors
    /// Cannot fail.
    async fn get_chain_id(&self) -> Result<u64>;

    /// Get the code of a given address.
    /// This function should be called after `start`.
    /// # Returns
    /// The code of the contract.
    /// # Errors
    /// If the call fails.
    async fn get_code(&self, address: &Address, block: BlockTag) -> Result<Vec<u8>>;

    /// Get the txs counts of an Ethereum address from a given block.
    /// This function should be called after `start`.
    /// # Returns
    /// The transaction count of a given address from a given block.
    /// # Errors
    /// If the call fails.
    async fn get_transaction_count(&self, address: &Address, block: BlockTag) -> Result<u64>;

    /// Get the txs counts of a given block number.
    /// This function should be called after `start`.
    /// # Returns
    /// The code of the contract.
    /// # Errors
    /// If the call fails.
    async fn get_block_transaction_count_by_number(&self, block: BlockTag) -> Result<u64>;

    /// Get the txs counts of a given block hash.
    /// This function should be called after `start`.
    /// # Returns
    /// The code of the contract.
    /// # Errors
    /// If the call fails.
    async fn get_block_transaction_count_by_hash(&self, hash: &[u8]) -> Result<u64>;

    /// Get transaction in a block at certain index.
    /// This function should be called after `start`.
    /// # Returns
    /// Transaction at that index.
    /// # Errors
    /// If the call fails.
    async fn get_transaction_by_block_hash_and_index(
        &self,
        hash: &[u8],
        index: usize,
    ) -> Result<Option<Transaction>>;

    /// Get the syncing status of the client.
    /// This function should be called after `start`.
    /// # Returns
    /// The SyncingStatus
    /// # Errors
    /// If the call fails.
    async fn syncing(&self) -> Result<SyncingStatus>;

    /// Get the coinbase address of the client.
    /// This function should be called after `start`.
    /// # Returns
    /// The coinbase address
    /// # Errors
    /// If the call fails.
    async fn coinbase(&self) -> Result<Address>;

    /// Get the tx_receipt of a certain tx
    /// This function should be called after `start`.
    /// # Returns
    /// Transaction Receipt.
    /// # Errors
    /// If the call fails.
    async fn get_transaction_receipt(&self, tx_hash: &H256) -> Result<Option<TransactionReceipt>>;

    /// Get the value at a certain storage position.
    /// This function should be called after `start`.
    /// # Returns
    /// Value at this storage position
    /// # Errors
    /// If the call fails.
    async fn get_storage_at(&self, address: &Address, slot: H256, block: BlockTag) -> Result<U256>;

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
    async fn get_gas_price(&self) -> Result<U256>;

    /// Generates and returns an estimate of how much gas is necessary to allow the transaction to complete.
    /// # Arguments
    /// * `opts` - Call options.
    /// # Returns
    /// The gas estimate.
    /// # Errors
    /// If the call fails.
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

    /// Get logs (blockchain events), based on the given filter.
    /// # Arguments
    /// * `from_block` - Either the hex value of a block number OR block tags.
    /// * `to_block` - Either the hex value of a block number OR block tags (e.g. 'latest').
    /// * `address` - Address from which logs come from. (e.g. 'latest').
    /// * `topics` - Array of 32 Bytes DATA topics. Topics are order-dependent. Each topic can also be an array of DATA with "or" options.
    /// * `block_hash` - Equivalent to using from_block = to_block. If provided, neither to_block or from_block are allowed.
    /// # Returns
    /// Vector of logs, matching the given filter params.
    /// # Errors
    /// If the call fails, or if there are more than 5 logs.
    async fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>>;

    // async fn get_logs(
    //     &self,
    //     from_block: &Option<String>,
    //     to_block: &Option<String>,
    //     address: &Option<String>,
    //     topics: &Option<Vec<String>>,
    //     block_hash: &Option<String>,
    // ) -> Result<Vec<Log>>;

    async fn starknet_last_proven_block(&self) -> Result<U256>;
    async fn starknet_state_root(&self) -> Result<U256>;
}
