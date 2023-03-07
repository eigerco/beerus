use serde::{de, Serializer};
use serde::{Deserialize, Deserializer, Serialize};
use starknet::core::crypto::pedersen_hash;
use starknet::core::types::FieldElement;

// #[cfg(not(feature = "std"))]
// #[allow(unused_imports)]
// #[macro_use]
// extern crate alloc;

#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Path {
    #[serde(deserialize_with = "from_hex_deser", serialize_with = "to_hex_ser")]
    value: FieldElement,
    len: u8,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Edge {
    pub path: Path,
    #[serde(deserialize_with = "from_hex_deser", serialize_with = "to_hex_ser")]
    pub child: FieldElement,
}

/// Lightweight representation of [BinaryNode]. Only holds left and right hashes.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Binary {
    #[serde(deserialize_with = "from_hex_deser", serialize_with = "to_hex_ser")]
    pub left: FieldElement,
    #[serde(deserialize_with = "from_hex_deser", serialize_with = "to_hex_ser")]
    pub right: FieldElement,
}

impl Edge {
    fn hash(&self) -> FieldElement {
        let child_hash = self.child;

        // Length should be smaller than the maximum size of a stark hash.
        let length = FieldElement::from(self.path.len);

        pedersen_hash(&child_hash, &self.path.value) + length
    }
}

impl Binary {
    fn hash(&self) -> FieldElement {
        pedersen_hash(&self.left, &self.right)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProofNode {
    Binary(Binary),
    Edge(Edge),
}

impl ProofNode {
    fn hash(&self) -> FieldElement {
        match self {
            ProofNode::Binary(bin) => bin.hash(),
            ProofNode::Edge(edge) => edge.hash(),
        }
    }
}

/// Utility function to deserialize a FieldElement
fn from_hex_deser<'de, D>(deserializer: D) -> Result<FieldElement, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    FieldElement::from_hex_be(s).map_err(de::Error::custom)
}

fn to_hex_ser<S>(v: &FieldElement, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("0x{v:x}"))
}

/// Holds the data and proofs for a specific contract.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ContractData {
    /// Required to verify the contract state hash to contract root calculation.
    #[serde(deserialize_with = "from_hex_deser", serialize_with = "to_hex_ser")]
    pub class_hash: FieldElement,
    /// Required to verify the contract state hash to contract root calculation.
    #[serde(deserialize_with = "from_hex_deser", serialize_with = "to_hex_ser")]
    pub nonce: FieldElement,

    /// Root of the Contract state tree
    #[serde(deserialize_with = "from_hex_deser", serialize_with = "to_hex_ser")]
    pub root: FieldElement,

    /// This is currently just a constant = 0, however it might change in the future.
    #[serde(deserialize_with = "from_hex_deser", serialize_with = "to_hex_ser")]
    pub contract_state_hash_version: FieldElement,

    /// The proofs associated with the queried storage values
    pub storage_proofs: Vec<Vec<ProofNode>>,
}

/// Holds the membership/non-membership of a contract and its associated contract contract if the contract exists.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct GetProofOutput {
    /// Membership / Non-membership proof for the queried contract
    pub contract_proof: Vec<ProofNode>,

    /// Additional contract data if it exists.
    pub contract_data: Option<ContractData>,
}

impl GetProofOutput {
    /// Verifies a full outpout (i.e, the contract proof and the storage proofs).
    /// The user is expected to provide `state_root`, `contract_address`, `storage_keys` and `storage_values`.
    /// Those values should come from a DIFFERENT source or else the proof verification would serve no purpose.
    pub fn verify(
        &self,
        state_root: FieldElement,
        contract_address: FieldElement,
        storage_keys: &[FieldElement],
        storage_values: &[FieldElement],
    ) -> Option<Vec<Option<Membership>>> {
        let contract_data = self.contract_data.clone()?;
        let class_hash = contract_data.class_hash;
        let contract_nonce = contract_data.nonce;
        let contract_root = contract_data.root;
        let version = contract_data.contract_state_hash_version;

        // Compute the contract state hash
        let a = pedersen_hash(&class_hash, &contract_root);
        let b = pedersen_hash(&a, &contract_nonce);
        let contract_state_hash = pedersen_hash(&b, &version);

        let contract_key = felt_to_bits_be(contract_address);
        let contract_request = ProofRequest::new(
            state_root,
            &contract_key[contract_key.len() - 251..],
            contract_state_hash,
            &self.contract_proof,
        );

        // Ensure the three arrays are of the same length
        let equal_lengths = [
            storage_keys.len(),
            storage_values.len(),
            contract_data.storage_proofs.len(),
        ]
        .windows(2)
        .all(|w| w[0] == w[1]);
        if !equal_lengths {
            return None;
        }

        // Allocate a vector of the correct size
        let mut storage_requests = Vec::with_capacity(storage_keys.len());

        // Map the keys to a bit representation
        let keys: Vec<[bool; 256]> = storage_keys.iter().map(|k| felt_to_bits_be(*k)).collect();

        // Create an array of corresponding proof requests
        for (i, _) in storage_keys.iter().enumerate() {
            let req = ProofRequest::new(
                contract_root,
                &keys[i][keys[i].len() - 251..],
                storage_values[i],
                &contract_data.storage_proofs[i],
            );
            storage_requests.push(req)
        }

        // Verify the proofs
        verify_proof_requests(contract_request, &storage_requests)
    }
}

