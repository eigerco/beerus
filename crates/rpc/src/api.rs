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
        request: Vec<BroadcastedTransaction>,
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
    #[method(name = "getProof")]
    async fn get_proof(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
        keys: Vec<FieldElement>,
    ) -> Result<StorageProof, BeerusRpcError>;

    #[method(name = "getStateRoot")]
    async fn get_state_root(&self) -> Result<FieldElement, BeerusRpcError>;

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
    async fn get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.get_block_with_tx_hashes(l1_block_num).await.map_err(BeerusRpcError::from)
    }

    async fn get_block_with_txs(&self, block_id: BlockId) -> Result<MaybePendingBlockWithTxs, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.get_block_with_txs(l1_block_num).await.map_err(BeerusRpcError::from)
    }

    async fn get_state_update(&self, block_id: BlockId) -> Result<MaybePendingStateUpdate, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.get_state_update(l1_block_num).await.map_err(BeerusRpcError::from)
    }

    async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_id: BlockId,
    ) -> Result<FieldElement, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        let l1_root = self.beerus.get_local_root().await;

        let fetched_val = self.beerus.starknet_client.get_storage_at(contract_address, key, l1_block_num).await?;
        let mut proof = self.beerus.get_proof(l1_block_num, contract_address.as_ref(), &[*key.as_ref()]).await?;

        proof.verify(l1_root, *contract_address.as_ref(), *key.as_ref(), fetched_val)?;

        Ok(fetched_val)
    }

    async fn get_transaction_by_hash(&self, transaction_hash: FieldElement) -> Result<Transaction, BeerusRpcError> {
        self.beerus.starknet_client.get_transaction_by_hash(transaction_hash).await.map_err(BeerusRpcError::from)
    }

    async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: u64,
    ) -> Result<Transaction, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus
            .starknet_client
            .get_transaction_by_block_id_and_index(l1_block_num, index)
            .await
            .map_err(BeerusRpcError::from)
    }

    async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt, BeerusRpcError> {
        self.beerus.starknet_client.get_transaction_receipt(transaction_hash).await.map_err(BeerusRpcError::from)
    }

    async fn get_class(&self, block_id: BlockId, class_hash: FieldElement) -> Result<ContractClass, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.get_class(l1_block_num, class_hash).await.map_err(BeerusRpcError::from)
    }

    async fn get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus
            .starknet_client
            .get_class_hash_at(l1_block_num, contract_address)
            .await
            .map_err(BeerusRpcError::from)
    }

    async fn get_class_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.get_class_at(l1_block_num, contract_address).await.map_err(BeerusRpcError::from)
    }

    async fn get_block_transaction_count(&self, block_id: BlockId) -> Result<u64, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.get_block_transaction_count(l1_block_num).await.map_err(BeerusRpcError::from)
    }

    async fn call(&self, request: FunctionCall, block_id: BlockId) -> Result<Vec<FieldElement>, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.call(request, l1_block_num).await.map_err(BeerusRpcError::from)
    }

    async fn estimate_fee(
        &self,
        request: Vec<BroadcastedTransaction>,
        block_id: BlockId,
    ) -> Result<Vec<FeeEstimate>, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.estimate_fee(request, l1_block_num).await.map_err(BeerusRpcError::from)
    }

    async fn estimate_message_fee(&self, message: MsgFromL1, block_id: BlockId) -> Result<FeeEstimate, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.estimate_message_fee(message, l1_block_num).await.map_err(BeerusRpcError::from)
    }

    async fn block_number(&self) -> Result<u64, BeerusRpcError> {
        Ok(self.beerus.get_local_block_num().await)
    }

    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, BeerusRpcError> {
        let block_hash = self.beerus.sn_state_block_hash().await.map_err(BeerusRpcError::from)?;
        let block_number = self.beerus.sn_state_block_number().await.map_err(BeerusRpcError::from)?;
        Ok(BlockHashAndNumber { block_hash, block_number })
    }

    async fn chain_id(&self) -> Result<FieldElement, BeerusRpcError> {
        self.beerus.starknet_client.chain_id().await.map_err(BeerusRpcError::from)
    }

    async fn pending_transactions(&self) -> Result<Vec<Transaction>, BeerusRpcError> {
        self.beerus.starknet_client.pending_transactions().await.map_err(BeerusRpcError::from)
    }

    async fn syncing(&self) -> Result<SyncStatusType, BeerusRpcError> {
        self.beerus.starknet_client.syncing().await.map_err(BeerusRpcError::from)
    }

    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, BeerusRpcError> {
        self.beerus
            .starknet_client
            .get_events(filter, continuation_token, chunk_size)
            .await
            .map_err(BeerusRpcError::from)
    }

    async fn get_nonce(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.get_nonce(l1_block_num, contract_address).await.map_err(BeerusRpcError::from)
    }

    async fn add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, BeerusRpcError> {
        self.beerus.starknet_client.add_invoke_transaction(invoke_transaction).await.map_err(BeerusRpcError::from)
    }

    async fn add_declare_transaction(
        &self,
        declare_transaction: BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, BeerusRpcError> {
        self.beerus.starknet_client.add_declare_transaction(declare_transaction).await.map_err(BeerusRpcError::from)
    }

    async fn add_deploy_account_transaction(
        &self,
        deploy_account_transaction: BroadcastedDeployAccountTransaction,
    ) -> Result<DeployAccountTransactionResult, BeerusRpcError> {
        self.beerus
            .starknet_client
            .add_deploy_account_transaction(deploy_account_transaction)
            .await
            .map_err(BeerusRpcError::from)
    }

    async fn estimate_fee_single(
        &self,
        request: BroadcastedTransaction,
        block_id: BlockId,
    ) -> Result<FeeEstimate, BeerusRpcError> {
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        self.beerus.starknet_client.estimate_fee_single(request, l1_block_num).await.map_err(BeerusRpcError::from)
    }

    // ------------------- Extended Starknet Provider Endpoints -------------------
    async fn get_proof(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
        keys: Vec<FieldElement>,
    ) -> Result<StorageProof, BeerusRpcError> {
        self.beerus.get_proof(block_id, &contract_address, &keys).await.map_err(BeerusRpcError::from)
    }

    async fn get_state_root(&self) -> Result<FieldElement, BeerusRpcError> {
        self.beerus.sn_state_root().await.map_err(BeerusRpcError::from)
    }

    async fn get_balance(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, BeerusRpcError> {
        // get local block number and root to verify proof with
        let l1_block_num = self.beerus.get_local_block_id(block_id).await;
        let root = self.beerus.get_local_root().await;

        // get the storage key for the queried contract address
        let balance_key = get_balance_key(contract_address);

        // get the proof for the contracts erc20 balance in the fee token contract
        let mut proof = self
            .beerus
            .get_proof(block_id, &self.beerus.config.fee_token_addr, &[balance_key])
            .await
            .map_err(BeerusRpcError::from)?;

        // call the untrusted RPC for the value to check via the storage proof
        let balance = self
            .call(
                FunctionCall {
                    contract_address: self.beerus.config.fee_token_addr,
                    entry_point_selector: selector!("balanceOf"),
                    calldata: vec![contract_address],
                },
                l1_block_num,
            )
            .await?;

        // verify the storage proof w/ the untrusted value
        proof.verify(root, self.beerus.config.fee_token_addr, balance_key, balance[0]).map_err(BeerusRpcError::from)?;

        Ok(balance[0])
    }
}
