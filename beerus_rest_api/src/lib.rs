pub mod api;

use crate::api::{ethereum, starknet};
use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use log::info;
use rocket::{Build, Rocket};
#[macro_use]
extern crate rocket;

pub async fn build_rocket_server() -> Rocket<Build> {
    env_logger::init();
    info!("starting Beerus Rest API...");
    // Create config.
    let config = Config::default();

    // Create a new Ethereum light client.
    let ethereum_lightclient = HeliosLightClient::new(config.clone()).unwrap();
    // Create a new StarkNet light client.
    let starknet_lightclient = StarkNetLightClientImpl::new(&config).unwrap();
    // Create a new Beerus light client.
    let mut beerus = BeerusLightClient::new(
        config,
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );

    // Start the Beerus light client.
    info!("starting the Beerus light client...");
    beerus.start().await.unwrap();
    info!("Beerus light client started and synced.");

    // Create the Rocket instance.
    rocket::build().manage(beerus).mount(
        "/",
        routes![
            index,
            ethereum::endpoints::query_balance,
            starknet::endpoints::query_starknet_state_root
        ],
    )
}

#[get("/")]
pub fn index() -> &'static str {
    "Hakai!"
}
