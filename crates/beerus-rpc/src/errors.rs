use jsonrpsee::{
    core::Error,
    types::error::{CallError, ErrorObject},
};

/// JSON-RPC error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum BeerusApiError {
    /// Failed to write transaction
    #[error("Failed to write transaction")]
    FailedToReceiveTransaction,
    /// Contract not found
    #[error("Contract not found")]
    ContractNotFound,
    /// Invalid message selector
    #[error("Invalid message selector")]
    InvalidMessageSelector,
    /// Invalid call data
    #[error("Invalid call data")]
    InvalidCallData,
    /// Block not found
    #[error("Block not found")]
    BlockNotFound,
    /// Transaction hash not found
    #[error("Transaction hash not found")]
    TransactionHashNotFound,
    /// Invalid transaction index in a block
    #[error("Invalid transaction index in a block")]
    InvalidTransactionIndex,
    /// Class hash not found
    #[error("Class hash not found")]
    ClassHashNotFound,
    /// Requested page size is too big
    #[error("Requested page size is too big")]
    PageSizeTooBig,
    /// There are no blocks
    #[error("There are no blocks")]
    NoBlocks,
    /// The supplied continuation token is invalid or unknown
    #[error("The supplied continuation token is invalid or unknown")]
    InvalidContinuationToken,
    /// Contract error
    #[error("Contract error")]
    ContractError,
    /// Invalid contract class
    #[error("Invalid contract class")]
    InvalidContractClass,
    /// Failed to fetch pending transactions
    #[error("Failed to fetch pending transactions")]
    FailedToFetchPendingTransactions,
    /// Internal server error
    #[error("Internal server error")]
    InternalServerError,
    /// Too many storage keys requested
    #[error("Too many storage keys requested")]
    ProofLimitExceeded,
    /// Too many keys provided in a filter
    #[error("Too many keys provided in a filter")]
    TooManyKeysInFilter,
}

impl TryFrom<i32> for BeerusApiError {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => BeerusApiError::FailedToReceiveTransaction,
            20 => BeerusApiError::ContractNotFound,
            21 => BeerusApiError::InvalidMessageSelector,
            22 => BeerusApiError::InvalidCallData,
            24 => BeerusApiError::BlockNotFound,
            25 => BeerusApiError::TransactionHashNotFound,
            27 => BeerusApiError::InvalidTransactionIndex,
            28 => BeerusApiError::ClassHashNotFound,
            31 => BeerusApiError::PageSizeTooBig,
            32 => BeerusApiError::NoBlocks,
            33 => BeerusApiError::InvalidContinuationToken,
            34 => BeerusApiError::TooManyKeysInFilter,
            38 => BeerusApiError::FailedToFetchPendingTransactions,
            40 => BeerusApiError::ContractError,
            50 => BeerusApiError::InvalidContractClass,
            500 => BeerusApiError::InternalServerError,
            10000 => BeerusApiError::ProofLimitExceeded,
            _ => return Err(()),
        })
    }
}

impl TryFrom<BeerusApiError> for i32 {
    type Error = ();
    fn try_from(value: BeerusApiError) -> Result<Self, Self::Error> {
        Ok(match value {
            BeerusApiError::FailedToReceiveTransaction => 1,
            BeerusApiError::ContractNotFound => 20,
            BeerusApiError::InvalidMessageSelector => 21,
            BeerusApiError::InvalidCallData => 22,
            BeerusApiError::BlockNotFound => 24,
            BeerusApiError::TransactionHashNotFound => 25,
            BeerusApiError::InvalidTransactionIndex => 27,
            BeerusApiError::ClassHashNotFound => 28,
            BeerusApiError::PageSizeTooBig => 31,
            BeerusApiError::NoBlocks => 32,
            BeerusApiError::InvalidContinuationToken => 33,
            BeerusApiError::TooManyKeysInFilter => 34,
            BeerusApiError::FailedToFetchPendingTransactions => 38,
            BeerusApiError::ContractError => 40,
            BeerusApiError::InvalidContractClass => 50,
            BeerusApiError::InternalServerError => 500,
            BeerusApiError::ProofLimitExceeded => 10000,
        })
    }
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
