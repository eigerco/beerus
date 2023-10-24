use std::net::SocketAddr;

use beerus_core::client::BeerusClient;
use beerus_core::utils::felt_rs2path;

use jsonrpsee::core::{async_trait, Error};
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use starknet::core::types::{
    BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction, BroadcastedDeployAccountTransaction,
    BroadcastedInvokeTransaction, BroadcastedTransaction, ContractClass, DeclareTransactionResult,
    DeployAccountTransactionResult, EventFilter, EventsPage, FeeEstimate, FieldElement, FunctionCall,
    InvokeTransactionResult, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, MaybePendingStateUpdate,
    MaybePendingTransactionReceipt, MsgFromL1, SyncStatusType, Transaction,
};
use starknet::providers::jsonrpc::HttpTransportError;

use starknet::providers::{Provider, ProviderError};

pub struct BeerusRpc {
    beerus: BeerusClient,
    rpc_address: &'static str,
}

impl BeerusRpc {
    pub fn new(beerus: BeerusClient) -> Self {
        Self { beerus, rpc_address: "127.0.0.1:3030" }
    }

    pub async fn run(self) -> Result<(SocketAddr, ServerHandle), Error> {
        Ok((addr, handle))
    }
}

#[async_trait]
impl Provider for BeerusRpc {
    type Error = starknet::providers::jsonrpc::JsonRpcClientError<HttpTransportError>;

    async fn get_block_with_tx_hashes<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxHashes, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.get_block_with_tx_hashes(&l1_block_num).await
    }

    async fn get_block_with_txs<B>(&self, block_id: B) -> Result<MaybePendingBlockWithTxs, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.get_block_with_txs(&l1_block_num).await
    }

    async fn get_state_update<B>(&self, block_id: B) -> Result<MaybePendingStateUpdate, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.get_state_update(&l1_block_num).await
    }

    async fn get_storage_at<A, K, B>(
        &self,
        contract_address: A,
        key: K,
        block_id: B,
    ) -> Result<FieldElement, ProviderError<Self::Error>>
    where
        A: AsRef<FieldElement> + Send + Sync,
        K: AsRef<FieldElement> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        let fetched_val = self.beerus.starknet_client.get_storage_at(contract_address, key, &l1_block_num).await?;
        let proof =
            self.beerus.get_contract_storage_proof(contract_address.as_ref(), vec![*key.as_ref()]).await.unwrap();

        let l1_root = self.beerus.get_local_root().await;
        proof
            .verify(
                l1_root,
                felt_rs2path(*contract_address.as_ref()),
                felt_rs2path(*key.as_ref()),
                felt_rs2path(fetched_val),
            )
            .unwrap();

        Ok(fetched_val)
    }

    async fn get_transaction_by_hash<H>(&self, transaction_hash: H) -> Result<Transaction, ProviderError<Self::Error>>
    where
        H: AsRef<FieldElement> + Send + Sync,
    {
        self.beerus.starknet_client.get_transaction_by_hash(transaction_hash).await
    }

    async fn get_transaction_by_block_id_and_index<B>(
        &self,
        block_id: B,
        index: u64,
    ) -> Result<Transaction, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.get_transaction_by_block_id_and_index(&l1_block_num, index).await
    }

    async fn get_transaction_receipt<H>(
        &self,
        transaction_hash: H,
    ) -> Result<MaybePendingTransactionReceipt, ProviderError<Self::Error>>
    where
        H: AsRef<FieldElement> + Send + Sync,
    {
        self.beerus.starknet_client.get_transaction_receipt(transaction_hash).await
    }

    async fn get_class<B, H>(&self, block_id: B, class_hash: H) -> Result<ContractClass, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        H: AsRef<FieldElement> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.get_class(&l1_block_num, class_hash).await
    }

    async fn get_class_hash_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.get_class_hash_at(&l1_block_num, contract_address).await
    }

    async fn get_class_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<ContractClass, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.get_class_at(&l1_block_num, contract_address).await
    }

    async fn get_block_transaction_count<B>(&self, block_id: B) -> Result<u64, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.get_block_transaction_count(&l1_block_num).await
    }

    async fn call<R, B>(&self, request: R, block_id: B) -> Result<Vec<FieldElement>, ProviderError<Self::Error>>
    where
        R: AsRef<FunctionCall> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.call(request, &l1_block_num).await
    }

    async fn estimate_fee<R, B>(&self, request: R, block_id: B) -> Result<Vec<FeeEstimate>, ProviderError<Self::Error>>
    where
        R: AsRef<[BroadcastedTransaction]> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.estimate_fee(request, &l1_block_num).await
    }

    async fn estimate_message_fee<M, B>(
        &self,
        message: M,
        block_id: B,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>>
    where
        M: AsRef<MsgFromL1> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.estimate_message_fee(message, &l1_block_num).await
    }

    async fn block_number(&self) -> Result<u64, ProviderError<Self::Error>> {
        Ok(self.beerus.get_local_block_num().await)
    }

    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, ProviderError<Self::Error>> {
        Ok(BlockHashAndNumber { block_hash: FieldElement::ZERO, block_number: self.beerus.get_local_block_num().await })
    }

    async fn chain_id(&self) -> Result<FieldElement, ProviderError<Self::Error>> {
        self.beerus.starknet_client.chain_id().await
    }

    async fn pending_transactions(&self) -> Result<Vec<Transaction>, ProviderError<Self::Error>> {
        self.beerus.starknet_client.pending_transactions().await
    }

    async fn syncing(&self) -> Result<SyncStatusType, ProviderError<Self::Error>> {
        self.beerus.starknet_client.syncing().await
    }

    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, ProviderError<Self::Error>> {
        self.beerus.starknet_client.get_events(filter, continuation_token, chunk_size).await
    }

    async fn get_nonce<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.get_nonce(&l1_block_num, contract_address).await
    }

    async fn add_invoke_transaction<I>(
        &self,
        invoke_transaction: I,
    ) -> Result<InvokeTransactionResult, ProviderError<Self::Error>>
    where
        I: AsRef<BroadcastedInvokeTransaction> + Send + Sync,
    {
        self.beerus.starknet_client.add_invoke_transaction(invoke_transaction).await
    }

    async fn add_declare_transaction<D>(
        &self,
        declare_transaction: D,
    ) -> Result<DeclareTransactionResult, ProviderError<Self::Error>>
    where
        D: AsRef<BroadcastedDeclareTransaction> + Send + Sync,
    {
        self.beerus.starknet_client.add_declare_transaction(declare_transaction).await
    }

    async fn add_deploy_account_transaction<D>(
        &self,
        deploy_account_transaction: D,
    ) -> Result<DeployAccountTransactionResult, ProviderError<Self::Error>>
    where
        D: AsRef<BroadcastedDeployAccountTransaction> + Send + Sync,
    {
        self.beerus.starknet_client.add_deploy_account_transaction(deploy_account_transaction).await
    }

    async fn estimate_fee_single<R, B>(
        &self,
        request: R,
        block_id: B,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>>
    where
        R: AsRef<BroadcastedTransaction> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        let l1_block_num = self.beerus.get_local_block_id().await;
        self.beerus.starknet_client.estimate_fee_single(request, &l1_block_num).await
    }
}
