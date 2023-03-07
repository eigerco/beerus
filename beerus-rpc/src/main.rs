use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use beerus_rpc::run_server;
use dotenv::dotenv;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    // Create config.
    let config = Config::default();

    // Create a new Ethereum light client.
    let ethereum_lightclient = HeliosLightClient::new(config.clone()).await.unwrap();
    // Create a new StarkNet light client.
    let starknet_lightclient = StarkNetLightClientImpl::new(&config).unwrap();
    // Create a new Beerus light client.
    let mut beerus = BeerusLightClient::new(
        config,
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );
    log::info!("starting the Beerus light client...");
    beerus.start().await.unwrap();
    log::info!("Beerus light client started and synced.");

    let (addr, server_handle) = run_server(beerus).await.unwrap();
    let url = format!("http://{addr}");
    log::info!("Server started, listening on {url}");

    server_handle.stopped().await;

    Ok(())
}
