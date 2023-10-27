use serde::{Deserialize, Serialize};
use stark_hash::Felt;

use super::StorageProof;

/// Holds the data and proofs for a specific contract.
#[derive(Debug, PartialEq, Deserialize, Clone, Serialize)]
pub struct ContractData {
    /// Required to verify the contract state hash to contract root calculation.
    pub class_hash: Felt,

    /// Required to verify the contract state hash to contract root calculation.
    pub nonce: Felt,

    /// Root of the Contract state tree
    pub root: Felt,

    /// This is currently just a constant = 0, however it might change in the future.
    pub contract_state_hash_version: Felt,

    /// The proofs associated with the queried storage values
    pub storage_proofs: Vec<Vec<TrieNode>>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct StorageProofResponse {
    pub jsonrpc: String,
    pub result: Option<StorageProof>,
    pub error: Option<RPCError>,
    pub id: u64,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RPCError {
    pub code: i128,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Path {
    pub value: Felt,
    pub len: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrieNode {
    Binary { left: Felt, right: Felt },
    Edge { child: Felt, path: Path },
}
