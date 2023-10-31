use beerus_core::storage_proofs::StorageProof;
use beerus_core::utils::get_balance_key;
use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use starknet::core::types::{
    BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction, BroadcastedDeployAccountTransaction,
    BroadcastedInvokeTransaction, BroadcastedTransaction, ContractClass, DeclareTransactionResult,
    DeployAccountTransactionResult, EventFilter, EventsPage, FeeEstimate, FieldElement, FunctionCall,
    InvokeTransactionResult, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, MaybePendingStateUpdate,
    MaybePendingTransactionReceipt, MsgFromL1, SyncStatusType, Transaction,
};
use starknet::macros::selector;
use starknet::providers::Provider;

use crate::error::BeerusRpcError;
use crate::BeerusRpc;

#[rpc(server, namespace = "starknet")]
pub trait BeerusRpc {
    // ------------------- Starknet Provider Endpoints -------------------
    //
    #[method(name = "getBlockWithTxHashes")]
    async fn get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, BeerusRpcError>;

    #[method(name = "getBlockWithTxs")]
    async fn get_block_with_txs(&self, block_id: BlockId) -> Result<MaybePendingBlockWithTxs, BeerusRpcError>;

    #[method(name = "getStateUpdate")]
    async fn get_state_update(&self, block_id: BlockId) -> Result<MaybePendingStateUpdate, BeerusRpcError>;

