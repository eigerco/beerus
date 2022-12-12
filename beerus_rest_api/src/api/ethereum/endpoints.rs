use crate::api::ethereum::resp::{
    QueryBalanceResponse, QueryBlockNumberResponse, QueryNonceResponse,
};
use crate::api::ApiResponse;

use beerus_core::lightclient::beerus::BeerusLightClient;
use ethers::{types::Address, utils};
use eyre::Result;
use helios::types::BlockTag;
use log::debug;
use rocket::{get, State};
use rocket_okapi::openapi;
use std::str::FromStr;

#[openapi]
#[get("/ethereum/balance/<address>")]
pub async fn query_balance(
    address: &str,
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryBalanceResponse> {
    ApiResponse::from_result(query_balance_inner(beerus, address).await)
}

#[openapi]
#[get("/ethereum/nonce/<address>")]
pub async fn query_nonce(
    address: &str,
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryNonceResponse> {
    ApiResponse::from_result(query_nonce_inner(beerus, address).await)
}

#[openapi]
#[get("/ethereum/block_number")]
pub async fn query_block_number(
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryBlockNumberResponse> {
    ApiResponse::from_result(query_block_number_inner(beerus).await)
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

/// Query the balance of an Ethereum address.
/// # Arguments
/// * `address` - The Ethereum address.
/// # Returns
/// `Ok(query_balance_response)` - The query balance response.
/// `Err(error)` - An error occurred.
/// # Errors
/// If the Ethereum address is invalid or the block tag is invalid.
/// # Examples

pub async fn query_nonce_inner(
    beerus: &State<BeerusLightClient>,
    address: &str,
) -> Result<QueryNonceResponse> {
    debug!("Querying nonce of address: {}", address);
    let addr = Address::from_str(address)?;
    let block = BlockTag::Latest;
    let nonce = beerus.ethereum_lightclient.get_nonce(&addr, block).await?;

    Ok(QueryNonceResponse {
        address: address.to_string(),
        nonce,
    })
}

/// Query the block number of the Ethereum chain.
/// # Returns
/// `Ok(block_number)` - The block number.
/// `Err(error)` - An error occurred.
/// # Errors
/// If the block number query fails.
/// # Examples
pub async fn query_block_number_inner(
    beerus: &State<BeerusLightClient>,
) -> Result<QueryBlockNumberResponse> {
    debug!("Querying block number");
    let block_number = beerus.ethereum_lightclient.get_block_number().await?;
    Ok(QueryBlockNumberResponse { block_number })
}
