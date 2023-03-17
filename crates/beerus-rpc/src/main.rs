use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use beerus_rpc::run_server;
use env_logger::Env;
use log::{error, info};
use std::process::exit;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::from_env();

    info!("creating ethereum(helios) lightclient...");
    let ethereum_lightclient = match HeliosLightClient::new(config.clone()).await {
        Ok(ethereum_lightclient) => ethereum_lightclient,
        Err(err) => {
            error! {"{}", err};
            exit(1);
        }
    };

    info!("creating starknet lightclient...");
    let starknet_lightclient = match StarkNetLightClientImpl::new(&config) {
        Ok(starknet_lightclient) => starknet_lightclient,
        Err(err) => {
            error! {"{}", err};
            exit(1);
        }
    };

    info!("creating beerus lightclient");
    let mut beerus = BeerusLightClient::new(
        config,
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );

    info!("starting the Beerus light client...");
    if let Err(err) = beerus.start().await {
        error!("{}", err);
        exit(1);
    };

    info!("starting beerus rpc server...");
    match run_server(beerus).await {
        Ok((addr, server_handle)) => {
            info!("===================================================");
            info!("Beerus JSON-RPC Server started: http://{addr}");
            info!("===================================================");

            server_handle.stopped().await;
        }
        Err(err) => {
            error! {"{}", err};
            exit(1);
        }
    };
}
