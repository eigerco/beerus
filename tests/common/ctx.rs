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
        block_number: 209582,
        block_hash: Felt::try_new(
            "0x45fb3ae1436743e74a81eac88e1beee6e8fb34aecb7b2e43e0577406f390f5f",
        )
        .unwrap(),
        root: Felt::try_new(
            "0x11dccdce33557ca6e14871d4235a8b65e9bd512722ac9e2cb96ff49bfb9af30",
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
            _ => common::ctx::ctx_mainnet().await,
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
