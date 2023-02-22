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
    println!("Starting Beerus light client...");
    beerus.start().await?;
    println!("Beerus light client started!");
    // Run the CLI command.
    println!("Before Running command");
    let command_response = runner::run(beerus, cli).await?;
    println!("After Command response");
    // Print the command response.
    // The handling of the command response is left to each `CommandResponse` implementation.
    println!("{command_response}");
    //Thread sleep to test Node/Payload storage
    thread::sleep(time::Duration::from_secs(200));

    Ok(())
}
