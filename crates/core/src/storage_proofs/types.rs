use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet::core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::FieldElement;

use super::StorageProof;

/// Holds the data and proofs for a specific contract.
#[serde_as]
#[derive(Debug, PartialEq, Deserialize, Clone, Serialize)]
pub struct ContractData {
    /// Required to verify the contract state hash to contract root calculation.
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,

    /// Required to verify the contract state hash to contract root calculation.
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,

    /// Root of the Contract state tree
    #[serde_as(as = "UfeHex")]
    pub root: FieldElement,

    /// This is currently just a constant = 0, however it might change in the future.
    #[serde_as(as = "UfeHex")]
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

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Path {
    #[serde_as(as = "UfeHex")]
    pub value: FieldElement,
    pub len: usize,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct BinaryNode {
    #[serde_as(as = "UfeHex")]
    pub left: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub right: FieldElement,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct EdgeNode {
    #[serde_as(as = "UfeHex")]
    pub child: FieldElement,
    pub path: Path,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrieNode {
    Binary(BinaryNode),
    Edge(EdgeNode),
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
