use beerus_core::client::BeerusClient;
use beerus_core::config::Config;
use beerus_rpc::BeerusRpc;
use clap::Parser;
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[clap(short = 'c', long)]
    conf: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let config = match args.conf {
        Some(conf) => Config::from_file(&conf),
        None => Config::from_env(),
    };

    info!("init beerus client: {:?}", config.network);
    let mut beerus = BeerusClient::new(config.clone()).await;
    if let Err(e) = beerus.start().await {
        error!("{}", e);
        std::process::exit(1);
    };

    match BeerusRpc::new(beerus).run().await {
        Ok((addr, server_handle)) => {
            info!("Beerus JSON-RPC server started 🚀: http://{addr}");
            server_handle.stopped().await;
        }
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    };
}
