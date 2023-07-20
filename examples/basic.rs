use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use env_logger::Env;
use eyre::Result;
use log::{error, info};
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // setting BEERUS_CONFIG env loads config from provided file
    env::set_var(
        "BEERUS_CONFIG",
        format!(
            "{}/examples/mainnet.toml",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        ),
    );
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = Config::from_env();

    let ethereum_lightclient = HeliosLightClient::new(config.clone()).await?;
    let starknet_lightclient = StarkNetLightClientImpl::new(&config)?;
    let mut beerus = BeerusLightClient::new(
        config.clone(),
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );
    beerus.start().await?;
    let res = beerus
        .ethereum_lightclient
        .lock()
        .await
        .get_block_number()
        .await?;
    info!("{:?}", res);
    Ok(())
}
