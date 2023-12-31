use serde::{Deserialize, Serialize};
// use starknet_crypto::FieldElement;
use starknet::core::types::FieldElement;

use super::StorageProof;

/// Holds the data and proofs for a specific contract.
#[derive(Debug, PartialEq, Deserialize, Clone, Serialize)]
pub struct ContractData {
    /// Required to verify the contract state hash to contract root calculation.
    pub class_hash: FieldElement,

    /// Required to verify the contract state hash to contract root calculation.
    pub nonce: FieldElement,

    /// Root of the Contract state tree
    pub root: FieldElement,

    /// This is currently just a constant = 0, however it might change in the future.
    pub contract_state_hash_version: FieldElement,

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
    pub value: FieldElement,
    pub len: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrieNode {
    Binary { left: FieldElement, right: FieldElement },
    Edge { child: FieldElement, path: Path },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl From<bool> for Direction {
    fn from(tf: bool) -> Self {
        match tf {
            true => Direction::Right,
            false => Direction::Left,
        }
    }
}

impl From<Direction> for bool {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Left => false,
            Direction::Right => true,
        }
    }
}
