use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("rpc call failed: {0:?}")]
    Rpc(#[from] iamgroot::jsonrpc::Error),
    #[error("missing env variable: {0:?}")]
    Var(#[from] std::env::VarError),
    #[error("execution failed: {0:?}")]
    Exe(#[from] beerus::exe::err::Error),
    #[error("serde failed: {0:?}")]
    Json(#[from] serde_json::Error),
    #[error("starknet api error: {0:?}")]
    Api(#[from] starknet_api::StarknetApiError),
}
