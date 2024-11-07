use std::sync::Arc;

use beerus::client::{Http, State};
use beerus::gen::Felt;
use beerus::{
    gen::client::Client,
    rpc::{serve, Server},
};
use tokio::sync::RwLock;

#[allow(dead_code)] // used in macros
pub struct Context {
    pub client: Client<Http>,
    pub server: Server,
}

#[allow(dead_code)]
pub async fn ctx_mainnet() -> Option<Context> {
    let url = std::env::var("STARKNET_MAINNET_URL").ok()?;

    let state = State {
        block_number: 652076,
        block_hash: Felt::try_new(
            "0x189fbe3beb92b93c74f3cdeeca9445ce0c889242ca8eb0be4eeaaa42a3b215a",
        )
        .unwrap(),
        root: Felt::try_new(
            "0x73be19f53a5f6daed7ac4f5111d75ed28f3b6ebbc51f058d3df3b47e51ffab9",
        )
        .unwrap(),
    };
    ctx(url, state).await
}

#[allow(dead_code)]
pub async fn ctx_sepolia() -> Option<Context> {
    let url = std::env::var("STARKNET_SEPOLIA_URL").ok()?;

    let state = State {
        block_number: 293268,
        block_hash: Felt::try_new(
            "0x7799ec4953a1786e59e5ad02b4576cd59fa3b9efa059b7d56a9eb2b6ad6f2e",
        )
        .unwrap(),
        root: Felt::try_new(
            "0x54882b0dcb575e5e18bfac4c22b00f0cadcd83885d8c35b0b9d6e0e125ce3be",
        )
        .unwrap(),
    };
    ctx(url, state).await
}

#[allow(dead_code)] // used in macros
async fn ctx(url: String, state: State) -> Option<Context> {
    let state = Arc::new(RwLock::new(state));
    let server = serve(&url, "127.0.0.1:0", state.clone()).await.ok()?;
    tracing::info!(port = server.port(), "test server is up");

    let url = format!("http://localhost:{}/rpc", server.port());
    let client = Client::new(&url, Http::new());
    Some(Context { server, client })
}

#[macro_export]
macro_rules! setup {
    () => {
        setup!("mainnet")
    };
    ($e:expr) => {{
        let run: bool = std::env::var("BEERUS_TEST_RUN")
            .ok()
            .map(|value| &value == "1")
            .unwrap_or_default();
        if !run {
            return Ok(());
        }
        let context = match $e {
            "sepolia" => common::ctx::ctx_sepolia().await,
            "mainnet" => common::ctx::ctx_mainnet().await,
            unknown => panic!("Unknown network: {unknown}. Supported networks: mainnet, sepolia"),
        };
        if let Some(ctx) = context {
            ctx
        } else {
            panic!("Invalid test setup");
        }
    }};
}

#[macro_export]
macro_rules! client {
    () => {{
        let run: bool = std::env::var("BEERUS_TEST_RUN")
            .ok()
            .map(|value| &value == "1")
            .unwrap_or_default();
        if !run {
            return Ok(());
        }
        if let Ok(url) = std::env::var("STARKNET_MAINNET_URL") {
            Client::new(&url, beerus::client::Http::new())
        } else {
            panic!("Invalid test setup");
        }
    }};
}
