use beerus_core::lightclient::beerus::BeerusLightClient;
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result},
    proc_macros::rpc,
};

use starknet::core::types::FieldElement;
use std::str::FromStr;

pub struct BeerusRpc {
    _beerus: BeerusLightClient,
}

#[rpc(server, client)]
trait BeerusApi {
    #[method(name = "hello_world")]
    async fn hello_world(&self) -> Result<String>;

    #[method(name = "stark_nonce")]
    async fn stark_nonce(&self, contract_address: String) -> Result<String>;
}

#[async_trait]
impl BeerusApiServer for BeerusRpc {
    async fn hello_world(&self) -> Result<String> {
        Ok("Hello World!".to_string())
    }

    async fn stark_nonce(&self, contract_address: String) -> Result<String> {
        let contract_address = FieldElement::from_str(&contract_address).unwrap();
        let nonce = self
            ._beerus
            .starknet_get_nonce(contract_address)
            .await
            .unwrap()
            .to_string();
        Ok(nonce)
    }
}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { _beerus: beerus }
    }
}