#[derive(Debug, PartialEq)]
pub enum Membership {
    Member,
    NonMember,
}

/// Verifies that `value` and `remaining_path` share the same bits
fn path_matches(value: FieldElement, remaining_path: &[bool]) -> bool {
    let bits = felt_to_bits_be(value);
    &bits[bits.len() - remaining_path.len()..] == remaining_path
}

/// Utility function to convert a [`FieldElement`] to a big endian bit representation
fn felt_to_bits_be(value: FieldElement) -> [bool; 256] {
    let mut bits = value.to_bits_le();
    bits.reverse();
    bits
}

/// A request for a proof (can be a storage proof or a contract proof, they both share the same structure).
pub struct ProofRequest<'a> {
    root: FieldElement,
    key: &'a [bool],
    value: FieldElement,
    proof: &'a [ProofNode],
}

impl<'a> ProofRequest<'a> {
    // Creates a new proof request
    pub fn new(
        root: FieldElement,
        key: &'a [bool],
        value: FieldElement,
        proof: &'a [ProofNode],
    ) -> Self {
        Self {
            root,
            key,
            value,
            proof,
        }
    }

    /// Verifies the proof request. Returns `None` if there's a hash mismatch or
    /// if the key is too small; else returns a `Membership` variant.
    fn verify(&self) -> Option<Membership> {
        // Protect from ill-formed keys
        if self.key.len() < 251 {
            return None;
        }

        let mut expected_hash = self.root;
        let mut remaining_path = self.key;

        for proof_node in self.proof.iter() {
            // Hash mismatch? Return None.
            if proof_node.hash() != expected_hash {
                return None;
            }
            match proof_node {
                ProofNode::Binary(bin) => {
                    // Direction will always correspond to the 0th index
                    // because we're removing bits on every iteration.
                    let direction = remaining_path[0];

                    // Set the next hash to be the left or right hash,
                    // depending on the direction
                    expected_hash = match direction {
                        false => bin.left,
                        true => bin.right,
                    };

                    // Advance by a single bit
                    remaining_path = &remaining_path[1..];
                }
                ProofNode::Edge(edge) => {
                    let path_matches =
                        path_matches(edge.path.value, &remaining_path[..edge.path.len as usize]);
                    if !path_matches {
                        // If paths don't match, we've found a proof of non membership because we:
                        // 1. Correctly moved towards the target insofar as is possible, and
                        // 2. hashing all the nodes along the path does result in the root hash, which means
                        // 3. the target definitely does not exist in this tree
                        return Some(Membership::NonMember);
                    }

                    // Set the next hash to the child's hash
                    expected_hash = edge.child;

                    // Advance by the whole edge path
                    remaining_path = &remaining_path[edge.path.len as usize..];
                }
            }
        }

        // At this point, we should reach `value` !
        if expected_hash == self.value {
            Some(Membership::Member)
        } else {
            // Hash mismatch. Return `None`.
            None
        }
    }
}

/// Verifies the contract and storage proof requests.
fn verify_proof_requests(
    contract_request: ProofRequest,
    storage_requests: &[ProofRequest],
) -> Option<Vec<Option<Membership>>> {
    // Verify the contract proof
    let contract_verified = contract_request.verify()?;

    // Return None if it's invalid
    if contract_verified == Membership::NonMember {
        return None;
    }

    // Verify Storage Proofs
    let mut res = vec![];
    for request in storage_requests {
        res.push(request.verify());
    }
    Some(res)
}
