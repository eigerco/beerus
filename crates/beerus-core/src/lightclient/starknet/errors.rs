use ethers::providers::ProviderError as EthersProviderError;
use starknet::core::types::StarknetError;
use starknet::providers::{
    jsonrpc::{HttpTransportError, JsonRpcClientError, JsonRpcError, RpcError},
    MaybeUnknownErrorCode, ProviderError as StarknetProviderError, StarknetErrorWithMessage,
};

pub struct JsonRpcClientErrorWrapper(StarknetProviderError<JsonRpcClientError<HttpTransportError>>);
#[derive(Debug, thiserror::Error)]
#[error("unable to map JsonRpcErrorClient type to JsonRpcError type")]
pub struct JsonRpcClientConversionError {
    pub message: String,
}

pub struct StarknetErrorCodeWrapper {
    code: i64,
}

impl TryFrom<JsonRpcClientErrorWrapper> for JsonRpcError {
    type Error = JsonRpcClientConversionError;
    fn try_from(err: JsonRpcClientErrorWrapper) -> Result<Self, Self::Error> {
        match err.0 {
            StarknetProviderError::StarknetError(err) => {
                let msg = err.message.clone();
                Ok(Self {
                    code: StarknetErrorCodeWrapper::from(err).code,
                    message: msg,
                })
            }
            _ => Err(JsonRpcClientConversionError {
                message: "Unable to map JsonRpcClientError, raw error: ".to_owned()
                    + &err.0.to_string(),
            }),
        }
    }
}

impl From<EthersProviderError> for JsonRpcClientErrorWrapper {
    fn from(err: EthersProviderError) -> Self {
        JsonRpcClientErrorWrapper(StarknetProviderError::Other(JsonRpcClientError::RpcError(
            RpcError::Unknown(JsonRpcError {
                code: 520, // Unknown error, at least we keep the message
                message: err.to_string(),
            }),
        )))
    }
}

impl From<JsonRpcClientErrorWrapper>
    for StarknetProviderError<JsonRpcClientError<HttpTransportError>>
{
    fn from(err: JsonRpcClientErrorWrapper) -> Self {
        err.0
    }
}

impl From<StarknetProviderError<JsonRpcClientError<HttpTransportError>>>
    for JsonRpcClientErrorWrapper
{
    fn from(err: StarknetProviderError<JsonRpcClientError<HttpTransportError>>) -> Self {
        JsonRpcClientErrorWrapper(err)
    }
}

// Since we dont have conversion StarknetError -> i64 (dont implemented in starknet-rs) this is necessary.
impl From<StarknetErrorWithMessage> for StarknetErrorCodeWrapper {
    fn from(err: StarknetErrorWithMessage) -> Self {
        match err.code {
            MaybeUnknownErrorCode::Unknown(code) => StarknetErrorCodeWrapper { code },
            MaybeUnknownErrorCode::Known(starknet_error) => match starknet_error {
                StarknetError::FailedToReceiveTransaction => StarknetErrorCodeWrapper { code: 1 },
                StarknetError::ContractNotFound => StarknetErrorCodeWrapper { code: 20 },
                StarknetError::BlockNotFound => StarknetErrorCodeWrapper { code: 24 },
                StarknetError::TransactionHashNotFound => StarknetErrorCodeWrapper { code: 25 },
                StarknetError::InvalidTransactionIndex => StarknetErrorCodeWrapper { code: 27 },
                StarknetError::ClassHashNotFound => StarknetErrorCodeWrapper { code: 28 },
                StarknetError::PageSizeTooBig => StarknetErrorCodeWrapper { code: 31 },
                StarknetError::NoBlocks => StarknetErrorCodeWrapper { code: 32 },
                StarknetError::InvalidContinuationToken => StarknetErrorCodeWrapper { code: 33 },
                StarknetError::TooManyKeysInFilter => StarknetErrorCodeWrapper { code: 34 },
                StarknetError::ContractError => StarknetErrorCodeWrapper { code: 40 },
                StarknetError::InvalidContractClass => StarknetErrorCodeWrapper { code: 50 },
                StarknetError::ClassAlreadyDeclared => StarknetErrorCodeWrapper { code: 51 },
            },
        }
    }
}
