use beerus_core::CoreError;
use eyre::Report;
use jsonrpsee::types::ErrorObjectOwned;
use starknet::core::types::StarknetError as StarknetErr;
use starknet::providers::ProviderError;
// use starknet::providers::jsonrpc::{HttpTransportError, JsonRpcClientError};
use starknet::providers::ProviderError::StarknetError;

#[derive(Debug)]
pub enum BeerusRpcError {
    Provider(ProviderError),
    Other((i32, String)),
}

impl From<BeerusRpcError> for ErrorObjectOwned {
    fn from(err: BeerusRpcError) -> Self {
        match err {
            BeerusRpcError::Provider(provider_err) => match provider_err {
                StarknetError(sn_err) => {
                    let mut data = None;
                    let code = match &sn_err {
                        StarknetErr::FailedToReceiveTransaction => 1,
                        StarknetErr::ContractNotFound => 20,
                        StarknetErr::BlockNotFound => 24,
                        StarknetErr::InvalidTransactionIndex => 27,
                        StarknetErr::ClassHashNotFound => 28,
                        StarknetErr::TransactionHashNotFound => 29,
                        StarknetErr::PageSizeTooBig => 31,
                        StarknetErr::NoBlocks => 32,
                        StarknetErr::InvalidContinuationToken => 33,
                        StarknetErr::TooManyKeysInFilter => 34,
                        StarknetErr::ContractError(err) => {
                            data = Some(format!("{err:?}"));
                            40
                        }
                        StarknetErr::TransactionExecutionError(err) => {
                            data = Some(format!("{err:?}"));
                            41
                        }
                        StarknetErr::ClassAlreadyDeclared => 51,
                        StarknetErr::InvalidTransactionNonce => 52,
                        StarknetErr::InsufficientMaxFee => 53,
                        StarknetErr::InsufficientAccountBalance => 54,
                        StarknetErr::ValidationFailure(err) => {
                            data = Some(err.to_string());
                            55
                        }
                        StarknetErr::CompilationFailed => 56,
                        StarknetErr::ContractClassSizeIsTooLarge => 57,
                        StarknetErr::NonAccount => 58,
                        StarknetErr::DuplicateTx => 59,
                        StarknetErr::CompiledClassHashMismatch => 60,
                        StarknetErr::UnsupportedTxVersion => 61,
                        StarknetErr::UnsupportedContractClassVersion => 62,
                        StarknetErr::UnexpectedError(err) => {
                            data = Some(err.to_string());
                            63
                        }
                        StarknetErr::NoTraceAvailable(err) => {
                            data = Some(format!("{err:?}"));
                            10
                        }
                    };
                    ErrorObjectOwned::owned(code, sn_err.message(), data)
                }
                _ => ErrorObjectOwned::owned(-32601, format!("{provider_err}"), None::<()>),
            },
            BeerusRpcError::Other(other_err) => ErrorObjectOwned::owned(other_err.0, other_err.1, None::<()>),
        }
    }
}

impl From<ProviderError> for BeerusRpcError {
    fn from(err: ProviderError) -> Self {
        BeerusRpcError::Provider(err)
    }
}

impl From<CoreError> for BeerusRpcError {
    fn from(err: CoreError) -> Self {
        BeerusRpcError::Other((-32601, format!("{err}")))
    }
}

impl From<Report> for BeerusRpcError {
    fn from(err: Report) -> Self {
        BeerusRpcError::Other((-32601, format!("{err}")))
    }
}
