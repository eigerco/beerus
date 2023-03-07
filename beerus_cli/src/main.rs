use std::{thread, time};

use beerus_cli::{model::Cli, runner};
use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use clap::Parser;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger.
    env_logger::init();
    // Parse the CLI arguments.
    let cli = Cli::parse();
    // Read the config from the environment.
    let config = Config::new_from_env()?;
    // Create a new Ethereum light client.
    let ethereum_lightclient = HeliosLightClient::new(config.clone()).await?;
    // Create a new StarkNet light client.
    let starknet_lightclient = StarkNetLightClientImpl::new(&config)?;

    // Create a new Beerus light client.
    let mut beerus = BeerusLightClient::new(
        config,
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );

    // Start the Beerus light client.
    log::info!("Starting Beerus light client...");
    beerus.start().await?;
    log::info!("Beerus light client started!");
    // Run the CLI command.
    log::info!("Before Running command");
    let command_response = runner::run(beerus, cli).await?;
    log::info!("After Command response");
    // Print the command response.
    // The handling of the command response is left to each `CommandResponse` implementation.
    log::info!("{command_response}");
    //Thread sleep to test Node/Payload storage
    //TODO: Remove once data/payload is stable
    thread::sleep(time::Duration::from_secs(200));

    Ok(())
}
