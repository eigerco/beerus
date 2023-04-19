use beerus_core::lightclient::starknet::storage_proof::GetProofOutput;
use jsonrpsee::{
    core::Error,
    proc_macros::rpc,
    types::error::{CallError, ErrorObject},
};

use ethers::types::U256;
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::models::{
        BlockHashAndNumber, BlockId, BroadcastedInvokeTransaction, ContractClass,
        DeclareTransactionResult, DeployTransactionResult, EventFilter, EventsPage, FeeEstimate,
        FunctionCall, InvokeTransactionResult, MaybePendingBlockWithTxHashes,
        MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, StateUpdate, SyncStatusType,
        Transaction,
    },
};
pub const INVALID_CALL_DATA: &str = "Invalid call data";
pub const CLASS_HASH_NOT_FOUND: &str = "Class hash not found";
pub const INVALID_CONTRACT_CLASS: &str = "Invalid contract class";
pub const INVALID_CONTINUATION_TOKEN: &str =
    "The supplied continuation token is invalid or unknown";
pub const TOO_MANY_KEYS_IN_FILTER: &str = "Too many keys provided in a filter";
pub const PAGE_SIZE_TOO_BIG: &str = "Requested page size is too big";
pub const BLOCK_NOT_FOUND: &str = "Block not found";
pub const INVALID_TXN_INDEX: &str = "Invalid transaction index in a block";
pub const CONTRACT_NOT_FOUND: &str = "Contract not found";
pub const CONTRACT_ERROR: &str = "Contract error";

#[derive(thiserror::Error, Clone, Copy, Debug)]
pub enum BeerusApiError {
    #[error("Failed to write transaction")]
    FailedToReceiveTxn = 1,
    #[error("Contract not found")]
    ContractNotFound = 20,
    #[error("Invalid message selector")]
    InvalidMessageSelector = 21,
    #[error("Invalid call data")]
    InvalidCallData = 22,
    #[error("Block not found")]
    BlockNotFound = 24,
    #[error("Transaction hash not found")]
    TxnHashNotFound = 25,
    #[error("Invalid transaction index in a block")]
    InvalidTxnIndex = 27,
    #[error("Class hash not found")]
    ClassHashNotFound = 28,
    #[error("Requested page size is too big")]
    PageSizeTooBig = 31,
    #[error("There are no blocks")]
    NoBlocks = 32,
    #[error("The supplied continuation token is invalid or unknown")]
    InvalidContinuationToken = 33,
    #[error("Contract error")]
    ContractError = 40,
    #[error("Invalid contract class")]
    InvalidContractClass = 50,
    #[error("Too many storage keys requested")]
    ProofLimitExceeded = 10000,
    #[error("Too many keys provided in a filter")]
    TooManyKeysInFilter = 34,
    #[error("Internal server error")]
    InternalServerError = 500,
    #[error("Failed to fetch pending transactions")]
    FailedToFetchPendingTransactions = 38,
}

pub struct BeerusApiErrorWithMessage {
    pub error: BeerusApiError,
    pub message: String,
}

impl From<BeerusApiError> for Error {
    fn from(err: BeerusApiError) -> Self {
        Error::Call(CallError::Custom(ErrorObject::owned(
            err as i32,
            err.to_string(),
            None::<()>,
        )))
    }
}

impl From<BeerusApiErrorWithMessage> for Error {
    fn from(err: BeerusApiErrorWithMessage) -> Self {
        Error::Call(CallError::Custom(ErrorObject::owned(
            err.error as i32,
            err.message,
            None::<()>,
        )))
    }
}

#[rpc(server, client, namespace = "starknet")]
pub trait BeerusApi {
    // Starknet endpoints
    #[method(name = "l2_to_l1_messages")]
    async fn l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256, Error>;

    #[method(name = "chainId")]
    async fn chain_id(&self) -> Result<String, Error>;

    #[method(name = "getNonce")]
    async fn get_nonce(&self, contract_address: String) -> Result<FieldElement, Error>;

    #[method(name = "blockNumber")]
    async fn block_number(&self) -> Result<u64, Error>;

    #[method(name = "getTransactionByHash")]
    async fn get_transaction_by_hash(&self, tx_hash: &str) -> Result<Transaction, Error>;

    #[method(name = "getBlockTransactionCount")]
    async fn get_block_transaction_count(&self, block_id: BlockId) -> Result<u64, Error>;

    #[method(name = "getClassAt")]
    async fn get_class_at(
        &self,
        block_id: BlockId,
        contract_address: String,
    ) -> Result<ContractClass, Error>;

    #[method(name = "blockHashAndNumber")]
    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, Error>;

    #[method(name = "getBlockWithTxHashes")]
    async fn get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, Error>;

    #[method(name = "getContractStorageProof")]
    async fn get_contract_storage_proof(
        &self,
        block_id: BlockId,
        contract_address: String,
        keys: Vec<String>,
    ) -> Result<GetProofOutput, Error>;

    #[method(name = "getTransactionByBlockIdAndIndex")]
    async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: &str,
    ) -> Result<Transaction, Error>;

    #[method(name = "addInvokeTransaction")]
    async fn add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, Error>;

    #[method(name = "getBlockWithTxs")]
    async fn get_block_with_txs(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxs, Error>;

    #[method(name = "getStateUpdate")]
    async fn get_state_update(&self, block_id: BlockId) -> Result<StateUpdate, Error>;

    #[method(name = "syncing")]
    async fn syncing(&self) -> Result<SyncStatusType, Error>;

    #[method(name = "l1_to_l2_messages")]
    async fn l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256, Error>;

    #[method(name = "l1_to_l2_message_nonce")]
    async fn l1_to_l2_message_nonce(&self) -> Result<U256, Error>;

    #[method(name = "l1_to_l2_message_cancellations")]
    async fn l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256, Error>;

    #[method(name = "getTransactionReceipt")]
    async fn get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<MaybePendingTransactionReceipt, Error>;

    #[method(name = "getClassHashAt")]
    async fn get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: String,
    ) -> Result<FieldElement, Error>;

    #[method(name = "getClass")]
    async fn get_class(
        &self,
        block_id: BlockId,
        class_hash: String,
    ) -> Result<ContractClass, Error>;

    #[method(name = "addDeployAccountTransaction")]
    async fn add_deploy_account_transaction(
        &self,
        contract_class: String,
        version: String,
        contract_address_salt: String,
        constructor_calldata: Vec<String>,
    ) -> Result<DeployTransactionResult, Error>;

    #[method(name = "getEvents")]
    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, Error>;

    #[method(name = "addDeclareTransaction")]
    async fn add_declare_transaction(
        &self,
        version: String,
        max_fee: String,
        signature: Vec<String>,
        nonce: String,
        contract_class: String,
        sender_address: String,
    ) -> Result<DeclareTransactionResult, Error>;

    #[method(name = "pendingTransactions")]
    async fn pending_transactions(&self) -> Result<Vec<Transaction>, Error>;

    #[method(name = "estimateFee")]
    async fn estimate_fee(
        &self,
        block_id: BlockId,
        broadcasted_transaction: String,
    ) -> Result<FeeEstimate, Error>;

    #[method(name = "call")]
    async fn call(
        &self,
        request: FunctionCall,
        block_number: u64,
    ) -> Result<Vec<FieldElement>, Error>;

    #[method(name = "getStorageAt")]
    async fn get_storage_at(
        &self,
        contract_address: String,
        key: String,
    ) -> Result<FieldElement, Error>;
}
