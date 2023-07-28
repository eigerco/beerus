use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use env_logger::Env;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
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

    let current_starknet_block = beerus.starknet_lightclient.block_number().await?;
    println!("{:?}", current_starknet_block);

    let current_ethereum_block = beerus
        .ethereum_lightclient
        .lock()
        .await
        .get_block_number()
        .await?;
    println!("{:?}", current_ethereum_block);
    Ok(())
}
