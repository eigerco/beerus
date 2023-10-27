use beerus_core::CoreError;
use eyre::Report;
use jsonrpsee::types::ErrorObjectOwned;
use starknet::providers::jsonrpc::{HttpTransportError, JsonRpcClientError};
use starknet::providers::MaybeUnknownErrorCode::{self, Known, Unknown};
use starknet::providers::ProviderError::StarknetError;
use starknet::providers::{AnyProviderError, ProviderError, StarknetErrorWithMessage};
pub struct BeerusRpcError(ProviderError<AnyProviderError>);

impl From<BeerusRpcError> for ErrorObjectOwned {
    fn from(err: BeerusRpcError) -> Self {
        let sn_err = match err.0 {
            StarknetError(x) => x,
            _ => StarknetErrorWithMessage {
                code: MaybeUnknownErrorCode::Unknown(-32601),
                message: "Method not found".to_string(),
            },
        };
        match sn_err.code {
            Known(_) => ErrorObjectOwned::owned(-32601, sn_err.message, None::<()>),
            Unknown(unknown_sn_err) => ErrorObjectOwned::owned(unknown_sn_err as i32, sn_err.message, None::<()>),
        }
    }
}

impl From<ProviderError<JsonRpcClientError<HttpTransportError>>> for BeerusRpcError {
    fn from(err: ProviderError<JsonRpcClientError<HttpTransportError>>) -> Self {
        match err {
            StarknetError(sn_err) => BeerusRpcError(ProviderError::StarknetError(sn_err)),
            _ => BeerusRpcError(ProviderError::StarknetError(StarknetErrorWithMessage {
                code: MaybeUnknownErrorCode::Unknown(-32601),
                message: "Method not found".to_string(),
            })),
        }
    }
}

impl From<CoreError> for BeerusRpcError {
    fn from(err: CoreError) -> Self {
        BeerusRpcError(ProviderError::StarknetError(StarknetErrorWithMessage {
            code: MaybeUnknownErrorCode::Unknown(-32601),
            message: format!("{err}"),
        }))
    }
}

impl From<Report> for BeerusRpcError {
    fn from(err: Report) -> Self {
        BeerusRpcError(ProviderError::StarknetError(StarknetErrorWithMessage {
            code: MaybeUnknownErrorCode::Unknown(-32601),
            message: format!("{err}"),
        }))
    }
}
