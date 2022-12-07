pub mod api;

use crate::api::{ethereum, starknet};
use beerus_core::lightclient::beerus::BeerusLightClient;
use rocket::{Build, Rocket};
#[macro_use]
extern crate rocket;

pub async fn build_rocket_server(beerus: BeerusLightClient) -> Rocket<Build> {
    // Create the Rocket instance.
    rocket::build().manage(beerus).mount(
        "/",
        routes![
            index,
            ethereum::endpoints::query_balance,
            starknet::endpoints::query_starknet_state_root,
            starknet::endpoints::query_starknet_contract_view,
            starknet::endpoints::query_starknet_get_storage_at,
        ],
    )
}

#[get("/")]
pub fn index() -> &'static str {
    "Hakai!"
}
