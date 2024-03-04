#[derive(Debug)]
pub enum Error {
    IamGroot(iamgroot::jsonrpc::Error),
    StarknetApi(starknet_api::StarknetApiError),
    State(blockifier::state::errors::StateError),
    EntryPoint(blockifier::execution::errors::EntryPointExecutionError),
    Transaction(blockifier::transaction::errors::TransactionExecutionError),
    Custom(&'static str),
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
