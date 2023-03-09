use beerus_core::lightclient::beerus::BeerusLightClient;
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result},
    proc_macros::rpc,
};

use starknet::providers::jsonrpc::models::BlockHashAndNumber;

pub struct BeerusRpc {
    _beerus: BeerusLightClient,
}

#[rpc(server, client)]
trait BeerusApi {
    #[method(name = "hello_world")]
    async fn hello_world(&self) -> Result<String>;

    #[method(name = "stark_blockHashAndNumber")]
    async fn get_block_hash_and_number(&self) -> Result<BlockHashAndNumber>;
}

#[async_trait]
impl BeerusApiServer for BeerusRpc {
    async fn hello_world(&self) -> Result<String> {
        Ok("Hello World!".to_string())
    }

    async fn get_block_hash_and_number(&self) -> Result<BlockHashAndNumber> {
        Ok(self
            ._beerus
            .starknet_lightclient
            .block_hash_and_number()
            .await
            .unwrap())
    }
}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { _beerus: beerus }
    }
}
