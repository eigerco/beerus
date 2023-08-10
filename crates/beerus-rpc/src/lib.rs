pub mod api;
pub mod errors;
pub mod models;
pub mod utils;

use crate::api::BeerusRpcServer;
use crate::models::EventFilterWithPage;
use beerus_core::{
    ethers_helper::{parse_eth_address, parse_eth_hash},
    lightclient::starknet::storage_proof::GetProofOutput,
};

use helios::types::{BlockTag, CallOpts, ExecutionBlock};
use jsonrpsee::{
    core::{async_trait, Error},
    server::{ServerBuilder, ServerHandle},
};

use crate::errors::{invalid_call_data, INTERNAL_SERVER_ERROR, INVALID_CALL_DATA};
use beerus_core::lightclient::beerus::BeerusLightClient;
use errors::BeerusApiError;
use ethers::types::{
    Address, Filter, Log, SyncingStatus, Transaction as EthTransaction, TransactionReceipt, H256,
    U256,
};
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::models::{
        BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
        BroadcastedDeployAccountTransaction, BroadcastedDeployTransaction,
        BroadcastedInvokeTransaction, BroadcastedTransaction, ContractClass,
        DeclareTransactionResult, DeployAccountTransaction, DeployAccountTransactionResult,
        DeployTransactionResult, EventsPage, FeeEstimate, FunctionCall, InvokeTransactionResult,
        MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, MaybePendingTransactionReceipt,
        StateUpdate, SyncStatusType, Transaction as StarknetTransaction,
    },
};
use std::net::SocketAddr;
use std::str::FromStr;

pub struct BeerusRpc {
    beerus: BeerusLightClient,
}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { beerus }
    }

    pub async fn run(self) -> Result<(SocketAddr, ServerHandle), Error> {
        let server = ServerBuilder::new()
            .build(self.beerus.config.beerus_rpc_address.unwrap())
            .await
            .map_err(|_| Error::from(BeerusApiError::from(INTERNAL_SERVER_ERROR)))?;

        let addr = server.local_addr()?;
        let handle = server.start(self.into_rpc())?;
        Ok((addr, handle))
    }
}

#[async_trait]
impl BeerusRpcServer for BeerusRpc {
    // Ethereum methods
    async fn eth_get_balance(&self, address: &str, block: BlockTag) -> Result<String, Error> {
        let address = Address::from_str(address)
            .map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?;
        let balance = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_balance(&address, block)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;

        Ok(hex_string!(balance))
    }

    async fn eth_get_transaction_count(
        &self,
        address: &str,
        block: BlockTag,
    ) -> Result<String, Error> {
        let address = parse_eth_address(address)
            .map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?;

        let tx_count = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_transaction_count(&address, block)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;

        Ok(hex_string!(tx_count))
    }

    async fn eth_get_block_transaction_count_by_hash(&self, hash: &str) -> Result<String, Error> {
        let hash = parse_eth_hash(hash)
            .map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?;
        let tx_count = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_block_transaction_count_by_hash(&hash)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;

        Ok(hex_string!(tx_count))
    }

    async fn eth_get_block_transaction_count_by_number(
        &self,
        block: BlockTag,
    ) -> Result<String, Error> {
        let tx_count = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_block_transaction_count_by_number(block)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;

        Ok(hex_string!(tx_count))
    }

    async fn eth_get_code(&self, address: &str, block: BlockTag) -> Result<String, Error> {
        let address = parse_eth_address(address)
            .map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?;
        let code = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_code(&address, block)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;
        Ok(format!("0x{}", hex::encode(code)))
    }

    async fn eth_call(&self, opts: CallOpts, block: BlockTag) -> Result<String, Error> {
        let res = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .call(&opts, block)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;
        Ok(format!("0x{}", hex::encode(res)))
    }

    async fn eth_estimate_gas(&self, opts: CallOpts) -> Result<String, Error> {
        let gas_estimation = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .estimate_gas(&opts)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;

        Ok(hex_string!(gas_estimation))
    }

    async fn eth_chain_id(&self) -> Result<String, Error> {
        let chain_id = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_chain_id()
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;

        Ok(hex_string!(chain_id))
    }

    async fn eth_gas_price(&self) -> Result<String, Error> {
        let gas_price = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_gas_price()
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;

        Ok(hex_string!(gas_price))
    }

    async fn eth_max_priority_fee_per_gas(&self) -> Result<String, Error> {
        let max_priority_fee_per_gas = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_priority_fee()
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;

        Ok(hex_string!(max_priority_fee_per_gas))
    }

    async fn eth_block_number(&self) -> Result<String, Error> {
        let block_number = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_block_number()
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;

        Ok(hex_string!(block_number))
    }

