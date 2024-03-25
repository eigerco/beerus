use beerus_core::CoreError;
use eyre::Report;
use jsonrpsee::core::Error as JsonRpcError;
use jsonrpsee::types::ErrorObjectOwned;
use starknet::core::types::StarknetError;
use starknet::providers::ProviderError;
use starknet::providers::ProviderError::StarknetError as StarknetProviderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunError {
    #[error("wrong RPC spec version: local is {1} but remove is {0}")]
    WrongSpecVersion(String, String),
    #[error(transparent)]
    RpcServer(#[from] JsonRpcError),
    #[error(transparent)]
    Provider(#[from] ProviderError),
}

#[derive(Error, Debug)]
pub enum RpcError {
    #[error(transparent)]
    Provider(#[from] ProviderError),
    #[error("{0:?}")]
    Other((i32, String)),
}

// TODO: Remove this completely after full releace cycle
#[allow(unused_imports)]
#[deprecated]
use self::RpcError as BeerusRpcError;

impl From<RpcError> for ErrorObjectOwned {
    fn from(err: RpcError) -> Self {
        match err {
            RpcError::Provider(provider_err) => match provider_err {
                StarknetProviderError(sn_err) => match &sn_err {
                    StarknetError::FailedToReceiveTransaction => {
                        ErrorObjectOwned::owned(1, sn_err.message(), None::<()>)
                    }
                    StarknetError::ContractNotFound => ErrorObjectOwned::owned(
                        20,
                        sn_err.message(),
                        None::<()>,
                    ),
                    StarknetError::BlockNotFound => ErrorObjectOwned::owned(
                        24,
                        sn_err.message(),
                        None::<()>,
                    ),
                    StarknetError::InvalidTransactionIndex => {
                        ErrorObjectOwned::owned(
                            27,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::ClassHashNotFound => {
                        ErrorObjectOwned::owned(
                            28,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::TransactionHashNotFound => {
                        ErrorObjectOwned::owned(
                            29,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::PageSizeTooBig => ErrorObjectOwned::owned(
                        31,
                        sn_err.message(),
                        None::<()>,
                    ),
                    StarknetError::NoBlocks => ErrorObjectOwned::owned(
                        32,
                        sn_err.message(),
                        None::<()>,
                    ),
                    StarknetError::InvalidContinuationToken => {
                        ErrorObjectOwned::owned(
                            33,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::TooManyKeysInFilter => {
                        ErrorObjectOwned::owned(
                            34,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::ContractError(data) => {
                        ErrorObjectOwned::owned(
                            40,
                            sn_err.message(),
                            Some(data),
                        )
                    }
                    StarknetError::TransactionExecutionError(data) => {
                        ErrorObjectOwned::owned(
                            41,
                            sn_err.message(),
                            Some(data),
                        )
                    }
                    StarknetError::ClassAlreadyDeclared => {
                        ErrorObjectOwned::owned(
                            51,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::InvalidTransactionNonce => {
                        ErrorObjectOwned::owned(
                            51,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::InsufficientMaxFee => {
                        ErrorObjectOwned::owned(
                            53,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::InsufficientAccountBalance => {
                        ErrorObjectOwned::owned(
                            54,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::ValidationFailure(data) => {
                        ErrorObjectOwned::owned(
                            55,
                            sn_err.message(),
                            Some(data),
                        )
                    }
                    StarknetError::CompilationFailed => {
                        ErrorObjectOwned::owned(
                            56,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::ContractClassSizeIsTooLarge => {
                        ErrorObjectOwned::owned(
                            57,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::NonAccount => ErrorObjectOwned::owned(
                        58,
                        sn_err.message(),
                        None::<()>,
                    ),
                    StarknetError::DuplicateTx => ErrorObjectOwned::owned(
                        59,
                        sn_err.message(),
                        None::<()>,
                    ),
                    StarknetError::CompiledClassHashMismatch => {
                        ErrorObjectOwned::owned(
                            60,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::UnsupportedTxVersion => {
                        ErrorObjectOwned::owned(
                            61,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::UnsupportedContractClassVersion => {
                        ErrorObjectOwned::owned(
                            62,
                            sn_err.message(),
                            None::<()>,
                        )
                    }
                    StarknetError::UnexpectedError(data) => {
                        ErrorObjectOwned::owned(
                            63,
                            sn_err.message(),
                            Some(data),
                        )
                    }
                    StarknetError::NoTraceAvailable(data) => {
                        ErrorObjectOwned::owned(
                            10,
                            sn_err.message(),
                            Some(data),
                        )
                    }
                },
                _ => ErrorObjectOwned::owned(
                    -32601,
                    format!("{provider_err}"),
                    None::<()>,
                ),
            },
            RpcError::Other(other_err) => {
                ErrorObjectOwned::owned(other_err.0, other_err.1, None::<()>)
            }
        }
    }
}

// TODO: break this unnecessary coupling
impl From<CoreError> for RpcError {
    fn from(err: CoreError) -> Self {
        RpcError::Other((-32601, format!("{err}")))
    }
}

// TODO: break this unnecessary coupling
impl From<Report> for RpcError {
    fn from(err: Report) -> Self {
        RpcError::Other((-32601, format!("{err}")))
    }
}

impl From<JsonRpcError> for RpcError {
    fn from(err: JsonRpcError) -> Self {
        RpcError::Other((-32601, format!("{err}")))
    }
}
