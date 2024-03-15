pub mod api;
pub mod error;

use std::net::SocketAddr;

use api::{BeerusRpcServer, SPEC_VERION};
use beerus_core::client::BeerusClient;
use error::BeerusRpcError;
use eyre::eyre;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use starknet::providers::Provider;

pub struct BeerusRpc {
    beerus: BeerusClient,
}

impl BeerusRpc {
    pub fn new(beerus: BeerusClient) -> Self {
        Self { beerus }
    }

    pub async fn run(self) -> eyre::Result<(SocketAddr, ServerHandle)> {
        let remote_spec_version =
            self.beerus.starknet_client.spec_version().await?;

        if remote_spec_version != SPEC_VERION {
            return Err(eyre!(
                "unexpected RPC spec version: {0}",
                remote_spec_version
            ));
        }

        // build the RPC server
        let server =
            ServerBuilder::default().build(self.beerus.config.rpc_addr).await?;

        // start the RPC Server
        let addr = server.local_addr().map_err(BeerusRpcError::from)?;
        let handle = server.start(self.into_rpc());
        Ok((addr, handle))
    }
}
