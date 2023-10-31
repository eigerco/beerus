use crate::config::Config;
use crate::client::BeerusClient;
use starknet::core::types::{
    BlockHashAndNumber, BlockId, BlockTag as SnBlockTag, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction, ContractClass,
    DeclareTransactionResult, DeployAccountTransactionResult, EventFilter, EventsPage, FeeEstimate, FieldElement,
    FunctionCall, InvokeTransactionResult, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
    MaybePendingStateUpdate, MaybePendingTransactionReceipt, MsgFromL1, SyncStatusType, Transaction,
};
use crate::utils::get_balance_key;
use jsonrpsee::core::async_trait;
use starknet::macros::selector;
use starknet::providers::jsonrpc::{HttpTransport, HttpTransportError, JsonRpcClient, JsonRpcClientError};
use starknet::providers::ProviderError::StarknetError;
use starknet::providers::{AnyProviderError, Provider, ProviderError, StarknetErrorWithMessage};
use crate::storage_proofs::types::StorageProofResponse;
use crate::storage_proofs::StorageProof;
use crate::CoreError;

pub struct Beerus {
    core: BeerusClient,
}

impl Beerus {
    pub async fn new(config: Config) -> Self {
        Self { core: BeerusClient::new(config).await }
    }
}
type StarknetErr = ProviderError<JsonRpcClientError<HttpTransportError>>;

impl Beerus {
    pub async fn get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.get_block_with_tx_hashes(l1_block_num).await
    }

    pub async fn get_block_with_txs(&self, block_id: BlockId) -> Result<MaybePendingBlockWithTxs, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.get_block_with_txs(l1_block_num).await
    }

    pub async fn get_state_update(&self, block_id: BlockId) -> Result<MaybePendingStateUpdate, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.get_state_update(l1_block_num).await
    }

    pub async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_id: BlockId,
    ) -> Result<FieldElement, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        let fetched_val = self.core.starknet_client.get_storage_at(contract_address, key, l1_block_num).await?;
        let mut proof = self
            .core
            .get_contract_storage_proof(block_id, contract_address.as_ref(), &[*key.as_ref()])
            .await
            .unwrap();

        let l1_root = self.core.get_local_root().await;
        proof.verify(l1_root, *contract_address.as_ref(), *key.as_ref(), fetched_val).unwrap();

        Ok(fetched_val)
    }

    pub async fn get_transaction_by_hash(&self, transaction_hash: FieldElement) -> Result<Transaction, StarknetErr> {
        self.core.starknet_client.get_transaction_by_hash(transaction_hash).await
    }

    pub async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: u64,
    ) -> Result<Transaction, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core
            .starknet_client
            .get_transaction_by_block_id_and_index(l1_block_num, index)
            .await
            
    }

    pub async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt, StarknetErr> {
        self.core.starknet_client.get_transaction_receipt(transaction_hash).await
    }

    pub async fn get_class(&self, block_id: BlockId, class_hash: FieldElement) -> Result<ContractClass, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.get_class(l1_block_num, class_hash).await
    }

    pub async fn get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core
            .starknet_client
            .get_class_hash_at(l1_block_num, contract_address)
            .await
            
    }

    pub async fn get_class_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.get_class_at(l1_block_num, contract_address).await
    }

    pub async fn get_block_transaction_count(&self, block_id: BlockId) -> Result<u64, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.get_block_transaction_count(l1_block_num).await
    }

    pub async fn call(&self, request: FunctionCall, block_id: BlockId) -> Result<Vec<FieldElement>, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.call(request, l1_block_num).await
    }

    pub async fn estimate_fee(
        &self,
        request: BroadcastedTransaction,
        block_id: BlockId,
    ) -> Result<Vec<FeeEstimate>, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.estimate_fee(vec![request], l1_block_num).await
    }

    pub async fn estimate_message_fee(&self, message: MsgFromL1, block_id: BlockId) -> Result<FeeEstimate, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.estimate_message_fee(message, l1_block_num).await
    }

    pub async fn block_number(&self) -> Result<u64, StarknetErr> {
        Ok(self.core.get_local_block_num().await)
    }

    pub async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, StarknetErr> {
        let block_hash = self.core.sn_state_block_hash().await?;
        let block_number = self.core.sn_state_block_number().await?;
        Ok(BlockHashAndNumber { block_hash, block_number })
    }

    pub async fn chain_id(&self) -> Result<FieldElement, StarknetErr> {
        self.core.starknet_client.chain_id().await
    }

    pub async fn pending_transactions(&self) -> Result<Vec<Transaction>, StarknetErr> {
        self.core.starknet_client.pending_transactions().await
    }

    pub async fn syncing(&self) -> Result<SyncStatusType, StarknetErr> {
        self.core.starknet_client.syncing().await
    }

    pub async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, StarknetErr> {
        self.core
            .starknet_client
            .get_events(filter, continuation_token, chunk_size)
            .await
            
    }

    pub async fn get_nonce(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.get_nonce(l1_block_num, contract_address).await
    }

    pub async fn add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, StarknetErr> {
        self.core.starknet_client.add_invoke_transaction(invoke_transaction).await
    }

    pub async fn add_declare_transaction(
        &self,
        declare_transaction: BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, StarknetErr> {
        self.core.starknet_client.add_declare_transaction(declare_transaction).await
    }

    pub async fn add_deploy_account_transaction(
        &self,
        deploy_account_transaction: BroadcastedDeployAccountTransaction,
    ) -> Result<DeployAccountTransactionResult, StarknetErr> {
        self.core
            .starknet_client
            .add_deploy_account_transaction(deploy_account_transaction)
            .await
            
    }

    pub async fn estimate_fee_single(
        &self,
        request: BroadcastedTransaction,
        block_id: BlockId,
    ) -> Result<FeeEstimate, StarknetErr> {
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        self.core.starknet_client.estimate_fee_single(request, l1_block_num).await
    }

    // ------------------- Extended Starknet Provider Endpoints -------------------
    //
    pub async fn get_contract_storage_proof(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
        keys: Vec<FieldElement>,
    ) -> Result<StorageProof, CoreError> {
        self.core.get_contract_storage_proof(block_id, &contract_address, &keys).await
    }

    pub async fn proven_state_root(&self) -> Result<FieldElement, CoreError> {
        self.core.sn_state_root().await
    }

    pub async fn proven_block_number(&self) -> Result<u64, CoreError> {
        self.core.sn_state_block_number().await
    }

    pub async fn get_balance(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, StarknetErr> {
        // get local block number and root to verify proof with
        let l1_block_num = self.core.get_local_block_id(block_id).await;
        let root = self.core.get_local_root().await;

        // get the storage key for the queried contract address
        let balance_key = get_balance_key(contract_address);

        // get the proof for the contracts erc20 balance in the fee token contract
        let mut proof = self
            .core
            .get_contract_storage_proof(block_id, &self.core.config.fee_token_addr, &[balance_key])
            .await
            ?;

        // call the untrusted RPC for the value to check via the storage proof
        let balance = self
            .call(
                FunctionCall {
                    contract_address: self.core.config.fee_token_addr,
                    entry_point_selector: selector!("balanceOf"),
                    calldata: vec![contract_address],
                },
                l1_block_num,
            )
            .await?;

        // verify the storage proof w/ the untrusted value
        proof.verify(root, self.core.config.fee_token_addr, balance_key, balance[0])?;

        Ok(balance[0])
    }
}
