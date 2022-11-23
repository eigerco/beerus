use beerus_core::{
    config::Config,
    lightclient::beerus::{Beerus, BeerusLightClient},
};
use log::debug;
use std::{str::FromStr, sync::Arc};

use ethers::{types::Address, utils};
use eyre::Result;
use helios::types::BlockTag;

use super::resp::QueryBalanceResponse;
/// Ethereum API endpoints handler.
pub struct EthereumAPI {
    /// The Beerus light client.
    beerus: Arc<BeerusLightClient>,
}

impl EthereumAPI {
    /// Create a new Ethereum API handler.
    pub async fn new(config: &Config) -> Result<Self> {
        // Create a new Beerus light client.
        let mut beerus = BeerusLightClient::new(config)?;
        // Start the Beerus light client.
        beerus.start().await?;
        Ok(Self {
            beerus: Arc::from(beerus),
        })
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
