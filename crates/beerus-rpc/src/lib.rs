pub mod api;
pub mod utils;

use crate::api::{BeerusApiError, BeerusRpcServer};
use beerus_core::{
    ethers_helper::{parse_eth_address, parse_eth_hash},
    lightclient::starknet::storage_proof::GetProofOutput,
};

use helios::types::{BlockTag, CallOpts, ExecutionBlock};
use jsonrpsee::{
    core::{async_trait, Error},
    server::{ServerBuilder, ServerHandle},
    types::error::CallError,
};

use beerus_core::lightclient::beerus::BeerusLightClient;
use ethers::types::{
    Address, Filter, Log, SyncingStatus, Transaction as EthTransaction, TransactionReceipt, H256,
    U256,
};
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::models::{
        BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
        BroadcastedDeclareTransactionV1, BroadcastedDeployTransaction,
        BroadcastedInvokeTransaction, BroadcastedTransaction, ContractClass,
        DeclareTransactionResult, DeployTransactionResult, EventFilter, EventsPage, FeeEstimate,
        FunctionCall, InvokeTransactionResult, MaybePendingBlockWithTxHashes,
        MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, StateUpdate, SyncStatusType,
        Transaction as StarknetTransaction,
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
            .map_err(|_| Error::from(BeerusApiError::InternalServerError))?;

        let addr = server.local_addr()?;
        let handle = server.start(self.into_rpc())?;
        Ok((addr, handle))
    }
}

