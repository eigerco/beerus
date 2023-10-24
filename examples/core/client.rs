use beerus_core::{config::Config, lightclient::beerus::BeerusLightClient};
use env_logger::Env;
use eyre::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Set helios client
    // Set the network to mainnet
    env::set_var("ETHEREUM_NETWORK", "mainnet");
    // Set the ethereum consensus rpc url
    env::set_var(
        "ETHEREUM_CONSENSUS_RPC_URL",
        "https://www.lightclientdata.org",
    );
    // Set the ethereum execution rpc url
    env::set_var(
        "ETHEREUM_EXECUTION_RPC_URL",
        "https://eth-mainnet.g.alchemy.com/v2/<YOUR_API_KEY>",
    );
    // Set the data dir
    env::set_var("DATA_DIR", "~/.beerus/tmp");
    // Set the checkpoint to the last known checkpoint.
    // Checkpoints can be found, for example, here https://sync.invis.tools/
    env::set_var(
        "ETHEREUM_CHECKPOINT",
        "0x419347336a423e0ad7ef3a1e8c0ca95f8b4f525122eea0178a11f1527ba38c0f",
    );

    // Set the Starknet rpc url
    env::set_var(
        "STARKNET_RPC_URL",
        "https://starknet-mainnet.infura.io/v3/<YOUR_API_KEY>",
    );
    // Set Beerus rpc address
    env::set_var("BEERUS_RPC_ADDR", "0.0.0.0:3030");

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = Config::from_env();

    let _beerus = BeerusLightClient::new(config.clone()).await?;
    println!("Constructed Beerus client!");
    Ok(())
}
