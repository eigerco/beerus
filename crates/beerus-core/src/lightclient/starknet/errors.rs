use ethers::providers::ProviderError;
use reqwest::Error as ReqwestError;
use starknet::providers::jsonrpc::{JsonRpcClientError, JsonRpcError, RpcError};
use starknet::providers::sequencer::ErrorCode;
use starknet::providers::ProviderError as StarknetProviderError;

pub struct JsonRpcClientErrorWrapper(StarknetProviderError<JsonRpcClientError<ReqwestError>>);
#[derive(Debug, thiserror::Error)]
#[error("unable to map JsonRpcErrorClient type to JsonRpcError type")]
pub struct JsonRpcClientConversionError {
    message: String,
}

pub struct StarknetErrorCodeWrapper {
    code: i64,
}

impl TryFrom<JsonRpcClientErrorWrapper> for JsonRpcError {
    type Error = JsonRpcClientConversionError;
    fn try_from(err: JsonRpcClientErrorWrapper) -> Result<Self, Self::Error> {
        Err(JsonRpcClientConversionError {
            message: "Unable to map JsonRpcClientError, raw error: ".to_owned()
                + &err.0.to_string(),
        })
        // match err.0 {
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::BlockNotFound)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::BlockNotFound).code,
        //         message: err.0.to_string(),
        //     })
        // }
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::ContractError)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::ContractError).code,
        //         message: err.0.to_string(),
        //     })
        // }
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::NoBlocks)) => Ok(JsonRpcError {
        //     code: StarknetErrorCodeWrapper::from(ErrorCode::NoBlocks).code,
        //     message: err.0.to_string(),
        // }),
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::ContractNotFound)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::ContractNotFound).code,
        //         message: err.0.to_string(),
        //     })
        // }
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::ClassHashNotFound)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::ClassHashNotFound).code,
        //         message: err.0.to_string(),
        //     })
        // }
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::InvalidContinuationToken)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::InvalidContinuationToken).code,
        //         message: err.0.to_string(),
        //     })
        // }
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::InvalidCallData)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::InvalidCallData).code,
        //         message: err.0.to_string(),
        //     })
        // }
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::FailedToReceiveTransaction)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::FailedToReceiveTransaction)
        //             .code,
        //         message: err.0.to_string(),
        //     })
        // }
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::InvalidMessageSelector)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::InvalidMessageSelector).code,
        //         message: err.0.to_string(),
        //     })
        // }
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::TransactionHashNotFound)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::TransactionHashNotFound).code,
        //         message: err.0.to_string(),
        //     })
        // }
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::PageSizeTooBig)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::PageSizeTooBig).code,
        //         message: err.0.to_string(),
        //     })
        // }
        // JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::InvalidTransactionIndex)) => {
        //     Ok(JsonRpcError {
        //         code: StarknetErrorCodeWrapper::from(ErrorCode::InvalidTransactionIndex).code,
        //         message: err.0.to_string(),
        //     })
        // }
        // _ => Err(JsonRpcClientConversionError {
        //     message: "Unable to map JsonRpcClientError, raw error: ".to_owned()
        //         + &err.0.to_string(),
        // }),
        // }
    }
}

impl From<ProviderError> for JsonRpcClientErrorWrapper {
    fn from(err: ProviderError) -> Self {
        JsonRpcClientErrorWrapper(StarknetProviderError::Other(JsonRpcClientError::RpcError(
            RpcError::Unknown(JsonRpcError {
                code: 520, // Unknown error, at least we keep the message
                message: err.to_string(),
            }),
        )))
    }
}

impl From<JsonRpcClientErrorWrapper> for StarknetProviderError<JsonRpcClientError<ReqwestError>> {
    fn from(err: JsonRpcClientErrorWrapper) -> Self {
        err.0
    }
}

impl From<StarknetProviderError<JsonRpcClientError<ReqwestError>>> for JsonRpcClientErrorWrapper {
    fn from(err: StarknetProviderError<JsonRpcClientError<ReqwestError>>) -> Self {
        JsonRpcClientErrorWrapper(err)
    }
}

// Since we dont have conversion ErrorCode -> i64 (dont implemented in starknet-rs) this is necessary.
impl From<ErrorCode> for StarknetErrorCodeWrapper {
    fn from(code: ErrorCode) -> Self {
        StarknetErrorCodeWrapper { code: 500 }
        // match code {
        //     ErrorCode::FailedToReceiveTransaction => StarknetErrorCodeWrapper { code: 1 },
        //     ErrorCode::ContractNotFound => StarknetErrorCodeWrapper { code: 20 },
        //     ErrorCode::InvalidMessageSelector => StarknetErrorCodeWrapper { code: 21 },
        //     ErrorCode::InvalidCallData => StarknetErrorCodeWrapper { code: 22 },
        //     ErrorCode::BlockNotFound => StarknetErrorCodeWrapper { code: 24 },
        //     ErrorCode::TransactionHashNotFound => StarknetErrorCodeWrapper { code: 25 },
        //     ErrorCode::InvalidTransactionIndex => StarknetErrorCodeWrapper { code: 27 },
        //     ErrorCode::ClassHashNotFound => StarknetErrorCodeWrapper { code: 28 },
        //     ErrorCode::PageSizeTooBig => StarknetErrorCodeWrapper { code: 31 },
        //     ErrorCode::NoBlocks => StarknetErrorCodeWrapper { code: 32 },
        //     ErrorCode::InvalidContinuationToken => StarknetErrorCodeWrapper { code: 33 },
        //     ErrorCode::ContractError => StarknetErrorCodeWrapper { code: 40 },
        // }
    }
}
