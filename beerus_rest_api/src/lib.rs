pub mod api;

use crate::api::{ethereum, starknet};
use beerus_core::lightclient::beerus::BeerusLightClient;
use rocket::{Build, Rocket};
use rocket_okapi::{openapi, openapi_get_routes};

#[macro_use]
extern crate rocket;

pub async fn build_rocket_server(beerus: BeerusLightClient) -> Rocket<Build> {
    // Create the Rocket instance.
    rocket::build().manage(beerus).mount(
        "/",
        openapi_get_routes![
            index,
            ethereum::endpoints::query_balance,
            ethereum::endpoints::query_nonce,
            ethereum::endpoints::query_block_number,
            starknet::endpoints::query_starknet_state_root,
            starknet::endpoints::query_starknet_contract_view,
            starknet::endpoints::query_starknet_get_storage_at,
        ],
    )
}

#[openapi]
#[get("/")]
pub fn index() -> &'static str {
    "Hakai!"
}
