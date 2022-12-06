use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use beerus_rest_api::api::{ethereum, starknet};
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
            starknet::endpoints::query_starknet_state_root,
            starknet::endpoints::query_starknet_contract_view
        ],
    )
}
