use beerus_core::client::BeerusClient;
use beerus_core::config::Config;
use beerus_rpc::BeerusRpc;

use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[clap(short = 'c', long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let config = args.config
        .map(|config| Config::from_file(&config))
        .unwrap_or_else(Config::from_env);

    info!("init beerus client: {:?}", config.network);
    let mut beerus = BeerusClient::new(config).await
        .map_err(|e| format!("failed to setup beerus client: {e}"))?;
    beerus.start().await
        .map_err(|e| format!("failed to start beerus client: {e}"))?;

    let (address, server) = BeerusRpc::new(beerus).run().await
        .map_err(|e| format!("failed to start JSON-RPC server: {e}"))?;
    info!("Beerus JSON-RPC server started 🚀: http://{address}");
    server.stopped().await;

    Ok(())
}
