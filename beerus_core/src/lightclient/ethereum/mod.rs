pub mod helios_lightclient;

use async_trait::async_trait;
use ethers::types::{Address, Transaction, H256};
use eyre::Result;
use helios::types::{BlockTag, CallOpts};
use mockall::automock;
use primitive_types::U256;

/// Ethereum light client trait.
/// This trait is used to abstract the Ethereum light client implementation.
// TODO: For now there is a dependency on Helios types, we should abstract this away eventually.
// TODO: Maybe we can let the possibility to get access to the underlying light client anyway.
#[automock]
#[async_trait]
pub trait EthereumLightClient: Send + Sync {
    /// Start and synchronise the Ethereum light client.
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

    /// Get the tx data of a given tx hash.
    /// This function should be called after `start`.
    /// # Returns
    /// The code of the contract.
    /// # Errors
    /// If the call fails.
    /// # TODO
    /// Add examples.
    async fn get_transaction_by_hash(&self, tx_hash: &H256) -> Result<Option<Transaction>>;
}
