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
    conf: Option<String>,
}

#[async_std::main]
async fn main() {
    use clap::Parser;
    use starknet::providers::Provider;
    use starknet::{
        core::{chain_id, types::*},
        macros::short_string,
        providers::{
            jsonrpc::HttpTransport,
            JsonRpcClient,
            // AnyProvider,Provider, ProviderError,
        },
    };
    use url::Url;

    /// Simple program to greet a person
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        #[clap(long = "rpc", help = "Starknet JSON-RPC endpoint")]
        rpc: Url,
    }
    let args = Args::parse();
    let provider = JsonRpcClient::new(HttpTransport::new(args.rpc));
    dbg!(provider.chain_id().await);
    std::process::exit(0);
    //     // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    //     let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    //
    //     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber
    // failed");
    //
    //     let args = Args::parse();
    //     let config = match args.conf {
    //         Some(conf) => Config::from_file(&conf),
    //         None => Config::from_env(),
    //     };
    //
    //     info!("init beerus client: {:?}", config.network);
    //     let mut beerus = BeerusClient::new(config.clone()).await;
    //     if let Err(err) = beerus.start().await {
    //         error! {"{}", err};
    //         std::process::exit(1);
    //     };
    //
    //     match BeerusRpc::new(beerus).run().await {
    //         Ok((addr, server_handle)) => {
    //             info!("========================================================");
    //             info!("Beerus JSON-RPC server started 🚀: http://{addr}");
    //             info!("========================================================");
    //
    //             server_handle.stopped().await;
    //         }
    //         Err(err) => {
    //             error! {"{}", err};
    //             std::process::exit(1);
    //         }
    //     };
}
