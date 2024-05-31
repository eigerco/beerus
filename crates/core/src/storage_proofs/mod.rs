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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use starknet_crypto::FieldElement;

    use crate::{
        storage_proofs::{
            types::{ContractData, TrieNode},
            StorageProof,
        },
        utils::felt_to_bits,
    };

    #[test]
    fn valid_one_level_parse_proof() {
        let key = felt_to_bits(
            FieldElement::from_str(
                "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1",
            )
            .unwrap(),
        );
        let value = FieldElement::from_str(
            "0x000000000000000000000000000047616d65206f66204c69666520546f6b656e",
        )
        .unwrap();
        let edge_node_string = r#"[{
            "edge": {
                "child": "0x47616d65206f66204c69666520546f6b656e",
                "path": {
                    "len": 231,
                    "value": "0x3dfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                }
            }
        }]"#;
        let mut proof: Vec<TrieNode> =
            serde_json::from_str(edge_node_string).unwrap();

        let value = StorageProof::parse_proof(&key, value, &mut proof).unwrap();
        assert_eq!(
            value,
            FieldElement::from_hex_be(
                "0x1e224db31dfb3e1b8c95670a12f1903d4a32ac7bb83f4b209029e14155bbca9"
            )
            .unwrap()
        );
    }

    #[test]
    fn valid_five_level_parse_proof() {
        let key = felt_to_bits(
            FieldElement::from_str(
                "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1",
            )
            .unwrap(),
        );
        let value = FieldElement::from_str(
            "0x000000000000000000000000000047616d65206f66204c69666520546f6b656e",
        )
        .unwrap();
        let proof_string = r#"[
        {
            "binary": {
                "left": "0x46e82293b0564764a071f1aa4488aa7577b1b5bb2e898321f8536d5593d371d",
                "right": "0x58adcf6ea8b96992aa316e2f092f2480ca406c3630fe97573046a32900745b5"
            }
        },
        {
            "binary": {
                "left": "0x716e211c75f4c0e14dbe46c361812b0129abd061b63faf91ad5569bf22b785c",
                "right": "0x3729d9699d4410223e413f3b3aa91a043d94242f888188036e6ea25b6962041"
            }
        },
        {
            "edge": {
                "child": "0x6281e42b5941ae1a77ea03836aad1190097f72e1a1ed534fae2e00b4118f504",
                "path": {
                    "len": 1,
                    "value": "0x1"
                }
            }
        },
        {
            "binary": {
                "left": "0x3e3800516f62800ef6491b1cb1915b3353026ea6a6afcf35e8d4c54e35b04ea",
                "right": "0x1e224db31dfb3e1b8c95670a12f1903d4a32ac7bb83f4b209029e14155bbca9"
            }
        },
        {
            "edge": {
                "child": "0x47616d65206f66204c69666520546f6b656e",
                "path": {
                    "len": 231,
                    "value": "0x3dfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                }
            }
        }]"#;
        let mut proof: Vec<TrieNode> =
            serde_json::from_str(proof_string).unwrap();

        let value = StorageProof::parse_proof(&key, value, &mut proof).unwrap();
        assert_eq!(
            value,
            FieldElement::from_hex_be(
                "0x6cc50a732b4256f7b642348e19bd1a8bee7ac76bed3fcee3bc34309538c00c6"
            )
            .unwrap()
        );
    }

    #[test]
    fn invalid_one_level_parse_proof() {
        let key = felt_to_bits(FieldElement::default());
        let value = FieldElement::ZERO;
        let mut proof: Vec<TrieNode> = serde_json::from_str(
            r#"[{
            "edge": {
                "child": "0xfaa",
                "path": {
                    "len": 1,
                    "value": "0xbad"
                }
            }
        }]"#,
        )
        .unwrap();
        assert!(StorageProof::parse_proof(&key, value, &mut proof).is_none());
    }

    #[test]
    fn invalid_one_level_proof_last_key_byte_2_instead_of_1() {
        let key = felt_to_bits(
            FieldElement::from_str(
                "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be2",
            )
            .unwrap(),
        );
        let value = FieldElement::from_str(
            "0x000000000000000000000000000047616d65206f66204c69666520546f6b656e",
        )
        .unwrap();
        let edge_node_string = r#"[{
            "edge": {
                "child": "0x47616d65206f66204c69666520546f6b656e",
                "path": {
                    "len": 231,
                    "value": "0x3dfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                }
            }
        }]"#;
        let mut proof: Vec<TrieNode> =
            serde_json::from_str(edge_node_string).unwrap();
        assert!(StorageProof::parse_proof(&key, value, &mut proof).is_none());
    }

    #[test]
    fn invalid_five_level_proof_len_7_instead_of_1() {
        let key = felt_to_bits(
            FieldElement::from_str(
                "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1",
            )
            .unwrap(),
        );
        let value = FieldElement::from_str(
            "0x000000000000000000000000000047616d65206f66204c69666520546f6b656e",
        )
        .unwrap();
        let proof_string = r#"[
        {
            "binary": {
                "left": "0x46e82293b0564764a071f1aa4488aa7577b1b5bb2e898321f8536d5593d371d",
                "right": "0x58adcf6ea8b96992aa316e2f092f2480ca406c3630fe97573046a32900745b5"
            }
        },
        {
            "binary": {
                "left": "0x716e211c75f4c0e14dbe46c361812b0129abd061b63faf91ad5569bf22b785c",
                "right": "0x3729d9699d4410223e413f3b3aa91a043d94242f888188036e6ea25b6962041"
            }
        },
        {
            "edge": {
                "child": "0x6281e42b5941ae1a77ea03836aad1190097f72e1a1ed534fae2e00b4118f504",
                "path": {
                    "len": 7,
                    "value": "0x1"
                }
            }
        },
        {
            "binary": {
                "left": "0x3e3800516f62800ef6491b1cb1915b3353026ea6a6afcf35e8d4c54e35b04ea",
                "right": "0x1e224db31dfb3e1b8c95670a12f1903d4a32ac7bb83f4b209029e14155bbca9"
            }
        },
        {
            "edge": {
                "child": "0x47616d65206f66204c69666520546f6b656e",
                "path": {
                    "len": 231,
                    "value": "0x3dfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                }
            }
        }]"#;
        let mut proof: Vec<TrieNode> =
            serde_json::from_str(proof_string).unwrap();
        assert!(StorageProof::parse_proof(&key, value, &mut proof).is_none());
    }

    #[test]
    fn valid_one_level_verify_proof() {
        let mut storage_proof = StorageProof {
            contract_data: ContractData {
                storage_proofs: vec![serde_json::from_str(
                    r#"[{
                    "edge": {
                        "child": "0x47616d65206f66204c69666520546f6b656e",
                        "path": {
                            "len": 231,
                            "value": "0x3dfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                        }
                    }
                }]"#,
                )
                .unwrap()],
                root: FieldElement::from_hex_be(
                    "0x1e224db31dfb3e1b8c95670a12f1903d4a32ac7bb83f4b209029e14155bbca9",
                )
                .unwrap(),
                ..Default::default()
            },
            ..Default::default()
        };

        let key = felt_to_bits(
            FieldElement::from_str(
                "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1",
            )
            .unwrap(),
        );
        let value = FieldElement::from_str(
            "0x000000000000000000000000000047616d65206f66204c69666520546f6b656e",
        )
        .unwrap();

        assert!(storage_proof.verify_storage_proofs(&key, value).is_ok());
    }

    #[test]
    fn invalid_one_level_verify_proof() {
        let mut storage_proof = StorageProof {
            contract_data: ContractData {
                storage_proofs: vec![serde_json::from_str(
                    r#"[{
                    "edge": {
                        "child": "0xbad",
                        "path": {
                            "len": 231,
                            "value": "0xfaa"
                        }
                    }
                }]"#,
                )
                .unwrap()],
                root: FieldElement::from_dec_str("42").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        };

        let key = felt_to_bits(FieldElement::default());
        let value = FieldElement::ONE;

        assert!(storage_proof.verify_storage_proofs(&key, value).is_err());
    }

    #[test]
    fn contract_state_hash_is_valid() {
        let storage_proof = StorageProof {
            contract_data: ContractData {
                class_hash: FieldElement::ONE,
                root: FieldElement::TWO,
                nonce: FieldElement::THREE,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            storage_proof.calculate_contract_state_hash(),
            FieldElement::from_hex_be(
                "0x7fdeb85518534a06e6b50c2ccdea7bbf3d47c607a9b36fbf690c41274976950"
            )
            .unwrap()
        );
    }

    #[test]
    fn calculate_global_root_is_valid() {
        let storage_proof = StorageProof {
            class_commitment: FieldElement::ONE,
            ..Default::default()
        };
        assert_eq!(
            storage_proof.calculate_global_root(FieldElement::TWO),
            FieldElement::from_hex_be(
                "0x57e17712cba54d27d07c13c60861ccb7aa16fa4cfad71845d63a4448203953c"
            )
            .unwrap()
        );
    }

    #[test]
    fn valid_verify_contract_proof() {
        let mut storage_proof = StorageProof {
            contract_proof: serde_json::from_str(
                r#"[{
                "edge": {
                    "child": "0x538a7653ef22e217f93066ac54784c0159a5e1e37d808f83c82d1b42d57457d",
                    "path": {
                        "len": 229,
                        "value": "0x4a03bb9e744479e3298f54705a35966ab04140d3d8dd797c1f6dc49d0"
                    }
                }
            }]"#,
            )
            .unwrap(),
            state_commitment: FieldElement::from_hex_be(
                "0x1e2a7a7ee40c1d897c8c0a9515720ea02c8075ee9e00db277f5f8c3e4edcb54",
            )
            .unwrap(),
            contract_data: ContractData {
                class_hash: FieldElement::from_hex_be(
                    "0x4e635d495504b31ec191cbfc3d99b5d109bfcae4d0d9e16f4909a43b2e24c07",
                )
                .unwrap(),
                root: FieldElement::from_hex_be(
                    "0x5826149cbab3f8538d346301869ba2742a159d1542463ce19a60a927b826a2f",
                )
                .unwrap(),
                nonce: FieldElement::ZERO,
                ..Default::default()
            },
            ..Default::default()
        };
        let global_root = storage_proof.state_commitment;
        let contract_address = FieldElement::from_hex_be(
            "0x06a05844a03bb9e744479e3298f54705a35966ab04140d3d8dd797c1f6dc49d0",
        )
        .unwrap();
        assert!(storage_proof
            .verify_contract_proof(global_root, contract_address)
            .is_ok());
    }

    #[test]
    fn invalid_verify_contract_proof() {
        let mut invalid_storage_proof = StorageProof::default();
        let global_root = FieldElement::ZERO;
        let contract_address = FieldElement::ZERO;
        assert!(invalid_storage_proof
            .verify_contract_proof(global_root, contract_address)
            .is_err());
    }
}