    async fn eth_get_block_by_number(
        &self,
        block: BlockTag,
        full_tx: bool,
    ) -> Result<Option<ExecutionBlock>, Error> {
        self.beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_block_by_number(block, full_tx)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))
    }

    async fn eth_get_block_by_hash(
        &self,
        hash: &str,
        full_tx: bool,
    ) -> Result<Option<ExecutionBlock>, Error> {
        let hash = parse_eth_hash(hash)
            .map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?;
        self.beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_block_by_hash(&hash, full_tx)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))
    }

    async fn eth_send_raw_transaction(&self, bytes: &str) -> Result<String, Error> {
        let bytes = parse_eth_hash(bytes)
            .map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?;
        let raw_tx = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .send_raw_transaction(&bytes)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;

        Ok(raw_tx.to_string())
    }

    async fn eth_get_transaction_receipt(
        &self,
        tx_hash: &str,
    ) -> Result<Option<TransactionReceipt>, Error> {
        let tx_hash = H256::from_str(tx_hash)
            .map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?;
        self.beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_transaction_receipt(&tx_hash)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))
    }

    async fn eth_get_transaction_by_hash(
        &self,
        hash: &str,
    ) -> Result<Option<EthTransaction>, Error> {
        let tx_hash = H256::from_str(hash)
            .map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?;
        self.beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_transaction_by_hash(&tx_hash)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))
    }

    async fn eth_get_transaction_by_block_hash_and_index(
        &self,
        hash: &str,
        index: usize,
    ) -> Result<Option<EthTransaction>, Error> {
        let block_hash = parse_eth_hash(hash)
            .map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?;
        self.beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_transaction_by_block_hash_and_index(&block_hash, index)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))
    }

    async fn eth_coinbase(&self) -> Result<Address, Error> {
        self.beerus
            .ethereum_lightclient
            .lock()
            .await
            .coinbase()
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))
    }

    async fn eth_syncing(&self) -> Result<SyncingStatus, Error> {
        self.beerus
            .ethereum_lightclient
            .lock()
            .await
            .syncing()
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))
    }

    async fn eth_get_logs(&self, filter: Filter) -> Result<Vec<Log>, Error> {
        self.beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_logs(&filter)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))
    }

    async fn eth_get_storage_at(
        &self,
        address: &str,
        slot: H256,
        block: BlockTag,
    ) -> Result<String, Error> {
        let address = parse_eth_address(address)
            .map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?;
        let storage = self
            .beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_storage_at(&address, slot, block)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))?;
        Ok(storage.to_string())
    }

    // Starknet methods
    async fn starknet_l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256, Error> {
        self.beerus
            .starknet_l2_to_l1_messages(msg_hash)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_chain_id(&self) -> Result<String, Error> {
        let chain_id = self
            .beerus
            .starknet_lightclient
            .chain_id()
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))?;

        Ok(chain_id.to_string())
    }

    async fn starknet_block_number(&self) -> Result<u64, Error> {
        self.beerus
            .starknet_lightclient
            .block_number()
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_nonce(
        &self,
        contract_address: String,
        block_id: BlockId,
    ) -> Result<String, Error> {
        let contract_address = FieldElement::from_hex_be(&contract_address)
            .map_err(|_| invalid_call_data("contract_address"))?;

        let nonce = self
            .beerus
            .starknet_get_nonce(contract_address, &block_id)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))?;

        Ok(nonce.to_string())
    }

    async fn starknet_get_transaction_by_hash(
        &self,
        tx_hash: &str,
    ) -> Result<StarknetTransaction, Error> {
        let tx_hash_felt =
            FieldElement::from_hex_be(tx_hash).map_err(|_| invalid_call_data("tx_hash_felt"))?;

        self.beerus
            .starknet_lightclient
            .get_transaction_by_hash(tx_hash_felt)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_block_transaction_count(&self, block_id: BlockId) -> Result<u64, Error> {
        self.beerus
            .starknet_lightclient
            .get_block_transaction_count(&block_id)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_block_hash_and_number(&self) -> Result<BlockHashAndNumber, Error> {
        self.beerus
            .starknet_lightclient
            .block_hash_and_number()
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_contract_storage_proof(
        &self,
        block_id: BlockId,
        contract_address: String,
        keys: Vec<String>,
    ) -> Result<GetProofOutput, Error> {
        let contract_address = FieldElement::from_str(&contract_address)
            .map_err(|_| invalid_call_data("contract_address"))?;

        let keys: Result<Vec<FieldElement>, _> =
            keys.iter().map(|k| FieldElement::from_str(k)).collect();

        self.beerus
            .starknet_lightclient
            .get_contract_storage_proof(
                contract_address,
                keys.map_err(|_| Error::from(BeerusApiError::from(INVALID_CALL_DATA)))?,
                &block_id,
            )
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_class_at(
        &self,
        block_id: BlockId,
        contract_address: String,
    ) -> Result<ContractClass, Error> {
        let contract_address = FieldElement::from_str(&contract_address)
            .map_err(|_| invalid_call_data("contract_address"))?;

        self.beerus
            .starknet_lightclient
            .get_class_at(&block_id, contract_address)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, Error> {
        self.beerus
            .starknet_lightclient
            .add_invoke_transaction(&invoke_transaction)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, Error> {
        self.beerus
            .starknet_lightclient
            .get_block_with_tx_hashes(&block_id)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: &str,
    ) -> Result<StarknetTransaction, Error> {
        let index = u64::from_str(index).map_err(|_| invalid_call_data("index"))?;

        self.beerus
            .starknet_lightclient
            .get_transaction_by_block_id_and_index(&block_id, index)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_block_with_txs(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxs, Error> {
        self.beerus
            .starknet_lightclient
            .get_block_with_txs(&block_id)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_state_update(&self, block_id: BlockId) -> Result<StateUpdate, Error> {
        self.beerus
            .starknet_lightclient
            .get_state_update(&block_id)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_syncing(&self) -> Result<SyncStatusType, Error> {
        self.beerus
            .starknet_lightclient
            .syncing()
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256, Error> {
        self.beerus
            .starknet_l1_to_l2_messages(msg_hash)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256, Error> {
        self.beerus
            .starknet_l1_to_l2_message_nonce()
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256, Error> {
        self.beerus
            .starknet_l1_to_l2_message_cancellations(msg_hash)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<MaybePendingTransactionReceipt, Error> {
        let tx_hash_felt =
            FieldElement::from_hex_be(&tx_hash).map_err(|_| invalid_call_data("tx_hash_felt"))?;

        self.beerus
            .starknet_lightclient
            .get_transaction_receipt(tx_hash_felt)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: String,
    ) -> Result<FieldElement, Error> {
        let contract_address = FieldElement::from_str(&contract_address)
            .map_err(|_| invalid_call_data("contract_address"))?;

        self.beerus
            .starknet_lightclient
            .get_class_hash_at(&block_id, contract_address)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_class(
        &self,
        block_id: BlockId,
        class_hash: String,
    ) -> Result<ContractClass, Error> {
        let class_hash =
            FieldElement::from_str(&class_hash).map_err(|_| invalid_call_data("class_hash"))?;

        self.beerus
            .starknet_lightclient
            .get_class(&block_id, class_hash)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_add_deploy_account_transaction(
        &self,
        deploy_account_transaction: BroadcastedDeployAccountTransaction,
    ) -> Result<DeployAccountTransactionResult, Error> {
        self.beerus
            .starknet_lightclient
            .add_deploy_account_transaction(&deploy_account_transaction)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_events(
        &self,
        custom_filter: EventFilterWithPage,
    ) -> Result<EventsPage, Error> {
        self.beerus
            .starknet_lightclient
            .get_events(
                custom_filter.filter,
                custom_filter.page.continuation_token,
                custom_filter.page.chunk_size,
            )
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_add_declare_transaction(
        &self,
        declare_transaction: BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, Error> {
        self.beerus
            .starknet_lightclient
            .add_declare_transaction(&declare_transaction)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_pending_transactions(&self) -> Result<Vec<StarknetTransaction>, Error> {
        self.beerus
            .starknet_lightclient
            .pending_transactions()
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_estimate_fee(
        &self,
        block_id: BlockId,
        broadcasted_transaction: BroadcastedTransaction,
    ) -> Result<FeeEstimate, Error> {
        self.beerus
            .starknet_lightclient
            .estimate_fee(broadcasted_transaction, &block_id)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_call(
        &self,
        request: FunctionCall,
        block_id: BlockId,
    ) -> Result<Vec<FieldElement>, Error> {
        self.beerus
            .starknet_lightclient
            .call(request, &block_id)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }

    async fn starknet_get_storage_at(
        &self,
        contract_address: String,
        key: String,
        block_id: BlockId,
    ) -> Result<FieldElement, Error> {
        let contract_address = FieldElement::from_hex_be(&contract_address)
            .map_err(|_| invalid_call_data("contract_address"))?;

        let key = FieldElement::from_hex_be(&key).map_err(|_| invalid_call_data("key"))?;

        self.beerus
            .starknet_get_storage_at(contract_address, key, &block_id)
            .await
            .map_err(|e| Error::from(BeerusApiError::from(e)))
    }
}
