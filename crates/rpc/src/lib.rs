pub mod api;
pub mod error;

use std::net::SocketAddr;

use api::BeerusRpcServer;
use beerus_core::client::BeerusClient;
use jsonrpsee::core::Error;
use jsonrpsee::server::{ServerBuilder, ServerHandle};

pub struct BeerusRpc {
    beerus: BeerusClient,
}

impl BeerusRpc {
    pub fn new(beerus: BeerusClient) -> Self {
        Self { beerus }
    }

    pub async fn run(self) -> Result<(SocketAddr, ServerHandle), Error> {
        // build the RPC server
        let server = ServerBuilder::default().build(self.beerus.config.rpc_addr).await.unwrap();

        // start the RPC Server
        let addr = server.local_addr()?;
        let handle = server.start(self.into_rpc());
        Ok((addr, handle))
    }
}
