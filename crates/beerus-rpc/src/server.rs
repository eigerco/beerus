use std::str::FromStr;

use beerus_core::lightclient::beerus::BeerusLightClient;
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result, Error::Custom},
    proc_macros::rpc,
    types::error::CallError,
};

use beerus_core::starknet_helper::block_id_string_to_block_id_type;
use ethers::types::U256;
use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::models::{
    BlockHashAndNumber, ContractClass, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
    MaybePendingTransactionReceipt, StateUpdate, SyncStatusType,Transaction
};

pub struct BeerusRpc {
    _beerus: BeerusLightClient,
}

#[rpc(server, client)]
trait BeerusApi {
    #[method(name = "hello_world")]
    async fn hello_world(&self) -> Result<String>;

    #[method(name = "starknet_l2_to_l1_messages")]
    async fn starknet_l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256>;

    #[method(name = "starknet_chainId")]
    async fn starknet_chain_id(&self) -> Result<String>;

    #[method(name = "starknet_blockNumber")]
    async fn starknet_block_number(&self) -> Result<u64>;

    #[method(name = "starknet_getBlockTransactionCount")]
    async fn starknet_get_block_transaction_count(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<u64>;

    #[method(name = "starknet_getClassAt")]
    async fn starknet_get_class_at(
        &self,
        block_id_type: String,
        block_id: String,
        contract_address: String,
    ) -> Result<ContractClass>;

    #[method(name = "starknet_blockHashAndNumber")]
    async fn starknet_block_hash_and_number(&self) -> Result<BlockHashAndNumber>;

    #[method(name = "starknet_getBlockWithTxHashes")]
    async fn stark_get_block_with_tx_hashes(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<MaybePendingBlockWithTxHashes>;

    #[method(name = "starknet_getTransactionByBlockIdAndIndex")]
    async fn starknet_get_transaction_by_block_id_and_index(
        &self,
        block_id_type: &str,
        block_id: &str,
        index: &str,
    ) -> Result<Transaction>;

    #[method(name = "starknet_getBlockWithTxs")]
    async fn starknet_get_block_with_txs(
        &self,
        block_id_type: &str,
        block_id: &str,
    ) -> Result<MaybePendingBlockWithTxs>;

    #[method(name = "starknet_getStateUpdate")]
    async fn starknet_get_state_update(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<StateUpdate>;

    #[method(name = "starknet_syncing")]
    async fn starknet_syncing(&self) -> Result<SyncStatusType>;

    #[method(name = "starknet_l1_to_l2_messages")]
    async fn starknet_l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256>;

    #[method(name = "starknet_l1_to_l2_message_nonce")]
    async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256>;

    #[method(name = "starknet_l1_to_l2_message_cancellations")]
    async fn starknet_l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256>;

    #[method(name = "starknet_getTransactionReceipt")]
    async fn starknet_get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<MaybePendingTransactionReceipt>;

    #[method(name = "starknet_getClassHash")]
    async fn starknet_get_class_hash(
        &self,
        block_id_type: String,
        block_id: String,
        contract_address: String,
    ) -> Result<FieldElement>;

    #[method(name = "starknet_pendingTransactions")]
    async fn starknet_pending_transactions(&self) -> Result<Vec<Transaction>>;

}

#[async_trait]
impl BeerusApiServer for BeerusRpc {
    async fn hello_world(&self) -> Result<String> {
        Ok("Hello World!".to_string())
    }

    async fn starknet_l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256> {
        Ok(self
            ._beerus
            .starknet_l2_to_l1_messages(msg_hash)
            .await
            .unwrap())
    }

    async fn starknet_chain_id(&self) -> Result<String> {
        let chain_id = self
            ._beerus
            .starknet_lightclient
            .chain_id()
            .await
            .unwrap()
            .to_string();

        Ok(chain_id)
    }

    async fn starknet_block_number(&self) -> Result<u64> {
        let block_number = self
            ._beerus
            .starknet_lightclient
            .block_number()
            .await
            .unwrap();

        Ok(block_number)
    }

    async fn starknet_get_block_transaction_count(
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

    async fn starknet_block_hash_and_number(&self) -> Result<BlockHashAndNumber> {
        Ok(self
            ._beerus
            .starknet_lightclient
            .block_hash_and_number()
            .await
            .unwrap())
    }

    async fn starknet_get_class_at(
        &self,
        block_id_type: String,
        block_id: String,
        contract_address: String,
    ) -> Result<ContractClass> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();
        let contract_address = FieldElement::from_str(&contract_address).unwrap();
        Ok(self
            ._beerus
            .starknet_lightclient
            .get_class_at(&block_id, contract_address)
            .await
            .unwrap())
    }

    async fn stark_get_block_with_tx_hashes(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<MaybePendingBlockWithTxHashes> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();
        Ok(self
            ._beerus
            .starknet_lightclient
            .get_block_with_tx_hashes(&block_id)
            .await
            .unwrap())
    }

    async fn starknet_get_transaction_by_block_id_and_index(
        &self,
        block_id_type: &str,
        block_id: &str,
        index: &str,
    ) -> Result<Transaction> {
        let block_id =
            beerus_core::starknet_helper::block_id_string_to_block_id_type(block_id_type, block_id)
                .map_err(|e| {
                    jsonrpsee::core::Error::Call(CallError::InvalidParams(anyhow::anyhow!(
                        e.to_string()
                    )))
                })?;
        let index = u64::from_str(index).map_err(|e| {
            jsonrpsee::core::Error::Call(CallError::InvalidParams(anyhow::anyhow!(e.to_string())))
        })?;
        let result = self
            ._beerus
            .starknet_lightclient
            .get_transaction_by_block_id_and_index(&block_id, index)
            .await
            .map_err(|e| {
                jsonrpsee::core::Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string())))
            })?;
        Ok(result)
    }
    async fn starknet_get_block_with_txs(
        &self,
        block_id_type: &str,
        block_id: &str,
    ) -> Result<MaybePendingBlockWithTxs> {
        let block_id =
            beerus_core::starknet_helper::block_id_string_to_block_id_type(block_id_type, block_id)
                .map_err(|e| {
                    jsonrpsee::core::Error::Call(CallError::InvalidParams(anyhow::anyhow!(
                        e.to_string()
                    )))
                })?;
        let result = self
            ._beerus
            .starknet_lightclient
            .get_block_with_txs(&block_id)
            .await
            .map_err(|e| {
                jsonrpsee::core::Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string())))
            })?;
        Ok(result)
    }

    async fn starknet_get_state_update(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<StateUpdate> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();
        Ok(self
            ._beerus
            .starknet_lightclient
            .get_state_update(&block_id)
            .await
            .unwrap())
    }

    async fn starknet_syncing(&self) -> Result<SyncStatusType> {
        let sync_status_type = self._beerus.starknet_lightclient.syncing().await.unwrap();
        Ok(sync_status_type)
    }

    async fn starknet_l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256> {
        Ok(self
            ._beerus
            .starknet_l1_to_l2_messages(msg_hash)
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

    async fn starknet_l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256> {
        Ok(self
            ._beerus
            .starknet_l1_to_l2_message_cancellations(msg_hash)
            .await
            .unwrap())
    }

    async fn starknet_get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<MaybePendingTransactionReceipt> {
        let tx_hash_felt = FieldElement::from_hex_be(&tx_hash).unwrap();
        Ok(self
            ._beerus
            .starknet_lightclient
            .get_transaction_receipt(tx_hash_felt)
            .await
            .unwrap())
    }

    async fn starknet_get_class_hash(
        &self,
        block_id_type: String,
        block_id: String,
        contract_address: String,
    ) -> Result<FieldElement> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();
        let contract_address = FieldElement::from_str(&contract_address).unwrap();

        Ok(self
            ._beerus
            .starknet_lightclient
            .get_class_hash_at(&block_id, contract_address)
            .await
            .unwrap())
    }

    async fn starknet_pending_transactions(&self) -> Result<Vec<Transaction>> {
        let transactions = self
            ._beerus
            .starknet_lightclient
            .pending_transactions()
            .await
            .map_err(|e| Custom(e.to_string()));
        Ok(transactions.unwrap())
    }

}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { _beerus: beerus }
    }
}
