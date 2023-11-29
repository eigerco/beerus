use beerus_core::config::Config;
use beerus_core::Beerus;
use beerus_rpc::BeerusRpc;
use clap::Parser;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[clap(short = 'c', long)]
    conf: Option<String>,
}

#[async_std::main]
async fn main() {
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Args::parse();
    let config = match args.conf {
        Some(conf) => Config::from_file(&conf),
        None => Config::from_env(),
    };

    info!("init beerus client: {:?}", config.network);
    let mut beerus = Beerus::new(config.clone()).await;
    if let Err(err) = beerus.start().await {
        error! {"{}", err};
        std::process::exit(1);
    };

    match BeerusRpc::new(beerus).run(config.rpc_addr).await {
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
