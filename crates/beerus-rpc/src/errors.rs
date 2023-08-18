use jsonrpsee::{
    core::Error,
    types::error::{CallError, ErrorObject},
};
use starknet::providers::jsonrpc::JsonRpcError;

pub const FAILED_TO_RECEIVE_TRANSACTION: i64 = 1;
pub const CONTRACT_NOT_FOUND: i64 = 20;
pub const INVALID_MESSAGE_SELECTOR: i64 = 21;
pub const INVALID_CALL_DATA: i64 = 22;
pub const BLOCK_NOT_FOUND: i64 = 24;
pub const TRANSACTION_HASH_NOT_FOUND: i64 = 25;
pub const INVALID_TRANSACTION_INDEX: i64 = 27;
pub const CLASS_HASH_NOT_FOUND: i64 = 28;
pub const PAGE_SIZE_TOO_BIG: i64 = 31;
pub const NO_BLOCKS: i64 = 32;
pub const INVALID_CONTINUATION_TOKEN: i64 = 33;
pub const TOO_MANY_KEYS_IN_FILTER: i64 = 34;
pub const FAILED_TO_FETCH_PENDING_TRANSACTIONS: i64 = 38;
pub const CONTRACT_ERROR: i64 = 40;
pub const INVALID_CONTRACT_CLASS: i64 = 50;
pub const INTERNAL_SERVER_ERROR: i64 = 500;
pub const PROOF_LIMIT_EXCEEDED: i64 = 10000;
pub const UNKNOWN_ERROR: i64 = 520;
pub const INVALID_PARAMS: i64 = 400;

/// JSON-RPC error codes
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BeerusApiError {
    #[error("Failed to write transaction")]
    FailedToReceiveTransaction(i64, String),
    #[error("Contract not found")]
    ContractNotFound(i64, String),
    #[error("Invalid message selector")]
    InvalidMessageSelector(i64, String),
    #[error("Invalid call data")]
    InvalidCallData(i64, String),
    #[error("Block not found")]
    BlockNotFound(i64, String),
    #[error("Transaction hash not found")]
    TransactionHashNotFound(i64, String),
    #[error("Invalid transaction index in a block")]
    InvalidTransactionIndex(i64, String),
    #[error("Class hash not found")]
    ClassHashNotFound(i64, String),
    #[error("Requested page size is too big")]
    PageSizeTooBig(i64, String),
    #[error("There are no blocks")]
    NoBlocks(i64, String),
    #[error("The supplied continuation token is invalid or unknown")]
    InvalidContinuationToken(i64, String),
    #[error("Contract error")]
    ContractError(i64, String),
    #[error("Invalid contract class")]
    InvalidContractClass(i64, String),
    #[error("Failed to fetch pending transactions")]
    FailedToFetchPendingTransactions(i64, String),
    #[error("Internal server error")]
    InternalServerError(i64, String),
    #[error("Too many storage keys requested")]
    ProofLimitExceeded(i64, String),
    #[error("Too many keys provided in a filter")]
    TooManyKeysInFilter(i64, String),
    #[error("Unknown error")]
    UnknownError(i64, String),
    #[error("Invalid params")]
    InvalidParams(i64, String),
}

pub fn invalid_call_data(param: &str) -> Error {
    let message = format!("Invalid params: cannot parse '{}'.", param);
    Error::from(BeerusApiError::InvalidParams(INVALID_PARAMS, message))
}

