use std::env;

use beerus_core::client::BeerusClient;
use beerus_core::config::Config;
use eyre::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = env::var("ALCHEMY_API_KEY").context("ALCHEMY_API_KEY is missing")?;

    let config = Config {
        network: helios::config::networks::Network::MAINNET,
        eth_execution_rpc: format!("https://eth-mainnet.g.alchemy.com/v2/{api_key}"),
        starknet_rpc: format!("https://starknet-mainnet.g.alchemy.com/v2/{api_key}"),
        ..Default::default()
    };
    let mut beerus = BeerusClient::new(config).await?;
    beerus.start().await?;

    let current_starknet_block = beerus.state_root().await?;
    tracing::info!("current starknet block: {current_starknet_block:X}");
    Ok(())
}
