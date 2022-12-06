use crate::api::ethereum::resp::QueryBalanceResponse;

use beerus_core::lightclient::beerus::BeerusLightClient;
use log::debug;
use rocket::{get, State};
use std::str::FromStr;

use ethers::{types::Address, utils};
use eyre::Result;
use helios::types::BlockTag;

use crate::api::ApiResponse;

#[get("/ethereum/balance/<address>")]
pub async fn query_balance(
    address: &str,
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryBalanceResponse> {
    ApiResponse::from_result(query_balance_inner(beerus, address).await)
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
pub async fn query_balance_inner(
    beerus: &State<BeerusLightClient>,
    address: &str,
) -> Result<QueryBalanceResponse> {
    debug!("Querying balance of address: {}", address);
    // Parse the Ethereum address.
    let addr = Address::from_str(address)?;
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
