use std::sync::Arc;

use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios::HeliosLightClient,
        starknet::StarkNetLightClient,
    },
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
    // Create a new Ethereum light client.
    let mut ethereum_lightclient = HeliosLightClient::new(&config).unwrap();
    // Create a new StarkNet light client.
    let starknet_lightclient = StarkNetLightClient::new(&config).unwrap();
    // Create a new Beerus light client.
    let mut beerus =
        BeerusLightClient::new(&config, &mut ethereum_lightclient, starknet_lightclient).unwrap();
    // Start the Beerus light client.
    info!("starting the Beerus light client...");
    beerus.start().await.unwrap();
    info!("Beerus light client started and synced.");
    let beerus = Arc::new(beerus);
    // Create a new Ethereum API handler.
    let ethereum_api = EthereumAPI::new(beerus.clone());
    // Create a new StarkNet API handler.
    let starknet_api = StarkNetAPI::new(beerus.clone());

    // Create the Rocket instance.
    rocket::build()
        //.manage(ethereum_api)
        //.manage(starknet_api)
        .mount(
            "/",
            routes![
                index,
                ethereum::endpoints::query_balance,
                starknet::endpoints::query_starknet_state_root
            ],
        )
}
