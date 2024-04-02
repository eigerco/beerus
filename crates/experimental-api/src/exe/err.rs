use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("Base64 error: {0:?}")]
    Base64(#[from] base64::DecodeError),
    #[error("Serde error: {0:?}")]
    Serde(#[from] serde_json::Error),
    #[error("Reqwest error: {0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Codegen error: {0:?}")]
    IamGroot(iamgroot::jsonrpc::Error),
    #[error("Starknet API error: {0:?}")]
    StarknetApi(#[from] starknet_api::StarknetApiError),
    #[error("Blockifier state error: {0:?}")]
    State(#[from] blockifier::state::errors::StateError),
    #[error("Blockifier entry point error: {0:?}")]
    EntryPoint(#[from] blockifier::execution::errors::EntryPointExecutionError),
    #[error("Blockifier transaction error: {0:?}")]
    Transaction(
        #[from] blockifier::transaction::errors::TransactionExecutionError,
    ),
    #[error("Program error: {0:?}")]
    Program(#[from] cairo_vm::types::errors::program_errors::ProgramError),
    #[error("{0}")]
    Custom(&'static str),
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

impl From<Error> for iamgroot::jsonrpc::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::IamGroot(e) => e,
            e => iamgroot::jsonrpc::Error {
                code: 500,
                message: e.to_string(),
            }    
        }
    }
}
