use serde::de;
use serde::{Deserialize, Deserializer};
use starknet::core::crypto::pedersen_hash;
use starknet::core::types::FieldElement;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Path {
    #[serde(deserialize_with = "from_hex_deser")]
    value: FieldElement,
    len: u8,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Edge {
    pub path: Path,
    #[serde(deserialize_with = "from_hex_deser")]
    pub child: FieldElement,
}

/// Lightweight representation of [BinaryNode]. Only holds left and right hashes.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Binary {
    #[serde(deserialize_with = "from_hex_deser")]
    pub left: FieldElement,
    #[serde(deserialize_with = "from_hex_deser")]
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

#[derive(Debug, PartialEq, Eq, Deserialize)]
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

// TODO
fn from_hex_deser<'de, D>(deserializer: D) -> Result<FieldElement, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    FieldElement::from_hex_be(s).map_err(de::Error::custom)
}

/// Holds the data and proofs for a specific contract.
#[derive(Debug, Deserialize)]
pub struct ContractData {
    /// Required to verify the contract state hash to contract root calculation.
    #[serde(deserialize_with = "from_hex_deser")]
    pub class_hash: FieldElement,
    /// Required to verify the contract state hash to contract root calculation.
    #[serde(deserialize_with = "from_hex_deser")]
    pub nonce: FieldElement,

    /// Root of the Contract state tree
    #[serde(deserialize_with = "from_hex_deser")]
    pub root: FieldElement,

    /// This is currently just a constant = 0, however it might change in the future.
    #[serde(deserialize_with = "from_hex_deser")]
    pub contract_state_hash_version: FieldElement,

    /// The proofs associated with the queried storage values
    pub storage_proofs: Vec<Vec<ProofNode>>,
}

/// Holds the membership/non-membership of a contract and its associated contract contract if the contract exists.
#[derive(Debug, Deserialize)]
pub struct GetProofOutput {
    /// Membership / Non-membership proof for the queried contract
    pub contract_proof: Vec<ProofNode>,

    /// Additional contract data if it exists.
    pub contract_data: Option<ContractData>,
}

#[derive(Debug, PartialEq)]
pub enum Membership {
    Member,
    NonMember,
}

// TODO docs
fn path_matches(value: FieldElement, remaining_path: &[bool]) -> bool {
    let bits = felt_to_bits_be(value);
    &bits[bits.len() - remaining_path.len()..] == remaining_path
}

// TODO docs
pub fn felt_to_bits_be(value: FieldElement) -> [bool; 256] {
    let mut bits = value.to_bits_le();
    bits.reverse();
    bits
}

pub struct ProofRequest<'a> {
    root: FieldElement,
    key: &'a [bool],
    value: FieldElement,
    proof: &'a [ProofNode],
}

impl<'a> ProofRequest<'a> {
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

    pub fn verify(&self) -> Option<Membership> {
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

pub fn verify_full_proof(
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
