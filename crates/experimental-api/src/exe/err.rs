use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO error: {0:?}")]
    Io(std::io::Error),
    #[error("Reqwest error: {0:?}")]
    Reqwest(reqwest::Error),
    #[error("Codegen error: {0:?}")]
    IamGroot(iamgroot::jsonrpc::Error),
    #[error("Starknet API error: {0:?}")]
    StarknetApi(starknet_api::StarknetApiError),
    #[error("Blockifier state error: {0:?}")]
    State(blockifier::state::errors::StateError),
    #[error("Blockifier entry point error: {0:?}")]
    EntryPoint(blockifier::execution::errors::EntryPointExecutionError),
    #[error("Blockifier transaction error: {0:?}")]
    Transaction(blockifier::transaction::errors::TransactionExecutionError),
    #[error("{0}")]
    Custom(&'static str),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
}

impl From<starknet_api::StarknetApiError> for Error {
    fn from(error: starknet_api::StarknetApiError) -> Self {
        Self::StarknetApi(error)
    }
}

impl From<blockifier::execution::errors::EntryPointExecutionError> for Error {
    fn from(
        error: blockifier::execution::errors::EntryPointExecutionError,
    ) -> Self {
        Self::EntryPoint(error)
    }
}

impl From<blockifier::transaction::errors::TransactionExecutionError>
    for Error
{
    fn from(
        error: blockifier::transaction::errors::TransactionExecutionError,
    ) -> Self {
        Self::Transaction(error)
    }
}

impl From<iamgroot::jsonrpc::Error> for Error {
    fn from(error: iamgroot::jsonrpc::Error) -> Self {
        Self::IamGroot(error)
    }
}

impl From<Error> for blockifier::state::errors::StateError {
    fn from(error: Error) -> Self {
        blockifier::state::errors::StateError::StateReadError(format!(
            "{error:?}"
        ))
    }
}
