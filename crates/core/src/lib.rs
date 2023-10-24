pub mod client;
pub mod config;
pub mod storage_proofs;
pub mod utils;

use eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("could not fetch l1 val: {0}")]
    FetchL1Val(Report),
    #[error("storage proof error: {0}")]
    StorageProof(Report),
}
