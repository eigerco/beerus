use crate::api::ethereum::resp::QueryBalanceResponse;

use rocket::{get, State};

use super::ethereum_api::EthereumAPI;
use crate::api::ApiResponse;

#[get("/ethereum/balance/<address>")]
pub async fn query_balance(
    address: &str,
    ethereum_api: &State<EthereumAPI>,
) -> ApiResponse<QueryBalanceResponse> {
    ApiResponse::from_result(ethereum_api.query_balance(address).await)
}
