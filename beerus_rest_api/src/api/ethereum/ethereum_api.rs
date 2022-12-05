use beerus_core::lightclient::{
    beerus::BeerusLightClient, ethereum::ethereum::EthereumLightClient,
};
use log::debug;
use std::{str::FromStr, sync::Arc};

use ethers::{types::Address, utils};
use eyre::Result;
use helios::types::BlockTag;

use super::resp::QueryBalanceResponse;
/// Ethereum API endpoints handler.
pub struct EthereumAPI<'a> {
    /// The Beerus light client.
    beerus: Arc<BeerusLightClient<'a>>,
}

impl<'a> EthereumAPI<'a> {
    /// Create a new Ethereum API handler.
    pub fn new(beerus: Arc<BeerusLightClient<'a>>) -> Self {
        Self {
            beerus: beerus.clone(),
        }
    }

    /// Query the balance of an Ethereum address.
    /// # Arguments
    /// * `address` - The Ethereum address.
    /// # Returns
    /// `Ok(query_balance_response)` - The query balance response.
    /// `Err(error)` - An error occurred.
    /// # Errors
    /// If the Ethereum address is invalid or the block tag is invalid.
    /// # Examples
    pub async fn query_balance(&self, address: &str) -> Result<QueryBalanceResponse> {
        debug!("Querying balance of address: {}", address);

        // Parse the Ethereum address.
        let addr = Address::from_str(&address)?;
        let beerus = self.beerus.clone();
        // TODO: Make the block tag configurable.
        let block = BlockTag::Latest;
        // Query the balance of the Ethereum address.
        let balance = beerus
            .ethereum_lightclient
            .get_balance(&addr, block)
            .await?;
        // Format the balance in Ether.
        let balance_in_eth = utils::format_units(balance, "ether")?;
        Ok(QueryBalanceResponse {
            address: address.to_string(),
            balance: balance_in_eth,
            unit: "ETH".to_string(),
        })
    }
}

unsafe impl Send for EthereumAPI<'_> {}
unsafe impl Sync for EthereumAPI<'_> {}
