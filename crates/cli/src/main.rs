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

fn get_config(args: Args) -> eyre::Result<Config> {
    Ok(if let Some(path) = args.config.as_ref() {
        Config::from_file(path)?
    } else {
        Config::from_env()
    })
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();
    let config = get_config(Args::parse())?;

    info!("init beerus client: {:?}", config.network);
    let mut beerus = BeerusClient::new(&config).await?;
    beerus.start().await?;

    #[cfg(feature = "experimental")]
    tokio::spawn(async move {
        beerus_api::rpc::serve(&config.starknet_rpc, "127.0.0.1:9000").await;
    });

    let (address, server) = BeerusRpc::new(beerus).run().await?;
    info!("Beerus JSON-RPC server started 🚀: http://{address}");
    server.stopped().await;

    Ok(())
}
