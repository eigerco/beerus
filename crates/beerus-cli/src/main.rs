use beerus_cli::{model::Cli, runner};
use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use clap::Parser;
use env_logger::Env;
use log::{error, info};
use std::process::exit;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // TODO: we need to print CLI usage
    let cli = Cli::parse();

    let config = match &cli.config {
        Some(path) => Config::from_file(path),
        None => Config::from_env(),
    };


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

    info!("starting beerus lightclient...");
    if let Err(err) = beerus.start().await {
        error!("{}", err);
        exit(1);
    };

    info!("running cli...");
    match runner::run(beerus, cli).await {
        Ok(cmd_response) => {
            info!("successful command run...");
            println!("{cmd_response}");
        }
        Err(err) => {
            error! {"{}", err};
            exit(1);
        }
    };
}
