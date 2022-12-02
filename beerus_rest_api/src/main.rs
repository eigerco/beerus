use std::sync::Arc;

use beerus_core::{
    config::Config,
    lightclient::beerus::{Beerus, BeerusLightClient},
};
use beerus_rest_api::api::{
    ethereum::{self, ethereum_api::EthereumAPI},
    starknet::{self, starknet_api::StarkNetAPI},
};
use log::info;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hakai!"
}

#[launch]
async fn rocket() -> _ {
    env_logger::init();
    info!("starting Beerus Rest API...");
    // Create config.
    let config = Config::new_from_env().unwrap();
    // Create a new Beerus light client.
    let mut beerus = BeerusLightClient::new(config).unwrap();
    // Start the Beerus light client.
    info!("starting the Beerus light client...");
    beerus.start().await.unwrap();
    info!("Beerus light client started and synced.");
    let beerus = Arc::new(beerus);
    // Create a new Ethereum API handler.
    let ethereum_api = EthereumAPI::new(beerus.clone());
    // Create a new StarkNet API handler.
    let starknet_api = StarkNetAPI::new(beerus.clone());
    rocket::build()
        .manage(ethereum_api)
        .manage(starknet_api)
        .mount(
            "/",
            routes![
                index,
                ethereum::endpoints::query_balance,
                starknet::endpoints::query_starknet_state_root
            ],
        )
}
