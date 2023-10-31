use std::env;

use beerus_core::client::BeerusClient;
use beerus_core::config::Config;
use eyre::Result;
use starknet::providers::Provider;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[async_std::main]
async fn main() -> Result<()> {
    // logging
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Configuring beerus via env
    // Set the network to mainnet
    env::set_var("NETWORK", "MAINNET");
    // Set the ethereum execution rpc url. Put your key instead of <YOUR_API_KEY>
    env::set_var("ETH_EXECUTION_RPC", "https://eth-mainnet.g.alchemy.com/v2/<YOUR_API_KEY>");
    // Set the Starknet rpc url. Put your key instead of <YOUR_API_KEY>
    env::set_var("STARKNET_RPC", "https://starknet-mainnet.g.alchemy.com/v2/<YOUR_API_KEY>");

    // Initialize beerus
    let config = Config::from_env();
    let mut beerus = BeerusClient::new(config).await;
    beerus.start().await?;

    // getting starknet block number
    let current_starknet_block = beerus.starknet_block_number().await;

    println!("starknet block {:?}", current_starknet_block);
    Ok(())
}
