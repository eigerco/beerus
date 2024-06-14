use std::sync::Arc;

use beerus_core::client::NodeData;
use beerus_experimental_api::{
    gen::client::Client,
    rpc::{serve, Server},
};
use starknet_crypto::FieldElement;
use thiserror::Error;
use tokio::sync::RwLock;

#[allow(dead_code)] // used in macros
pub struct Context {
    pub client: Client,
    pub server: Server,
}

#[allow(dead_code)] // used in macros
pub async fn ctx() -> Option<Context> {
    let url = std::env::var("BEERUS_EXPERIMENTAL_TEST_STARKNET_URL").ok()?;
    let node = Arc::new(RwLock::new(NodeData {
        l1_block_number: 647978,
        l1_state_root: FieldElement::from_hex_be(
            "0x2a5aa70350b7d047cd3dd2f5ad01f8925409a64fc42e509e8e79c3a2c17425",
        )
        .unwrap(),
        ..Default::default()
    }));
    let server = serve(&url, "127.0.0.1:0", node.clone()).await.ok()?;
    tracing::info!(port = server.port(), "test server is up");

    let url = format!("http://localhost:{}/rpc", server.port());
    let client = Client::new(&url);
    Some(Context { server, client })
}

#[macro_export]
macro_rules! setup {
    () => {{
        let run: bool = std::env::var("BEERUS_EXPERIMENTAL_TEST_RUN")
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
        let run: bool = std::env::var("BEERUS_EXPERIMENTAL_TEST_RUN")
            .ok()
            .map(|value| &value == "1")
            .unwrap_or_default();
        if !run {
            return Ok(());
        }
        if let Ok(url) = std::env::var("BEERUS_EXPERIMENTAL_TEST_STARKNET_URL")
        {
            Client::new(&url)
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
    Exe(#[from] beerus_experimental_api::exe::err::Error),
    #[error("serde failed: {0:?}")]
    Json(#[from] serde_json::Error),
    #[error("starknet api error: {0:?}")]
    Api(#[from] starknet_api::StarknetApiError),
}
