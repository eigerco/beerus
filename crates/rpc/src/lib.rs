pub mod api;
pub mod error;

use std::net::SocketAddr;

use api::{BeerusRpcServer, SPEC_VERION};
use beerus_core::client::BeerusClient;
use error::BeerusRpcError;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use starknet::providers::Provider;

pub struct BeerusRpc {
    beerus: BeerusClient,
}

impl BeerusRpc {
    pub fn new(beerus: BeerusClient) -> Self {
        Self { beerus }
    }

    pub async fn run(
        self,
    ) -> Result<(SocketAddr, ServerHandle), BeerusRpcError> {
        let remote_spec_version =
            self.beerus.starknet_client.spec_version().await?;

        if remote_spec_version != SPEC_VERION {
            return Err(BeerusRpcError::Other{code: -32601, msg: format!("Spec version mismatch between Beerus {} and remote Starkent RPC {}", SPEC_VERION, remote_spec_version)});
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
