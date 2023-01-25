use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use beerus_rest_api::build_rocket_server;
use rocket::{Build, Rocket};
#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> Rocket<Build> {
    env_logger::init();

    info!("starting Beerus Rest API...");
    // Create config.
    let config = Config::default();
    let config_clone = config.clone();

    // Create a new Ethereum light client.
    let ethereum_lightclient = HeliosLightClient::new(config.clone()).await.unwrap();
    let ethereum_lightclient_clone = HeliosLightClient::new(config.clone()).await.unwrap();

    // Create a new StarkNet light client.
    let starknet_lightclient = StarkNetLightClientImpl::new(&config).unwrap();
    let starknet_lightclient_clone = StarkNetLightClientImpl::new(&config).unwrap();

    // Create a new Beerus light client.
    let mut beerus = BeerusLightClient::new(
        config,
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );
    info!("starting the Beerus light client...");
    beerus
        .start(
            config_clone,
            Box::new(ethereum_lightclient_clone),
            Box::new(starknet_lightclient_clone),
        )
        .await
        .unwrap();
    info!("Beerus light client started and synced.");

    build_rocket_server(beerus).await
}
