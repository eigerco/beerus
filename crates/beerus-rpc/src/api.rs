use crate::models::EventFilterWithPage;

use beerus_core::lightclient::starknet::storage_proof::GetProofOutput;
use helios::types::{BlockTag, CallOpts, ExecutionBlock};
use jsonrpsee::{core::Error, proc_macros::rpc};

use ethers::types::{
    Address, Filter, Log, SyncingStatus, Transaction as EthTransaction, TransactionReceipt, H256,
    U256,
};
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::models::{
        BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
        BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction,
        ContractClass, DeclareTransactionResult, DeployAccountTransactionResult, EventsPage,
        FeeEstimate, FunctionCall, InvokeTransactionResult, MaybePendingBlockWithTxHashes,
        MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, StateUpdate, SyncStatusType,
        Transaction as StarknetTransaction,
    },
};

#[rpc(server)]
pub trait BeerusRpc {
    // Ethereum endpoints
    #[method(name = "eth_getBalance")]
    async fn eth_get_balance(&self, address: &str, block: BlockTag) -> Result<String, Error>;

    #[method(name = "eth_getTransactionCount")]
    async fn eth_get_transaction_count(
        &self,
        address: &str,
        block: BlockTag,
    ) -> Result<String, Error>;

    #[method(name = "eth_getBlockTransactionCountByHash")]
    async fn eth_get_block_transaction_count_by_hash(&self, hash: &str) -> Result<String, Error>;

    #[method(name = "eth_getBlockTransactionCountByNumber")]
    async fn eth_get_block_transaction_count_by_number(
        &self,
        block: BlockTag,
    ) -> Result<String, Error>;

    #[method(name = "eth_getCode")]
    async fn eth_get_code(&self, address: &str, block: BlockTag) -> Result<String, Error>;

    #[method(name = "eth_call")]
    async fn eth_call(&self, opts: CallOpts, block: BlockTag) -> Result<String, Error>;

    #[method(name = "eth_estimateGas")]
    async fn eth_estimate_gas(&self, opts: CallOpts) -> Result<String, Error>;

    #[method(name = "eth_chainId")]
    async fn eth_chain_id(&self) -> Result<String, Error>;

    #[method(name = "eth_gasPrice")]
    async fn eth_gas_price(&self) -> Result<String, Error>;

    #[method(name = "eth_maxPriorityFeePerGas")]
    async fn eth_max_priority_fee_per_gas(&self) -> Result<String, Error>;

    #[method(name = "eth_blockNumber")]
    async fn eth_block_number(&self) -> Result<String, Error>;

    #[method(name = "eth_getBlockByNumber")]
    async fn eth_get_block_by_number(
        &self,
        block: BlockTag,
        full_tx: bool,
    ) -> Result<Option<ExecutionBlock>, Error>;

    #[method(name = "eth_getBlockByHash")]
    async fn eth_get_block_by_hash(
        &self,
        hash: &str,
        full_tx: bool,
    ) -> Result<Option<ExecutionBlock>, Error>;

    #[method(name = "eth_sendRawTransaction")]
    async fn eth_send_raw_transaction(&self, bytes: &str) -> Result<String, Error>;

    #[method(name = "eth_getTransactionReceipt")]
    async fn eth_get_transaction_receipt(
        &self,
        hash: &str,
    ) -> Result<Option<TransactionReceipt>, Error>;

    #[method(name = "eth_getTransactionByHash")]
    async fn eth_get_transaction_by_hash(
        &self,
        hash: &str,
    ) -> Result<Option<EthTransaction>, Error>;

    #[method(name = "eth_getTransactionByBlockHashAndIndex")]
    async fn eth_get_transaction_by_block_hash_and_index(
        &self,
        hash: &str,
        index: usize,
    ) -> Result<Option<EthTransaction>, Error>;

    #[method(name = "eth_getLogs")]
    async fn eth_get_logs(&self, filter: Filter) -> Result<Vec<Log>, Error>;

    #[method(name = "eth_getStorageAt")]
    async fn eth_get_storage_at(
        &self,
        address: &str,
        slot: H256,
        block: BlockTag,
    ) -> Result<String, Error>;

    #[method(name = "eth_coinbase")]
    async fn eth_coinbase(&self) -> Result<Address, Error>;

    #[method(name = "eth_syncing")]
    async fn eth_syncing(&self) -> Result<SyncingStatus, Error>;

    // Starknet endpoints
    #[method(name = "starknet_l2ToL1Messages")]
    async fn starknet_l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256, Error>;

