use beerus_core::lightclient::beerus::BeerusLightClient;
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result},
    proc_macros::rpc,
};

use beerus_core::starknet_helper::block_id_string_to_block_id_type;
pub struct BeerusRpc {
    _beerus: BeerusLightClient,
}

#[rpc(server, client)]
trait BeerusApi {
    #[method(name = "hello_world")]
    async fn hello_world(&self) -> Result<String>;

    #[method(name = "stark_blockTransactionCount")]
    async fn stark_block_transaction_count(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<u64>;
}

#[async_trait]
impl BeerusApiServer for BeerusRpc {
    async fn hello_world(&self) -> Result<String> {
        Ok("Hello World!".to_string())
    }

    async fn stark_block_transaction_count(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<u64> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();
        let block_transaction_count = self
            ._beerus
            .starknet_lightclient
            .get_block_transaction_count(&block_id)
            .await
            .unwrap();
        Ok(block_transaction_count)
    }
}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { _beerus: beerus }
    }
}
