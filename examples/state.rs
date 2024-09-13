use beerus::client::{Client, Http};
use beerus::config::Config;
use eyre::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = std::env::var("ALCHEMY_API_KEY")
        .context("ALCHEMY_API_KEY is missing")?;

    let config = Config {
        ethereum_rpc: format!(
            "https://eth-mainnet.g.alchemy.com/v2/{api_key}"
        ),
        starknet_rpc: format!(
            "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0.6/{api_key}"
        ),
        data_dir: "tmp".to_owned(),
    };

    let http = Http::new();
    let beerus = Client::new(&config, http).await?;

    let state = beerus.get_state().await?;
    tracing::info!("{state:#?}");

    Ok(())
}
