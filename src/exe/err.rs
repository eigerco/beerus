use cairo_lang_starknet_classes::casm_contract_class::StarknetSierraCompilationError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("io error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("base64 error: {0:?}")]
    Base64(#[from] base64::DecodeError),
    #[error("serde error: {0:?}")]
    Serde(#[from] serde_json::Error),
    #[error("reqwest error: {0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("codegen error: {0:?}")]
    IamGroot(#[from] iamgroot::jsonrpc::Error),
    #[error("starknet api error: {0:?}")]
    StarknetApi(#[from] starknet_api::StarknetApiError),
    #[error("blockifier state error: {0:?}")]
    State(#[from] blockifier::state::errors::StateError),
    #[error("blockifier entry point error: {0:?}")]
    EntryPoint(#[from] blockifier::execution::errors::EntryPointExecutionError),
    #[error("blockifier transaction error: {0:?}")]
    Transaction(
        #[from] blockifier::transaction::errors::TransactionExecutionError,
    ),
    #[error("sierra compilation error: {0:?}")]
    SierraCompilation(#[from] StarknetSierraCompilationError),
    #[error("program error: {0}")]
    Program(String),
    #[error("{0}")]
    Custom(&'static str),
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
            e => iamgroot::jsonrpc::Error { code: 500, message: e.to_string() },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_to_iamgroot_jsonrpc_error() {
        let error = Error::Custom("test");
        let iamgroot_jsonrpc_error: iamgroot::jsonrpc::Error = error.into();
        assert_eq!(iamgroot_jsonrpc_error, iamgroot::jsonrpc::Error {code: 500, message: "test".to_string()});


        let error = Error::IamGroot(iamgroot::jsonrpc::Error {code: 500, message: "test".to_string()});
        let iamgroot_jsonrpc_error: iamgroot::jsonrpc::Error = error.into();

        assert_eq!(
            iamgroot::jsonrpc::Error {code: 500, message: "test".to_string()},
            iamgroot_jsonrpc_error,
        );
   }
   #[test]
   fn test_conversion_to_blockifier_state_errors_state_error() {
        let error = Error::Custom("test");
        let blockifier_error: blockifier::state::errors::StateError = error.into();

        assert_eq!(
            blockifier_error.to_string(),
            blockifier::state::errors::StateError::StateReadError("Custom(\"test\")".into()).to_string()
        );
   }
}
