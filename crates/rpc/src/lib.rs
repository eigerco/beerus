pub mod api;
pub mod error;

use std::net::SocketAddr;

use api::BeerusRpcServer;
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

    pub async fn run(self) -> Result<(SocketAddr, ServerHandle), BeerusRpcError> {
        self.check_rpc_spec_versions().await?;

        let server =
            ServerBuilder::default().build(self.beerus.config.rpc_addr).await?;

        let addr = server.local_addr()?;
        let handle = server.start(self.into_rpc());
        Ok((addr, handle))
    }

    async fn check_rpc_spec_versions(&self) -> Result<(), BeerusRpcError> {
        let expected = self.spec_version().await?;

        let received = self.beerus.starknet_client
            .spec_version()
            .await?;

        if expected == received {
            Ok(())
        } else {
            Err(BeerusRpcError::SpecVersionMismatch { expected, received })
        }
    }
}
