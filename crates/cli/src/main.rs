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
}

#[async_std::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    let config = Config::from_file(&args.conf);
    println!("CONFIG: {:#?}", config);

    info!("creating Beerus lightclient");
    let mut beerus = BeerusClient::new(config.clone()).await;

    info!("starting the Beerus light client...");
    if let Err(err) = beerus.start().await {
        error!("{}", err);
        std::process::exit(1);
    };

    info!("starting beerus rpc server...");
    match BeerusRpc::new(beerus).run().await {
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
