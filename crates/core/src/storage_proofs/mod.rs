pub mod types;
use bitvec::prelude::Msb0;
use bitvec::slice::BitSlice;
use eyre::{eyre, Result};
use pathfinder_common::{felt_bytes, ClassHash, ContractNonce, ContractRoot};
use pathfinder_merkle_tree::contract_state::calculate_contract_state_hash;
use pathfinder_merkle_tree::merkle_node::Direction;
use serde::Deserialize;
use stark_hash::{stark_hash, Felt};
use types::{ContractData, TrieNode};

use crate::utils::{felt_from_bits, felt_to_bits};

const GLOBAL_STATE_VERSION: Felt = felt_bytes!(b"STARKNET_STATE_V0");

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct StorageProof {
    /// The global state commitment for Starknet 0.11.0 blocks onwards, if absent the hash
    /// of the first node in the [contract_proof](GetProofOutput#contract_proof) is the global state
    /// commitment.
    pub state_commitment: Felt,
    /// Required to verify that the hash of the class commitment and the root of the
    /// [contract_proof](GetProofOutput::contract_proof) matches the
    /// [state_commitment](Self#state_commitment). Present only for Starknet blocks 0.11.0 onwards.
    pub class_commitment: Felt,

    /// Membership / Non-membership proof for the queried contract
    pub contract_proof: Vec<TrieNode>,

    /// Additional contract data if it exists.
    pub contract_data: ContractData,
}

impl StorageProof {
    pub fn verify(&mut self, global_root: Felt, contract_address: Felt, key: Felt, value: Felt) -> Result<Felt> {
        verify_proof(self.contract_data.root, &felt_to_bits(key), value, &mut self.contract_data.storage_proofs[0])?;

        let state_hash = self.calculate_contract_state_hash();

        if let Some(storage_commitment) =
            parse_proof(&felt_to_bits(contract_address), state_hash, &mut self.contract_proof)
        {
            let parsed_global_root = self.calculate_global_root(storage_commitment);
            if self.state_commitment == parsed_global_root && global_root == parsed_global_root {
                return Ok(parsed_global_root);
            }
        }

        Err(eyre!("could not parse global root for root: {global_root}"))
    }

    pub fn calculate_contract_state_hash(&self) -> Felt {
        calculate_contract_state_hash(
            ClassHash(self.contract_data.class_hash),
            ContractRoot(self.contract_data.root),
            ContractNonce(self.contract_data.nonce),
        )
        .0
    }

    pub fn calculate_global_root(&self, storage_commitment: Felt) -> Felt {
        stark_poseidon::poseidon_hash_many(&[
            GLOBAL_STATE_VERSION.into(),
            storage_commitment.into(),
            self.class_commitment.into(),
        ])
        .into()
    }
}

pub fn verify_proof(root: Felt, key: &BitSlice<u8, Msb0>, value: Felt, proof: &mut [TrieNode]) -> Result<Felt> {
    if let Some(parsed_root) = parse_proof(key, value, proof) {
        return Ok(parsed_root);
    }

    Err(eyre!("proof invalid for root: {root}"))
}

pub fn parse_proof(key: &BitSlice<u8, Msb0>, value: Felt, proof: &mut [TrieNode]) -> Option<Felt> {
    if key.len() != 251 {
        return None;
    }

    // placeholders for the hash chain
    let (mut hold, mut path_len) = (Felt::from(0_u64), 0);
    // reverse the proof in order to hash from the key towards the root
    for (i, node) in proof.iter().rev().enumerate() {
        match node {
            TrieNode::Edge { child, path } => {
                // calculate edge hash given by provider
                let provided_hash = stark_hash(*child, path.value) + Felt::from_u64(path.len as u64);
                if i == 0 {
                    // mask storage key
                    let computed_hash = match felt_from_bits(key, Some(251 - path.len)) {
                        Ok(masked_key) => stark_hash(value, masked_key) + Felt::from_u64(path.len as u64),
                        Err(_) => return None,
                    };

                    // verify computed hash against provided hash
                    if provided_hash != computed_hash {
                        return None;
                    };
                }

                // walk up the remaining path
                path_len += path.len;
                hold = provided_hash;
            }
            TrieNode::Binary { left, right } => {
                path_len += 1;

                // identify path direction for this node
                let expected_hash = match Direction::from(key[251 - path_len]) {
                    Direction::Left => stark_hash(hold, *right),
                    Direction::Right => stark_hash(*left, hold),
                };

                hold = stark_hash(*left, *right);
                // verify calculated hash vs provided hash for the node
                if hold != expected_hash {
                    return None;
                };
            }
        };
    }

    Some(hold)
}
