use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO error: {0:?}")]
    Io(std::io::Error),
    #[error("Base64 error: {0:?}")]
    Base64(base64::DecodeError),
    #[error("Serde error: {0:?}")]
    Serde(serde_json::Error),
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
    #[error("Program error: {0:?}")]
    Program(cairo_vm::types::errors::program_errors::ProgramError),
    #[error("{0}")]
    Custom(&'static str),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        Self::Base64(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error)
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

impl From<cairo_vm::types::errors::program_errors::ProgramError> for Error {
    fn from(
        error: cairo_vm::types::errors::program_errors::ProgramError,
    ) -> Self {
        Self::Program(error)
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
