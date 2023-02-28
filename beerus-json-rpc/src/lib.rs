use eyre::Result;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use std::net::{AddrParseError, SocketAddr};
use thiserror::Error;
mod beerus_rpc;
use beerus_core::lightclient::beerus::BeerusLightClient;
use beerus_rpc::{BeerusApiServer, BeerusRpc};

#[derive(Error, Debug)]
pub enum RpcError {
    #[error(transparent)]
    JsonRpcServerError(#[from] jsonrpsee::core::Error),
    #[error(transparent)]
    ParseError(#[from] AddrParseError),
}

pub async fn run_server(beerus: BeerusLightClient) -> Result<(SocketAddr, ServerHandle), RpcError> {
    let socket_addr = format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or("3030".to_owned())
    )
    .parse::<SocketAddr>()
    .unwrap();

    let server = ServerBuilder::default().build(socket_addr).await?;
    let addr = server.local_addr()?;
    let rpc_calls = BeerusRpc::new(beerus);
    let handle = server.start(rpc_calls.into_rpc()).unwrap();

    Ok((addr, handle))
}
