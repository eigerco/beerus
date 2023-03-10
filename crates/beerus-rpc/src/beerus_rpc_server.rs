use beerus_core::lightclient::beerus::BeerusLightClient;
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result},
    proc_macros::rpc,
};

use beerus_core::starknet_helper::block_id_string_to_block_id_type;
use ethers::types::U256;
use starknet::providers::jsonrpc::models::BlockHashAndNumber;

pub struct BeerusRpc {
    _beerus: BeerusLightClient,
}

#[rpc(server, client)]
trait BeerusApi {
    #[method(name = "hello_world")]
    async fn hello_world(&self) -> Result<String>;

    #[method(name = "stark_chainId")]
    async fn stark_chain_id(&self) -> Result<String>;

    #[method(name = "stark_blockNumber")]
    async fn stark_block_number(&self) -> Result<u64>;

    #[method(name = "stark_blockTransactionCount")]
    async fn stark_block_transaction_count(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<u64>;

    #[method(name = "stark_blockHashAndNumber")]
    async fn get_block_hash_and_number(&self) -> Result<BlockHashAndNumber>;

    #[method(name = "starknet_l1_to_l2_message_nonce")]
    async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256>;
}

#[async_trait]
impl BeerusApiServer for BeerusRpc {
    async fn hello_world(&self) -> Result<String> {
        Ok("Hello World!".to_string())
    }

    async fn stark_chain_id(&self) -> Result<String> {
        let chain_id = self
            ._beerus
            .starknet_lightclient
            .chain_id()
            .await
            .unwrap()
            .to_string();

        Ok(chain_id)
    }

    async fn stark_block_number(&self) -> Result<u64> {
        let block_number = self
            ._beerus
            .starknet_lightclient
            .block_number()
            .await
            .unwrap();

        Ok(block_number)
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

    async fn get_block_hash_and_number(&self) -> Result<BlockHashAndNumber> {
        Ok(self
            ._beerus
            .starknet_lightclient
            .block_hash_and_number()
            .await
            .unwrap())
    }

    async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256> {
        let nonce = self
            ._beerus
            .starknet_l1_to_l2_message_nonce()
            .await
            .unwrap();
        Ok(nonce)
    }
}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { _beerus: beerus }
    }
}
