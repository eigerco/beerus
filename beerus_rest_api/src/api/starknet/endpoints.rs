use crate::api::starknet::{resp::QueryStateRootResponse, starknet_api::StarkNetAPI};

use rocket::{get, State};

use crate::api::ApiResponse;

#[get("/starknet/state/root")]
pub async fn query_starknet_state_root(
    starknet_api: &State<StarkNetAPI>,
) -> ApiResponse<QueryStateRootResponse> {
    ApiResponse::from_result(starknet_api.query_state_root().await)
}
