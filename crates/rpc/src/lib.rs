pub mod api;
pub mod error;

use std::net::SocketAddr;

use api::BeerusRpcServer;
use beerus_core::public::Beerus;
use jsonrpsee::core::Error;
use jsonrpsee::server::{ServerBuilder, ServerHandle};

pub struct BeerusRpc {
    beerus: Beerus,
}

impl BeerusRpc {
    pub fn new(beerus: Beerus) -> Self {
        Self { beerus }
    }

    pub async fn run(self, listen_address: SocketAddr) -> Result<(SocketAddr, ServerHandle), Error> {
        // build the RPC server
        let server = ServerBuilder::default().build(listen_address).await.unwrap();

        // start the RPC Server
        let addr = server.local_addr()?;
        let handle = server.start(self.into_rpc());
        Ok((addr, handle))
    }
}