    #[method(name = "starknet_chainId")]
    async fn starknet_chain_id(&self) -> Result<String, Error>;

    #[method(name = "starknet_getNonce")]
    async fn starknet_get_nonce(
        &self,
        contract_address: String,
        block_id: BlockId,
    ) -> Result<String, Error>;

    #[method(name = "starknet_blockNumber")]
    async fn starknet_block_number(&self) -> Result<u64, Error>;

    #[method(name = "starknet_getTransactionByHash")]
    async fn starknet_get_transaction_by_hash(
        &self,
        tx_hash: &str,
    ) -> Result<StarknetTransaction, Error>;

    #[method(name = "starknet_getBlockTransactionCount")]
    async fn starknet_get_block_transaction_count(&self, block_id: BlockId) -> Result<u64, Error>;

    #[method(name = "starknet_getClassAt")]
    async fn starknet_get_class_at(
        &self,
        block_id: BlockId,
        contract_address: String,
    ) -> Result<ContractClass, Error>;

    #[method(name = "starknet_blockHashAndNumber")]
    async fn starknet_block_hash_and_number(&self) -> Result<BlockHashAndNumber, Error>;

    #[method(name = "starknet_getBlockWithTxHashes")]
    async fn starknet_get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, Error>;

    #[method(name = "starknet_getContractStorageProof")]
    async fn starknet_get_contract_storage_proof(
        &self,
        block_id: BlockId,
        contract_address: String,
        keys: Vec<String>,
    ) -> Result<GetProofOutput, Error>;

    #[method(name = "starknet_getTransactionByBlockIdAndIndex")]
    async fn starknet_get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: &str,
    ) -> Result<StarknetTransaction, Error>;

    #[method(name = "starknet_addInvokeTransaction")]
    async fn starknet_add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, Error>;

    #[method(name = "starknet_getBlockWithTxs")]
    async fn starknet_get_block_with_txs(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxs, Error>;

    #[method(name = "starknet_getStateUpdate")]
    async fn starknet_get_state_update(&self, block_id: BlockId) -> Result<StateUpdate, Error>;

    #[method(name = "starknet_syncing")]
    async fn starknet_syncing(&self) -> Result<SyncStatusType, Error>;

    #[method(name = "starknet_l1ToL2Messages")]
    async fn starknet_l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256, Error>;

    #[method(name = "starknet_l1ToL2MessageNonce")]
    async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256, Error>;

    #[method(name = "starknet_l1ToL2MessageCancellations")]
    async fn starknet_l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256, Error>;

    #[method(name = "starknet_getTransactionReceipt")]
    async fn starknet_get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<MaybePendingTransactionReceipt, Error>;

    #[method(name = "starknet_getClassHashAt")]
    async fn starknet_get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: String,
    ) -> Result<FieldElement, Error>;

    #[method(name = "starknet_getClass")]
    async fn starknet_get_class(
        &self,
        block_id: BlockId,
        class_hash: String,
    ) -> Result<ContractClass, Error>;

    #[method(name = "starknet_addDeployAccountTransaction")]
    async fn starknet_add_deploy_account_transaction(
        &self,
        deploy_account_transaction: BroadcastedDeployAccountTransaction,
    ) -> Result<DeployAccountTransactionResult, Error>;

    #[method(name = "starknet_getEvents")]
    async fn starknet_get_events(
        &self,
        custom_filter: EventFilterWithPage,
    ) -> Result<EventsPage, Error>;

    #[method(name = "starknet_addDeclareTransaction")]
    async fn starknet_add_declare_transaction(
        &self,
        declare_transaction: BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, Error>;

    #[method(name = "starknet_pendingTransactions")]
    async fn starknet_pending_transactions(&self) -> Result<Vec<StarknetTransaction>, Error>;

    #[method(name = "starknet_estimateFee")]
    async fn starknet_estimate_fee(
        &self,
        block_id: BlockId,
        broadcasted_transaction: BroadcastedTransaction,
    ) -> Result<FeeEstimate, Error>;

    #[method(name = "starknet_call")]
    async fn starknet_call(
        &self,
        request: FunctionCall,
        block_id: BlockId,
    ) -> Result<Vec<FieldElement>, Error>;

    #[method(name = "starknet_getStorageAt")]
    async fn starknet_get_storage_at(
        &self,
        contract_address: String,
        key: String,
        block_id: BlockId,
    ) -> Result<FieldElement, Error>;
}
