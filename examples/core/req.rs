use clap::Parser;
use starknet::providers::Provider;
use starknet::{
    core::{chain_id, types::*},
    macros::short_string,
    providers::{
        jsonrpc::HttpTransport,  JsonRpcClient,
        // AnyProvider,Provider, ProviderError,
    },
};
use url::Url;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(
        long = "rpc",
        env = "STARKNET_RPC",
        help = "Starknet JSON-RPC endpoint"
    )]
    rpc: Url,
}

// #[tokio::main]
#[async_std::main]
async fn main() {
    let args = Args::parse();

    let provider = JsonRpcClient::new(HttpTransport::new(args.rpc));
    //
    //
    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }
    dbg!(provider.chain_id().await);
    // match provider.chain_id().await {
    //     Err(e) => dbg!(e),
    //     Ok(res) => dbg!(res),
    // }
}

