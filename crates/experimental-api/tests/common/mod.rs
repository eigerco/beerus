use beerus_experimental_api::{
    gen::client::Client,
    rpc::{serve, Server},
};

#[derive(Debug)]
pub enum Error {
    Rpc(iamgroot::jsonrpc::Error),
    Var(std::env::VarError),
    Exe(beerus_experimental_api::exe::err::Error),
    Json(serde_json::Error),
}

impl From<iamgroot::jsonrpc::Error> for Error {
    fn from(error: iamgroot::jsonrpc::Error) -> Self {
        Self::Rpc(error)
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Self::Var(error)
    }
}

impl From<beerus_experimental_api::exe::err::Error> for Error {
    fn from(error: beerus_experimental_api::exe::err::Error) -> Self {
        Self::Exe(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

pub struct Context {
    pub client: Client,
    pub server: Server,
}

pub async fn ctx() -> Option<Context> {
    let url = std::env::var("BEERUS_EXPERIMENTAL_TEST_STARKNET_URL").ok()?;
    let server = serve(&url, "127.0.0.1:0").await.ok()?;
    tracing::info!(port = server.port(), "test server is up");

    let url = format!("http://localhost:{}/rpc", server.port());
    let client = Client::new(&url);
    Some(Context { server, client })
}
