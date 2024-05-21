use beerus_cli::{get_config, Args};
use beerus_core::client::BeerusClient;
use beerus_rpc::BeerusRpc;

use clap::Parser;
use tracing::info;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();
    let config = get_config(Args::parse())?;
    config.validate_params().await?;

    info!("init beerus client: {:?}", config.network);
    let mut beerus = BeerusClient::new(&config).await?;
    beerus.start().await?;

    let (address, server) = BeerusRpc::new(beerus).run().await?;
    info!("Beerus JSON-RPC server started 🚀: http://{address}");
    server.stopped().await;

    Ok(())
}
