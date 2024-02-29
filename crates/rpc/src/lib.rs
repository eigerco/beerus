pub mod api;
pub mod error;

use std::net::SocketAddr;

use api::BeerusRpcServer;
use beerus_core::client::BeerusClient;
use error::BeerusRpcError;
use jsonrpsee::core::Error;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use starknet::providers::Provider;
use tracing::error;

pub struct BeerusRpc {
    beerus: BeerusClient,
}

impl BeerusRpc {
    pub fn new(beerus: BeerusClient) -> Self {
        Self { beerus }
    }

    pub async fn run(self) -> Result<(SocketAddr, ServerHandle), Error> {
        let local_spec_version =
            self.spec_version().await.map_err(BeerusRpcError::from).unwrap();

        let remote_spec_version = self
            .beerus
            .starknet_client
            .spec_version()
            .await
            .map_err(BeerusRpcError::from)
            .unwrap();

        if local_spec_version != remote_spec_version {
            error!(
                "Spec version mismatch between Beerus {} and remote Starknet RPC {}", local_spec_version, remote_spec_version
            );
            return Err(Error::Custom(
                format!("Spec version mismatch between Beerus {} and remote Starkent RPC {}", local_spec_version, remote_spec_version)
                    .to_string(),
            ));
        }

        // build the RPC server
        let server =
            ServerBuilder::default().build(self.beerus.config.rpc_addr).await?;

        // start the RPC Server
        let addr = server.local_addr()?;
        let handle = server.start(self.into_rpc());
        Ok((addr, handle))
    }
}
