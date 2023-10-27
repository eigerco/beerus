use beerus_core::client::BeerusClient;
use beerus_core::config::Config;
use beerus_rpc::BeerusRpc;
use clap::Parser;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[clap(short = 'c', long)]
    conf: String,
}

#[async_std::main]
async fn main() {
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Args::parse();
    let config = Config::from_file(&args.conf);

    info!("init beerus client: {:?}", config.network);
    let mut beerus = BeerusClient::new(config.clone()).await;
    if let Err(err) = beerus.start().await {
        error! {"{}", err};
        std::process::exit(1);
    };

    match BeerusRpc::new(beerus).run().await {
        Ok((addr, server_handle)) => {
            info!("========================================================");
            info!("Beerus JSON-RPC server started 🚀: http://{addr}");
            info!("========================================================");

            server_handle.stopped().await;
        }
        Err(err) => {
            error! {"{}", err};
            std::process::exit(1);
        }
    };
}
