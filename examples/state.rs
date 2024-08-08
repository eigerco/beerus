use std::{env, path::PathBuf};

use beerus::client::Client;
use beerus::config::Config;
use eyre::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let api_key =
        env::var("ALCHEMY_API_KEY").context("ALCHEMY_API_KEY is missing")?;

    let config = Config {
        network: helios::config::networks::Network::MAINNET,
        eth_execution_rpc: format!(
            "https://eth-mainnet.g.alchemy.com/v2/{api_key}"
        ),
        starknet_rpc: format!(
            "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0.6/{api_key}"
        ),
        data_dir: PathBuf::from("tmp"),
        poll_secs: 300,
        rpc_addr: ([127, 0, 0, 1], 3030).into(),
    };

    let beerus = Client::new(&config).await?;
    beerus.start().await?;

    let state = beerus.get_state().await?;
    tracing::info!("{state:#?}");

    Ok(())
}
