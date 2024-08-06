use std::sync::Arc;
use tokio::sync::RwLock;

use beerus::client::State;
use beerus::gen::Felt;
use beerus::{
    gen::client::Client,
    rpc::{serve, Server},
};

#[allow(dead_code)] // used in macros
pub struct Context {
    pub client: Client,
    pub server: Server,
}

#[allow(dead_code)] // used in macros
pub async fn ctx() -> Option<Context> {
    let url = std::env::var("BEERUS_TEST_STARKNET_URL").ok()?;
    let state = Arc::new(RwLock::new(State {
        block_number: 652076,
        block_hash: Felt::try_new("0x0").unwrap(),
        root: Felt::try_new(
            "0x2a5aa70350b7d047cd3dd2f5ad01f8925409a64fc42e509e8e79c3a2c17425",
        )
        .unwrap(),
    }));
    let server = serve(&url, "127.0.0.1:0", state).await.ok()?;
    tracing::info!(port = server.port(), "test server is up");

    let client =
        Client::new(&format!("http://localhost:{}/rpc", server.port()));
    Some(Context { server, client })
}
