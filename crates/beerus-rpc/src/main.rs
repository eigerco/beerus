use beerus_core::{config::Config, lightclient::beerus::BeerusLightClient};
use beerus_rpc::BeerusRpc;
use env_logger::Env;
use log::{error, info};
use std::process::exit;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::from_env();

    info!("creating Beerus lightclient");
    let mut beerus = match BeerusLightClient::new(config.clone()).await {
        Ok(beerus) => beerus,
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    };

    info!("starting the Beerus light client...");
    if let Err(err) = beerus.start().await {
        error!("{}", err);
        exit(1);
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
            exit(1);
        }
    };
}