    #[method(name = "getStorageAt")]
    async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_id: BlockId,
    ) -> Result<FieldElement, BeerusRpcError>;

    #[method(name = "getTransactionByHash")]
    async fn get_transaction_by_hash(&self, transaction_hash: FieldElement) -> Result<Transaction, BeerusRpcError>;

    #[method(name = "getTransactionByBlockIdAndIndex")]
    async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: u64,
    ) -> Result<Transaction, BeerusRpcError>;

    #[method(name = "getTransactionReceipt")]
    async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt, BeerusRpcError>;

    #[method(name = "getClass")]
    async fn get_class(&self, block_id: BlockId, class_hash: FieldElement) -> Result<ContractClass, BeerusRpcError>;

    #[method(name = "getClassHashAt")]
    async fn get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, BeerusRpcError>;

    #[method(name = "getClassAt")]
    async fn get_class_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass, BeerusRpcError>;

    #[method(name = "getBlockTransactionCount")]
    async fn get_block_transaction_count(&self, block_id: BlockId) -> Result<u64, BeerusRpcError>;

    #[method(name = "call")]
    async fn call(&self, request: FunctionCall, block_id: BlockId) -> Result<Vec<FieldElement>, BeerusRpcError>;

    #[method(name = "estimateFee")]
    async fn estimate_fee(
        &self,
        request: BroadcastedTransaction,
        block_id: BlockId,
    ) -> Result<Vec<FeeEstimate>, BeerusRpcError>;

    #[method(name = "estimateMessageFee")]
    async fn estimate_message_fee(&self, message: MsgFromL1, block_id: BlockId) -> Result<FeeEstimate, BeerusRpcError>;

    #[method(name = "blockNumber")]
    async fn block_number(&self) -> Result<u64, BeerusRpcError>;

    #[method(name = "blockHashAndNumber")]
    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, BeerusRpcError>;

    #[method(name = "chainId")]
    async fn chain_id(&self) -> Result<FieldElement, BeerusRpcError>;

    #[method(name = "pendingTransactions")]
    async fn pending_transactions(&self) -> Result<Vec<Transaction>, BeerusRpcError>;

    #[method(name = "syncing")]
    async fn syncing(&self) -> Result<SyncStatusType, BeerusRpcError>;

    #[method(name = "getEvents")]
    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, BeerusRpcError>;

    #[method(name = "getNonce")]
    async fn get_nonce(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, BeerusRpcError>;

    #[method(name = "addInvokeTransaction")]
    async fn add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, BeerusRpcError>;

    #[method(name = "addDeclareTransaction")]
    async fn add_declare_transaction(
        &self,
        declare_transaction: BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, BeerusRpcError>;

    #[method(name = "addDeployAccountTransaction")]
    async fn add_deploy_account_transaction(
        &self,
        deploy_account_transaction: BroadcastedDeployAccountTransaction,
    ) -> Result<DeployAccountTransactionResult, BeerusRpcError>;

    #[method(name = "estimateFeeSingle")]
    async fn estimate_fee_single(
        &self,
        request: BroadcastedTransaction,
        block_id: BlockId,
    ) -> Result<FeeEstimate, BeerusRpcError>;

    // ------------------- Extended Starknet Provider Endpoints -------------------
    //
    #[method(name = "getContractStorageProof")]
    async fn get_contract_storage_proof(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
        keys: Vec<FieldElement>,
    ) -> Result<StorageProof, BeerusRpcError>;

    #[method(name = "provenStateRoot")]
    async fn proven_state_root(&self) -> Result<FieldElement, BeerusRpcError>;

    #[method(name = "provenBlockNumber")]
    async fn proven_block_number(&self) -> Result<u64, BeerusRpcError>;

    #[method(name = "getBalance")]
    async fn get_balance(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, BeerusRpcError>;
}

#[async_trait]
impl BeerusRpcServer for BeerusRpc {
    // ------------------- Starknet Provider Endpoints -------------------
    //
    async fn get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, BeerusRpcError> {
        self.beerus.get_block_with_tx_hashes(block_id).await.map_err(BeerusRpcError::from)
    }

    async fn get_block_with_txs(&self, block_id: BlockId) -> Result<MaybePendingBlockWithTxs, BeerusRpcError> {
        self.beerus.get_block_with_txs(block_id).await.map_err(BeerusRpcError::from)
    }

    async fn get_state_update(&self, block_id: BlockId) -> Result<MaybePendingStateUpdate, BeerusRpcError> {
        self.beerus.get_state_update(block_id).await.map_err(BeerusRpcError::from)
    }

    async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_id: BlockId,
    ) -> Result<FieldElement, BeerusRpcError> {
        self.beerus.get_storage_at(contract_address, key, block_id).await.map_err(BeerusRpcError::from)
    }

    async fn get_transaction_by_hash(&self, transaction_hash: FieldElement) -> Result<Transaction, BeerusRpcError> {
        self.beerus.get_transaction_by_hash(transaction_hash).await.map_err(BeerusRpcError::from)
    }

    async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: u64,
    ) -> Result<Transaction, BeerusRpcError> {
        self.beerus.get_transaction_by_block_id_and_index(block_id, index).await.map_err(BeerusRpcError::from)
    }

    async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt, BeerusRpcError> {
        self.beerus.get_transaction_receipt(transaction_hash).await.map_err(BeerusRpcError::from)
    }

    async fn get_class(&self, block_id: BlockId, class_hash: FieldElement) -> Result<ContractClass, BeerusRpcError> {
        self.beerus.get_class(block_id, class_hash).await.map_err(BeerusRpcError::from)
    }

    async fn get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, BeerusRpcError> {
        self.beerus.get_class_hash_at(block_id, contract_address).await.map_err(BeerusRpcError::from)
    }

    async fn get_class_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass, BeerusRpcError> {
        self.beerus.get_class_at(block_id, contract_address).await.map_err(BeerusRpcError::from)
    }

    async fn get_block_transaction_count(&self, block_id: BlockId) -> Result<u64, BeerusRpcError> {
        self.beerus.get_block_transaction_count(block_id).await.map_err(BeerusRpcError::from)
    }

    async fn call(&self, request: FunctionCall, block_id: BlockId) -> Result<Vec<FieldElement>, BeerusRpcError> {
        self.beerus.call(request, block_id).await.map_err(BeerusRpcError::from)
    }

    async fn estimate_fee(
        &self,
        request: BroadcastedTransaction,
        block_id: BlockId,
    ) -> Result<Vec<FeeEstimate>, BeerusRpcError> {
        self.beerus.estimate_fee(vec![request], block_id).await.map_err(BeerusRpcError::from)
    }

    async fn estimate_message_fee(&self, message: MsgFromL1, block_id: BlockId) -> Result<FeeEstimate, BeerusRpcError> {
        self.beerus.estimate_message_fee(message, block_id).await.map_err(BeerusRpcError::from)
    }

    async fn block_number(&self) -> Result<u64, BeerusRpcError> {
        Ok(self.beerus.get_local_block_num().await)
    }

    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, BeerusRpcError> {
        self.beerus.block_hash_and_number().await.map_err(BeerusRpcError::from)
    }

    async fn chain_id(&self) -> Result<FieldElement, BeerusRpcError> {
        self.beerus.chain_id().await.map_err(BeerusRpcError::from)
    }

    async fn pending_transactions(&self) -> Result<Vec<Transaction>, BeerusRpcError> {
        self.beerus.pending_transactions().await.map_err(BeerusRpcError::from)
    }

    async fn syncing(&self) -> Result<SyncStatusType, BeerusRpcError> {
        self.beerus.syncing().await.map_err(BeerusRpcError::from)
    }

    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, BeerusRpcError> {
        self.beerus.get_events(filter, continuation_token, chunk_size).await.map_err(BeerusRpcError::from)
    }

    async fn get_nonce(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, BeerusRpcError> {
        self.beerus.get_nonce(block_id, contract_address).await.map_err(BeerusRpcError::from)
    }

    async fn add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, BeerusRpcError> {
        self.beerus.add_invoke_transaction(invoke_transaction).await.map_err(BeerusRpcError::from)
    }

    async fn add_declare_transaction(
        &self,
        declare_transaction: BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, BeerusRpcError> {
        self.beerus.add_declare_transaction(declare_transaction).await.map_err(BeerusRpcError::from)
    }

    async fn add_deploy_account_transaction(
        &self,
        deploy_account_transaction: BroadcastedDeployAccountTransaction,
    ) -> Result<DeployAccountTransactionResult, BeerusRpcError> {
        self.beerus
            .starknet_add_deploy_account_transaction(deploy_account_transaction)
            .await
            .map_err(BeerusRpcError::from)
    }

    async fn estimate_fee_single(
        &self,
        request: BroadcastedTransaction,
        block_id: BlockId,
    ) -> Result<FeeEstimate, BeerusRpcError> {
        self.beerus.estimate_fee_single(request, block_id).await.map_err(BeerusRpcError::from)
    }

    // ------------------- Extended Starknet Provider Endpoints -------------------
    //
    async fn get_contract_storage_proof(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
        keys: Vec<FieldElement>,
    ) -> Result<StorageProof, BeerusRpcError> {
        self.beerus.get_contract_storage_proof(block_id, &contract_address, &keys).await.map_err(BeerusRpcError::from)
    }

    async fn proven_state_root(&self) -> Result<FieldElement, BeerusRpcError> {
        self.beerus.proven_state_root().await.map_err(BeerusRpcError::from)
    }

    async fn proven_block_number(&self) -> Result<u64, BeerusRpcError> {
        self.beerus.proven_block_number().await.map_err(BeerusRpcError::from)
    }

    async fn get_balance(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, BeerusRpcError> {
        // self.beerus.get_balance(block_id,
        // contract_address).await.map_err(BeerusRpcError::from)
    }
}
