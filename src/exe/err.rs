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
    #[error("program error: {0:?}")]
    Program(#[from] cairo_vm::types::errors::program_errors::ProgramError),
    #[error("sierra compilation error: {0:?}")]
    SierraCompilation(#[from] StarknetSierraCompilationError),
    #[error("runner error: {0}")]
    Runner(#[from] cairo_vm::vm::errors::runner_errors::RunnerError),
    #[error("cairo vm error: {0}")]
    VM(#[from] cairo_vm::vm::errors::vm_errors::VirtualMachineError),
    #[error("cairo vm memory error: {0}")]
    Mem(#[from] cairo_vm::vm::errors::memory_errors::MemoryError),
    #[error("cairo vm math error: {0}")]
    Math(#[from] cairo_vm::types::errors::math_errors::MathError),
    #[error("cairo pre execution error: {0}")]
    Pre(#[from] blockifier::execution::errors::PreExecutionError),
    #[error("cairo post execution error: {0}")]
    Post(#[from] blockifier::execution::errors::PostExecutionError),
    #[error("cairo run error: {0}")]
    Run(#[from] cairo_vm::vm::errors::cairo_run_errors::CairoRunError),
    #[error("trace error: {0}")]
    Trace(#[from] cairo_vm::vm::errors::trace_errors::TraceError),
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
