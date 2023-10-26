use beerus_core::client::BeerusClient;
use beerus_core::config::Config;
use beerus_rpc::BeerusRpc;
use clap::Parser;
use env_logger::Env;
use log::{error, info};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[clap(short = 'c', long)]
    conf: String,
    #[clap(short = 'p', long)]
    port: Option<u16>,
}

#[async_std::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    let config = Config::from_file(&args.conf);

    info!("creating Beerus lightclient");
    let mut beerus = BeerusClient::new(config.clone()).await;

    info!("starting the Beerus light client...");
    if let Err(err) = beerus.start().await {
        error!("{}", err);
        std::process::exit(1);
    };

    info!("starting beerus rpc server...");
    let rpc_client = match args.port {
        Some(p) => BeerusRpc::with_port(beerus, p),
        None => BeerusRpc::new(beerus),
    };

    match rpc_client.run().await {
        Ok((addr, server_handle)) => {
            info!("===================================================");
            info!("Beerus JSON-RPC server started 🚀: http://{addr}");
            info!("===================================================");

            server_handle.stopped().await;
        }
        Err(err) => {
            error! {"{}", err};
            std::process::exit(1);
        }
    };
}
