use beerus_api::{
    gen::client::Client,
    rpc::{serve, Server},
};

use async_once_cell::OnceCell;

#[derive(Debug)]
pub enum Error {
    Rpc(iamgroot::jsonrpc::Error),
}

impl From<iamgroot::jsonrpc::Error> for Error {
    fn from(error: iamgroot::jsonrpc::Error) -> Self {
        Self::Rpc(error)
    }
}

pub struct Context {
    pub client: Client,
    pub server: Server,
}

async fn setup() -> Option<Context> {
    tracing_subscriber::fmt::init();

    let url = std::env::var("TEST_URL").ok()?;
    let server = serve(&url, "127.0.0.1:0").await;
    tracing::info!(port = server.port(), "test server is up");

    let url = format!("http://localhost:{}/rpc", server.port());
    let client = Client::new(&url);
    Some(Context { server, client })
}

// `rstest` does not support async functions as fixtures,
// thus such "manual" approach with `async_once_cell`
pub async fn ctx<'a>() -> &'a Option<Context> {
    static ONCE: OnceCell<Option<Context>> = OnceCell::new();
    ONCE.get_or_init(async { setup().await }).await
}
