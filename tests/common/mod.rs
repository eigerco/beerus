pub mod constants;
pub mod katana;
pub mod matchers;
pub mod node;

use std::sync::Arc;

use beerus::client::{Http, State};
use beerus::gen::Felt;
use beerus::{
    gen::client::Client,
    rpc::{serve, Server},
};
use thiserror::Error;
use tokio::sync::RwLock;

#[allow(dead_code)] // used in macros
pub struct Context {
    pub client: Client<Http>,
    pub server: Server,
}

#[allow(dead_code)] // used in macros
pub async fn ctx() -> Option<Context> {
    let url = std::env::var("BEERUS_TEST_STARKNET_URL").ok()?;

    let state = State {
        block_number: 652076,
        block_hash: Felt::try_new("0x189fbe3beb92b93c74f3cdeeca9445ce0c889242ca8eb0be4eeaaa42a3b215a").unwrap(),
        root: Felt::try_new("0x73be19f53a5f6daed7ac4f5111d75ed28f3b6ebbc51f058d3df3b47e51ffab9").unwrap(),
    };
    let state = Arc::new(RwLock::new(state));
    let server = serve(&url, "127.0.0.1:0", state.clone()).await.ok()?;
    tracing::info!(port = server.port(), "test server is up");

    let url = format!("http://localhost:{}/rpc", server.port());
    let client = Client::new(&url, Http::new());
    Some(Context { server, client })
}

#[macro_export]
macro_rules! setup {
    () => {{
        let run: bool = std::env::var("BEERUS_TEST_RUN")
            .ok()
            .map(|value| &value == "1")
            .unwrap_or_default();
        if !run {
            return Ok(());
        }
        if let Some(ctx) = common::ctx().await {
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
        if let Ok(url) = std::env::var("BEERUS_TEST_STARKNET_URL") {
            Client::new(&url, beerus::client::Http::new())
        } else {
            panic!("Invalid test setup");
        }
    }};
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("rpc call failed: {0:?}")]
    Rpc(#[from] iamgroot::jsonrpc::Error),
    #[error("missing env variable: {0:?}")]
    Var(#[from] std::env::VarError),
    #[error("execution failed: {0:?}")]
    Exe(#[from] beerus::exe::err::Error),
    #[error("serde failed: {0:?}")]
    Json(#[from] serde_json::Error),
    #[error("starknet api error: {0:?}")]
    Api(#[from] starknet_api::StarknetApiError),
}
