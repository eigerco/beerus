use beerus_api::{
    gen::client::Client,
    rpc::{serve, Server},
};

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

pub async fn ctx() -> Option<Context> {
    let url = std::env::var("BEERUS_EXPERIMENTAL_TEST_STARKNET_URL").ok()?;
    let server = serve(&url, "127.0.0.1:0").await;
    tracing::info!(port = server.port(), "test server is up");

    let url = format!("http://localhost:{}/rpc", server.port());
    let client = Client::new(&url);
    Some(Context { server, client })
}
