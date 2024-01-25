use beerus_core::CoreError;
use eyre::Report;
use jsonrpsee::types::ErrorObjectOwned;
use starknet::core::types::StarknetError;
use starknet::providers::jsonrpc::{HttpTransportError, JsonRpcClientError};
use starknet::providers::{MaybeUnknownErrorCode, ProviderError};

#[derive(Debug)]
pub enum BeerusRpcError {
    Provider(ProviderError<JsonRpcClientError<HttpTransportError>>),
    Other((i32, String)),
}

impl From<BeerusRpcError> for ErrorObjectOwned {
    fn from(err: BeerusRpcError) -> Self {
        match err {
            BeerusRpcError::Provider(e) => {
                match e {
                    ProviderError::StarknetError(e) => {
                        let (code, message) = (e.code, e.message);
                        let code = match code {
                        MaybeUnknownErrorCode::Known(known) => match known {
                            StarknetError::FailedToReceiveTransaction => 1,
                            StarknetError::ContractNotFound => 20,
                            StarknetError::BlockNotFound => 24,
                            StarknetError::InvalidTransactionIndex => 27,
                            StarknetError::ClassHashNotFound => 28,
                            StarknetError::TransactionHashNotFound => 29,
                            StarknetError::PageSizeTooBig => 31,
                            StarknetError::NoBlocks => 32,
                            StarknetError::InvalidContinuationToken => 33,
                            StarknetError::TooManyKeysInFilter => 34,
                            StarknetError::ContractError => 40,
                            StarknetError::ClassAlreadyDeclared => 51,
                            StarknetError::InvalidTransactionNonce => 52,
                            StarknetError::InsufficientMaxFee => 53,
                            StarknetError::InsufficientAccountBalance => 54,
                            StarknetError::ValidationFailure => 55,
                            StarknetError::CompilationFailed => 56,
                            StarknetError::ContractClassSizeIsTooLarge => 57,
                            StarknetError::NonAccount => 58,
                            StarknetError::DuplicateTx => 59,
                            StarknetError::CompiledClassHashMismatch => 60,
                            StarknetError::UnsupportedTxVersion => 61,
                            StarknetError::UnsupportedContractClassVersion => 62,
                            StarknetError::UnexpectedError => 63,
                        },
                        MaybeUnknownErrorCode::Unknown(unknown) => unknown as i32,
                    };
                        ErrorObjectOwned::owned(code, message, None::<()>)
                    }
                    _ => ErrorObjectOwned::owned(
                        -32601,
                        format!("{e}"),
                        None::<()>,
                    ),
                }
            }
            BeerusRpcError::Other((code, message)) => {
                ErrorObjectOwned::owned(code, message, None::<()>)
            }
        }
    }
}

impl From<ProviderError<JsonRpcClientError<HttpTransportError>>>
    for BeerusRpcError
{
    fn from(e: ProviderError<JsonRpcClientError<HttpTransportError>>) -> Self {
        BeerusRpcError::Provider(e)
    }
}

impl From<CoreError> for BeerusRpcError {
    fn from(e: CoreError) -> Self {
        BeerusRpcError::Other((-32601, format!("{e}")))
    }
}

impl From<Report> for BeerusRpcError {
    fn from(e: Report) -> Self {
        BeerusRpcError::Other((-32601, format!("{e}")))
    }
}