// The conversion from JsonRpcError to BeerusApiError is done based on the error code.
// I avoid directly using BeerusApiError::from(JsonRpcError.code) because the JsonRpcError
// may contain an error message from lower layers, and we want to prevent any loss of information.
impl From<JsonRpcError> for BeerusApiError {
    fn from(err: JsonRpcError) -> Self {
        match err.code {
            FAILED_TO_RECEIVE_TRANSACTION => BeerusApiError::FailedToReceiveTransaction(
                FAILED_TO_RECEIVE_TRANSACTION,
                err.message,
            ),
            CONTRACT_NOT_FOUND => BeerusApiError::ContractNotFound(CONTRACT_NOT_FOUND, err.message),
            INVALID_MESSAGE_SELECTOR => {
                BeerusApiError::InvalidMessageSelector(INVALID_MESSAGE_SELECTOR, err.message)
            }
            INVALID_CALL_DATA => BeerusApiError::InvalidCallData(INVALID_CALL_DATA, err.message),
            BLOCK_NOT_FOUND => BeerusApiError::BlockNotFound(BLOCK_NOT_FOUND, err.message),
            TRANSACTION_HASH_NOT_FOUND => {
                BeerusApiError::TransactionHashNotFound(TRANSACTION_HASH_NOT_FOUND, err.message)
            }
            INVALID_TRANSACTION_INDEX => {
                BeerusApiError::InvalidTransactionIndex(INVALID_TRANSACTION_INDEX, err.message)
            }
            CLASS_HASH_NOT_FOUND => {
                BeerusApiError::ClassHashNotFound(CLASS_HASH_NOT_FOUND, err.message)
            }
            PAGE_SIZE_TOO_BIG => BeerusApiError::PageSizeTooBig(PAGE_SIZE_TOO_BIG, err.message),
            NO_BLOCKS => BeerusApiError::NoBlocks(NO_BLOCKS, err.message),
            INVALID_CONTINUATION_TOKEN => {
                BeerusApiError::InvalidContinuationToken(INVALID_CONTINUATION_TOKEN, err.message)
            }
            TOO_MANY_KEYS_IN_FILTER => {
                BeerusApiError::TooManyKeysInFilter(TOO_MANY_KEYS_IN_FILTER, err.message)
            }
            FAILED_TO_FETCH_PENDING_TRANSACTIONS => {
                BeerusApiError::FailedToFetchPendingTransactions(
                    FAILED_TO_FETCH_PENDING_TRANSACTIONS,
                    err.message,
                )
            }
            CONTRACT_ERROR => BeerusApiError::ContractError(CONTRACT_ERROR, err.message),
            INVALID_CONTRACT_CLASS => {
                BeerusApiError::InvalidContractClass(INVALID_CONTRACT_CLASS, err.message)
            }
            INTERNAL_SERVER_ERROR => {
                BeerusApiError::InternalServerError(INTERNAL_SERVER_ERROR, err.message)
            }
            PROOF_LIMIT_EXCEEDED => {
                BeerusApiError::ProofLimitExceeded(PROOF_LIMIT_EXCEEDED, err.message)
            }
            _ => BeerusApiError::UnknownError(UNKNOWN_ERROR, err.message),
        }
    }
}

impl From<BeerusApiError> for Error {
    fn from(err: BeerusApiError) -> Self {
        // Todo :: consider this conversion seems to be not convenient, cause
        //         we use only error code and message and ignoring generated thiserror msg
        let (code, msg) = err.clone().into();
        Error::Call(CallError::Custom(ErrorObject::owned(
            code as i32,
            msg,
            None::<()>,
        )))
    }
}

