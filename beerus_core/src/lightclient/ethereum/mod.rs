pub mod helios_lightclient;

use async_trait::async_trait;
use ethers::types::Address;
use eyre::Result;
use helios::types::{BlockTag, CallOpts};
use primitive_types::U256;

/// Ethereum light client trait.
/// This trait is used to abstract the Ethereum light client implementation.
// TODO: For now there is a dependency on Helios types, we should abstract this away eventually.
// TODO: Maybe we can let the possibility to get access to the underlying light client anyway.
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
}
