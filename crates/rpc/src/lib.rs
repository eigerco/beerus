pub mod api;
pub mod error;

use api::BeerusRpcServer;
use beerus_core::client::BeerusClient;
use jsonrpsee::core::Error;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use std::net::SocketAddr;
use std::str::FromStr;

const DEFAULT_IP_V4_ADDR: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3030;

pub struct BeerusRpc {
    beerus: BeerusClient,
    rpc_address: SocketAddr,
}

impl BeerusRpc {
    pub fn new(beerus: BeerusClient) -> Self {
        Self { beerus, rpc_address: SocketAddr::from_str(&format!("{DEFAULT_IP_V4_ADDR}:{DEFAULT_PORT}")).unwrap() }
    }

    pub fn with_port(beerus: BeerusClient, port: u16) -> Self {
        Self { beerus, rpc_address: SocketAddr::from_str(&format!("{DEFAULT_IP_V4_ADDR}:{port}")).unwrap() }
    }

    pub fn with_addr(beerus: BeerusClient, addr: &str) -> Self {
        Self { beerus, rpc_address: SocketAddr::from_str(addr).unwrap() }
    }

    pub async fn run(self) -> Result<(SocketAddr, ServerHandle), Error> {
        let server = ServerBuilder::default().build(self.rpc_address).await.unwrap();

        let addr = server.local_addr()?;
        let handle = server.start(self.into_rpc());
        Ok((addr, handle))
    }
}