#[async_trait]
impl BeerusRpcServer for BeerusRpc {
    // Ethereum methods
    async fn eth_get_balance(&self, address: &str, block: BlockTag) -> Result<String, Error> {
        let address =
            Address::from_str(address).map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
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
        let address =
            parse_eth_address(address).map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;

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
        let hash =
            parse_eth_hash(hash).map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
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
        let address =
            parse_eth_address(address).map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
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
        let hash =
            parse_eth_hash(hash).map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
        self.beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_block_by_hash(&hash, full_tx)
            .await
            .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))
    }

    async fn eth_send_raw_transaction(&self, bytes: &str) -> Result<String, Error> {
        let bytes =
            parse_eth_hash(bytes).map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
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
        let tx_hash =
            H256::from_str(tx_hash).map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
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
        let tx_hash =
            H256::from_str(hash).map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
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
        let block_hash =
            parse_eth_hash(hash).map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
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
        let address =
            parse_eth_address(address).map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
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
        Ok(self
            .beerus
            .starknet_l2_to_l1_messages(msg_hash)
            .await
            .unwrap())
    }

    async fn starknet_chain_id(&self) -> Result<String, Error> {
        let chain_id = self
            .beerus
            .starknet_lightclient
            .chain_id()
            .await
            .unwrap()
            .to_string();

        Ok(chain_id)
    }

    async fn starknet_block_number(&self) -> Result<u64, Error> {
        self.beerus
            .starknet_lightclient
            .block_number()
            .await
            .map_err(|_| Error::from(BeerusApiError::BlockNotFound))
    }

    async fn starknet_get_nonce(
        &self,
        contract_address: String,
        block_id: BlockId,
    ) -> Result<String, Error> {
        let contract_address = FieldElement::from_hex_be(&contract_address).unwrap();
        let nonce = self
            .beerus
            .starknet_get_nonce(contract_address, &block_id)
            .await
            .unwrap()
            .to_string();
        Ok(nonce)
    }

    async fn starknet_get_transaction_by_hash(
        &self,
        tx_hash: &str,
    ) -> Result<StarknetTransaction, Error> {
        let tx_hash_felt = FieldElement::from_hex_be(tx_hash)
            .map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
        self.beerus
            .starknet_lightclient
            .get_transaction_by_hash(tx_hash_felt)
            .await
            .map_err(|_| Error::from(BeerusApiError::TxnHashNotFound))
    }

    async fn starknet_get_block_transaction_count(&self, block_id: BlockId) -> Result<u64, Error> {
        let block_transaction_count = self
            .beerus
            .starknet_lightclient
            .get_block_transaction_count(&block_id)
            .await
            .unwrap();

        Ok(block_transaction_count)
    }

    async fn starknet_block_hash_and_number(&self) -> Result<BlockHashAndNumber, Error> {
        Ok(self
            .beerus
            .starknet_lightclient
            .block_hash_and_number()
            .await
            .unwrap())
    }

    async fn starknet_get_contract_storage_proof(
        &self,
        block_id: BlockId,
        contract_address: String,
        keys: Vec<String>,
    ) -> Result<GetProofOutput, Error> {
        let contract_address = FieldElement::from_str(&contract_address)
            .map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
        let keys: Result<Vec<FieldElement>, _> =
            keys.iter().map(|k| FieldElement::from_str(k)).collect();

        self.beerus
            .starknet_lightclient
            .get_contract_storage_proof(
                contract_address,
                keys.map_err(|_| Error::from(BeerusApiError::InvalidCallData))?,
                &block_id,
            )
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))
    }

    async fn starknet_get_class_at(
        &self,
        block_id: BlockId,
        contract_address: String,
    ) -> Result<ContractClass, Error> {
        let contract_address = FieldElement::from_str(&contract_address).unwrap();
        Ok(self
            .beerus
            .starknet_lightclient
            .get_class_at(&block_id, contract_address)
            .await
            .unwrap())
    }

    async fn starknet_add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, Error> {
        self.beerus
            .starknet_lightclient
            .add_invoke_transaction(&invoke_transaction)
            .await
            .map_err(|_| Error::from(BeerusApiError::InvalidCallData))
    }

    async fn starknet_get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, Error> {
        self.beerus
            .starknet_lightclient
            .get_block_with_tx_hashes(&block_id)
            .await
            .map_err(|_| Error::from(BeerusApiError::BlockNotFound))
    }

    async fn starknet_get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: &str,
    ) -> Result<StarknetTransaction, Error> {
        let index = u64::from_str(index)
            .map_err(|e| Error::Call(CallError::InvalidParams(anyhow::anyhow!(e.to_string()))))?;
        let result = self
            .beerus
            .starknet_lightclient
            .get_transaction_by_block_id_and_index(&block_id, index)
            .await
            .map_err(|e| Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string()))))?;
        Ok(result)
    }

    async fn starknet_get_block_with_txs(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxs, Error> {
        let result = self
            .beerus
            .starknet_lightclient
            .get_block_with_txs(&block_id)
            .await
            .map_err(|e| Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string()))))?;
        Ok(result)
    }

    async fn starknet_get_state_update(&self, block_id: BlockId) -> Result<StateUpdate, Error> {
        Ok(self
            .beerus
            .starknet_lightclient
            .get_state_update(&block_id)
            .await
            .unwrap())
    }

    async fn starknet_syncing(&self) -> Result<SyncStatusType, Error> {
        let sync_status_type = self.beerus.starknet_lightclient.syncing().await.unwrap();
        Ok(sync_status_type)
    }

    async fn starknet_l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256, Error> {
        Ok(self
            .beerus
            .starknet_l1_to_l2_messages(msg_hash)
            .await
            .unwrap())
    }

    async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256, Error> {
        let nonce = self.beerus.starknet_l1_to_l2_message_nonce().await.unwrap();
        Ok(nonce)
    }

    async fn starknet_l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256, Error> {
        Ok(self
            .beerus
            .starknet_l1_to_l2_message_cancellations(msg_hash)
            .await
            .unwrap())
    }

    async fn starknet_get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<MaybePendingTransactionReceipt, Error> {
        let tx_hash_felt = FieldElement::from_hex_be(&tx_hash).unwrap();
        Ok(self
            .beerus
            .starknet_lightclient
            .get_transaction_receipt(tx_hash_felt)
            .await
            .unwrap())
    }

    async fn starknet_get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: String,
    ) -> Result<FieldElement, Error> {
        let contract_address = FieldElement::from_str(&contract_address).unwrap();

        Ok(self
            .beerus
            .starknet_lightclient
            .get_class_hash_at(&block_id, contract_address)
            .await
            .unwrap())
    }

    async fn starknet_get_class(
        &self,
        block_id: BlockId,
        class_hash: String,
    ) -> Result<ContractClass, Error> {
        let class_hash = FieldElement::from_str(&class_hash)
            .map_err(|e| Error::Call(CallError::InvalidParams(anyhow::anyhow!(e.to_string()))))?;
        let result = self
            .beerus
            .starknet_lightclient
            .get_class(&block_id, class_hash)
            .await
            .map_err(|e| Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string()))))?;

        Ok(result)
    }

    async fn starknet_add_deploy_account_transaction(
        &self,
        contract_class: String,
        version: String,
        contract_address_salt: String,
        constructor_calldata: Vec<String>,
    ) -> Result<DeployTransactionResult, Error> {
        let contract_class_bytes = contract_class.as_bytes();
        let contract_class = serde_json::from_slice(contract_class_bytes).unwrap();
        let version: u64 = version.parse().unwrap();
        let contract_address_salt: FieldElement =
            FieldElement::from_str(&contract_address_salt).unwrap();
        let constructor_calldata = constructor_calldata
            .iter()
            .map(|x| FieldElement::from_str(x).unwrap())
            .collect();
        let deploy_transaction = BroadcastedDeployTransaction {
            contract_class,
            version,
            contract_address_salt,
            constructor_calldata,
        };
        let result = self
            .beerus
            .starknet_lightclient
            .add_deploy_transaction(&deploy_transaction)
            .await
            .map_err(|e| Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string()))))?;

        Ok(result)
    }

    async fn starknet_get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, Error> {
        Ok(self
            .beerus
            .starknet_lightclient
            .get_events(filter, continuation_token, chunk_size)
            .await
            .unwrap())
    }

    async fn starknet_add_declare_transaction(
        &self,
        version: String,
        max_fee: String,
        signature: Vec<String>,
        nonce: String,
        contract_class: String,
        sender_address: String,
        compile_class_hash: String,
    ) -> Result<DeclareTransactionResult, Error> {
        let max_fee: FieldElement = FieldElement::from_str(&max_fee).unwrap();
        let _version: u64 = version.parse().unwrap();
        let signature = signature
            .iter()
            .map(|x| FieldElement::from_str(x).unwrap())
            .collect();
        let nonce: FieldElement = FieldElement::from_str(&nonce).unwrap();
        let _compiled_class_hash = compile_class_hash;

        let contract_class_bytes = contract_class.as_bytes();
        let contract_class = serde_json::from_slice(contract_class_bytes)?;
        let sender_address: FieldElement = FieldElement::from_str(&sender_address).unwrap();

        let declare_transaction =
            BroadcastedDeclareTransaction::V1(BroadcastedDeclareTransactionV1 {
                max_fee,
                signature,
                nonce,
                contract_class,
                sender_address,
            });

        Ok(self
            .beerus
            .starknet_lightclient
            .add_declare_transaction(&declare_transaction)
            .await
            .unwrap())
    }

    async fn starknet_pending_transactions(&self) -> Result<Vec<StarknetTransaction>, Error> {
        let transactions_result = self
            .beerus
            .starknet_lightclient
            .pending_transactions()
            .await
            .map_err(|_| Error::from(BeerusApiError::FailedToFetchPendingTransactions));
        Ok(transactions_result.unwrap())
    }

    async fn starknet_estimate_fee(
        &self,
        block_id: BlockId,
        broadcasted_transaction: String,
    ) -> Result<FeeEstimate, Error> {
        let broadcasted_transaction: BroadcastedTransaction =
            serde_json::from_str(&broadcasted_transaction).map_err(|e| {
                Error::Call(CallError::InvalidParams(anyhow::anyhow!(e.to_string())))
            })?;

        let estimate_fee = self
            .beerus
            .starknet_lightclient
            .estimate_fee(broadcasted_transaction, &block_id)
            .await
            .map_err(|e| Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string()))))?;
        Ok(estimate_fee)
    }

    async fn starknet_call(
        &self,
        request: FunctionCall,
        block_number: u64,
    ) -> Result<Vec<FieldElement>, Error> {
        self.beerus
            .starknet_lightclient
            .call(request, block_number)
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))
    }

    async fn starknet_get_storage_at(
        &self,
        contract_address: String,
        key: String,
    ) -> Result<FieldElement, Error> {
        let contract_address = FieldElement::from_hex_be(&contract_address)
            .map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;
        let key = FieldElement::from_hex_be(&key)
            .map_err(|_| Error::from(BeerusApiError::InvalidCallData))?;

        self.beerus
            .starknet_get_storage_at(contract_address, key)
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))
    }
}