// The conversion from i64 to BeerusApiError will include the default messages and codes.
impl From<i64> for BeerusApiError {
    fn from(value: i64) -> BeerusApiError {
        match value {
            FAILED_TO_RECEIVE_TRANSACTION => BeerusApiError::FailedToReceiveTransaction(
                FAILED_TO_RECEIVE_TRANSACTION,
                "Failed to write transaction".into(),
            ),
            CONTRACT_NOT_FOUND => {
                BeerusApiError::ContractNotFound(CONTRACT_NOT_FOUND, "Contract not found".into())
            }
            INVALID_MESSAGE_SELECTOR => BeerusApiError::InvalidMessageSelector(
                INVALID_MESSAGE_SELECTOR,
                "Invalid message selector".into(),
            ),
            INVALID_CALL_DATA => {
                BeerusApiError::InvalidCallData(INVALID_CALL_DATA, "Invalid call data".into())
            }
            BLOCK_NOT_FOUND => {
                BeerusApiError::BlockNotFound(BLOCK_NOT_FOUND, "Block not found".into())
            }
            TRANSACTION_HASH_NOT_FOUND => BeerusApiError::TransactionHashNotFound(
                TRANSACTION_HASH_NOT_FOUND,
                "Transaction hash not found".into(),
            ),
            INVALID_TRANSACTION_INDEX => BeerusApiError::InvalidTransactionIndex(
                INVALID_TRANSACTION_INDEX,
                "Invalid transaction index in a block".into(),
            ),
            CLASS_HASH_NOT_FOUND => BeerusApiError::ClassHashNotFound(
                CLASS_HASH_NOT_FOUND,
                "Class hash not found".into(),
            ),
            PAGE_SIZE_TOO_BIG => BeerusApiError::PageSizeTooBig(
                PAGE_SIZE_TOO_BIG,
                "Requested page size is too big".into(),
            ),
            NO_BLOCKS => BeerusApiError::NoBlocks(NO_BLOCKS, "There are no blocks".into()),
            INVALID_CONTINUATION_TOKEN => BeerusApiError::InvalidContinuationToken(
                INVALID_CONTINUATION_TOKEN,
                "The supplied continuation token is invalid or unknown".into(),
            ),
            TOO_MANY_KEYS_IN_FILTER => BeerusApiError::TooManyKeysInFilter(
                TOO_MANY_KEYS_IN_FILTER,
                "Too many keys provided in a filter".into(),
            ),
            FAILED_TO_FETCH_PENDING_TRANSACTIONS => {
                BeerusApiError::FailedToFetchPendingTransactions(
                    FAILED_TO_FETCH_PENDING_TRANSACTIONS,
                    "Failed to fetch pending transactions".into(),
                )
            }
            CONTRACT_ERROR => {
                BeerusApiError::ContractError(CONTRACT_ERROR, "Contract error".into())
            }
            INVALID_CONTRACT_CLASS => BeerusApiError::InvalidContractClass(
                INVALID_CONTRACT_CLASS,
                "Invalid contract class".into(),
            ),
            INTERNAL_SERVER_ERROR => BeerusApiError::InternalServerError(
                INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
            PROOF_LIMIT_EXCEEDED => BeerusApiError::ProofLimitExceeded(
                PROOF_LIMIT_EXCEEDED,
                "Too many storage keys requested".into(),
            ),
            _ => BeerusApiError::UnknownError(UNKNOWN_ERROR, "Unknown error".into()),
        }
    }
}

impl From<BeerusApiError> for (i64, String) {
    fn from(value: BeerusApiError) -> (i64, String) {
        match value {
            BeerusApiError::FailedToReceiveTransaction(code, msg) => (code, msg),
            BeerusApiError::ContractNotFound(code, msg) => (code, msg),
            BeerusApiError::InvalidMessageSelector(code, msg) => (code, msg),
            BeerusApiError::InvalidCallData(code, msg) => (code, msg),
            BeerusApiError::BlockNotFound(code, msg) => (code, msg),
            BeerusApiError::TransactionHashNotFound(code, msg) => (code, msg),
            BeerusApiError::InvalidTransactionIndex(code, msg) => (code, msg),
            BeerusApiError::ClassHashNotFound(code, msg) => (code, msg),
            BeerusApiError::PageSizeTooBig(code, msg) => (code, msg),
            BeerusApiError::NoBlocks(code, msg) => (code, msg),
            BeerusApiError::InvalidContinuationToken(code, msg) => (code, msg),
            BeerusApiError::TooManyKeysInFilter(code, msg) => (code, msg),
            BeerusApiError::FailedToFetchPendingTransactions(code, msg) => (code, msg),
            BeerusApiError::ContractError(code, msg) => (code, msg),
            BeerusApiError::InvalidContractClass(code, msg) => (code, msg),
            BeerusApiError::InternalServerError(code, msg) => (code, msg),
            BeerusApiError::ProofLimitExceeded(code, msg) => (code, msg),
            BeerusApiError::UnknownError(code, msg) => (code, msg),
            _ => (520, String::from("Unknown")), // Unknown error
        }
    }
}
