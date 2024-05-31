pub mod types;
use bitvec::prelude::Msb0;
use bitvec::slice::BitSlice;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet::core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::{pedersen_hash, poseidon_hash_many, FieldElement};
use types::{BinaryNode, ContractData, Direction, EdgeNode, TrieNode};

use crate::utils::{felt_from_bits, felt_to_bits};

#[serde_as]
#[derive(Debug, PartialEq, Deserialize, Clone, Serialize, Default)]
pub struct StorageProof {
    /// The global state commitment for Starknet 0.11.0 blocks onwards, if absent the hash
    /// of the first node in the [contract_proof](GetProofOutput#contract_proof) is the global state
    /// commitment.
    #[serde_as(as = "UfeHex")]
    pub state_commitment: FieldElement,
    /// Required to verify that the hash of the class commitment and the root of the
    /// [contract_proof](GetProofOutput::contract_proof) matches the
    /// [state_commitment](Self#state_commitment). Present only for Starknet blocks 0.11.0 onwards.
    #[serde_as(as = "UfeHex")]
    pub class_commitment: FieldElement,

    /// Membership / Non-membership proof for the queried contract
    pub contract_proof: Vec<TrieNode>,

    /// Additional contract data if it exists.
    pub contract_data: ContractData,
}

impl StorageProof {
    pub fn verify(
        &mut self,
        global_root: FieldElement,
        contract_address: FieldElement,
        key: FieldElement,
        value: FieldElement,
    ) -> Result<FieldElement> {
        self.verify_storage_proofs(&felt_to_bits(key), value)?;
        self.verify_contract_proof(global_root, contract_address)
    }

    fn verify_storage_proofs(
        &mut self,
        key: &BitSlice<u8, Msb0>,
        value: FieldElement,
    ) -> Result<FieldElement> {
        let root = self.contract_data.root;
        let storage_proofs = &mut self.contract_data.storage_proofs[0];

        match Self::parse_proof(key, value, storage_proofs) {
            Some(computed_root) => match computed_root == root {
                true => Ok(computed_root),
                false => Err(eyre!(
                    "Proof invalid:\nprovided-root -> {:x}\ncomputed-root -> {:x}\n",
                    root, computed_root
                )),
            },
            None => Err(eyre!("Proof invalid for root {:x}", root)),
        }
    }

    fn verify_contract_proof(
        &mut self,
        global_root: FieldElement,
        contract_address: FieldElement,
    ) -> Result<FieldElement> {
        let state_hash = self.calculate_contract_state_hash();

        match Self::parse_proof(
            &felt_to_bits(contract_address),
            state_hash,
            &mut self.contract_proof,
        ) {
            Some(storage_commitment) => {
                let parsed_global_root =
                    self.calculate_global_root(storage_commitment);
                match self.state_commitment == parsed_global_root && global_root == parsed_global_root {
                    true => Ok(parsed_global_root),
                    false => Err(eyre!(
                        "Proof invalid:\nstate commitment -> {:x}\nparsed global root -> {:x}\n global root -> {:x}", 
                        self.state_commitment, parsed_global_root, global_root
                    )),
                }
            }
            None => Err(eyre!(
                "Could not parse global root for root: {:x}",
                global_root
            )),
        }
    }

    fn calculate_contract_state_hash(&self) -> FieldElement {
        // The contract state hash is defined as H(H(H(hash, root), nonce), CONTRACT_STATE_HASH_VERSION)
        const CONTRACT_STATE_HASH_VERSION: FieldElement = FieldElement::ZERO;
        let hash = pedersen_hash(
            &self.contract_data.class_hash,
            &self.contract_data.root,
        );
        let hash = pedersen_hash(&hash, &self.contract_data.nonce);
        pedersen_hash(&hash, &CONTRACT_STATE_HASH_VERSION)
    }

    fn calculate_global_root(
        &self,
        storage_commitment: FieldElement,
    ) -> FieldElement {
        let global_state_ver =
            FieldElement::from_byte_slice_be(b"STARKNET_STATE_V0").unwrap();
        poseidon_hash_many(&[
            global_state_ver,
            storage_commitment,
            self.class_commitment,
        ])
    }

    fn parse_proof(
        key: &BitSlice<u8, Msb0>,
        value: FieldElement,
        proof: &mut [TrieNode],
    ) -> Option<FieldElement> {
        if key.len() != 251 {
            return None;
        }

        // initialized to the value so if the last node
        // in the proof is a binary node we can still verify
        let (mut hold, mut path_len) = (value, 0);
        // reverse the proof in order to hash from the leaf towards the root
        for (i, node) in proof.iter().rev().enumerate() {
            match node {
                TrieNode::Edge(EdgeNode { child, path }) => {
                    // calculate edge hash given by provider
                    let provided_hash = pedersen_hash(child, &path.value)
                        + FieldElement::from(path.len as u64);
                    if i == 0 {
                        // mask storage key
                        let computed_hash =
                            match felt_from_bits(key, Some(251 - path.len)) {
                                Ok(masked_key) => {
                                    pedersen_hash(&value, &masked_key)
                                        + FieldElement::from(path.len as u64)
                                }
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
                TrieNode::Binary(BinaryNode { left, right }) => {
                    path_len += 1;
                    // identify path direction for this node
                    let expected_hash =
                        match Direction::from(key[251 - path_len]) {
                            Direction::Left => pedersen_hash(&hold, right),
                            Direction::Right => pedersen_hash(left, &hold),
                        };

                    hold = pedersen_hash(left, right);
                    // verify calculated hash vs provided hash for the node
                    if hold != expected_hash {
                        return None;
                    };
                }
            };
        }

        Some(hold)
    }
}
