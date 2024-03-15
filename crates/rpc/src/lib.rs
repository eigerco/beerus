pub mod api;
pub mod error;

use std::net::SocketAddr;

use api::{BeerusRpcServer, SPEC_VERION};
use beerus_core::client::BeerusClient;
use error::RunError;
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
    ) -> Result<(SocketAddr, ServerHandle), RunError> {
        self.check_spec_version(SPEC_VERION).await?;

        let server =
            ServerBuilder::default().build(self.beerus.config.rpc_addr).await?;

        let addr = server.local_addr()?;
        let handle = server.start(self.into_rpc());
        Ok((addr, handle))
    }

    async fn check_spec_version(
        &self,
        expected: &str,
    ) -> Result<(), RunError> {
        let actual = self.beerus.starknet_client.spec_version().await?;
        if actual != expected {
            return Err(RunError::WrongSpecVersion(
                actual,
                expected.to_owned(),
            ));
        }
        Ok(())
    }
}
