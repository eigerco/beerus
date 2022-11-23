use crate::api::ethereum::resp::QueryBalanceResponse;
use beerus_core::config::Config;

use eyre::Result;
use rocket::get;

use super::ethereum_api::EthereumAPI;
use crate::api::ApiResponse;

#[get("/ethereum/balance/<address>")]
pub async fn query_balance(address: &str) -> ApiResponse<QueryBalanceResponse> {
    ApiResponse::from_result(query_balance_inner(address).await)
}

/// Query the balance of an Ethereum address.
/// # Arguments
/// * `address` - The Ethereum address.
/// # Returns
/// `Ok(query_balance_response)` - The query balance response.
/// `Err(error)` - An error occurred.
/// # Errors
/// If the Ethereum address is invalid.
pub async fn query_balance_inner(address: &str) -> Result<QueryBalanceResponse> {
    // Create config.
    let config = Config::new_from_env().unwrap();
    // Create a new Ethereum API handler.
    let ethereum_api = EthereumAPI::new(&config).await.unwrap();
    // Query the balance of the Ethereum address.
    ethereum_api.query_balance(address).await
}
