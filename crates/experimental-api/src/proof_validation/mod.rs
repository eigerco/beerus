use beerus_core::{
    storage_proofs::types::Direction,
    utils::{felt_from_bits, felt_to_bits},
};
use iamgroot::jsonrpc;
use starknet_crypto::{pedersen_hash, poseidon_hash_many, FieldElement};

use crate::gen::{
    Address, BinaryNode, BinaryNodeBinary, ContractData, EdgeNode,
    EdgeNodeEdge, Felt, GetProofResult, Node, StorageKey,
};

impl GetProofResult {
    pub fn verify(
        &mut self,
        global_root: Felt,
        contract_address: Address,
        key: StorageKey,
        value: Felt,
    ) -> Result<(), jsonrpc::Error> {
        let contract_data = self.contract_data.as_ref().ok_or(
            jsonrpc::Error::new(-32700, "No contract data found".to_string()),
        )?;
        self.verify_storage_proofs(contract_data, key, value)?;
        self.verify_contract_proof(contract_data, global_root, contract_address)
    }

    fn verify_storage_proofs(
        &self,
        contract_data: &ContractData,
        key: StorageKey,
        value: Felt,
    ) -> Result<(), jsonrpc::Error> {
        let root = &contract_data.root;
        let storage_proofs = &contract_data.storage_proofs.as_ref().ok_or(
            jsonrpc::Error::new(-32700, "No storage proof found".to_string()),
        )?[0];

        match Self::parse_proof(key.as_ref(), value, storage_proofs)? {
            Some(computed_root) => match computed_root == *root {
                true => Ok(()),
                false => Err(jsonrpc::Error::new(
                    -32700,
                    format!(
                        "Proof invalid:\nprovided-root -> {}\ncomputed-root -> {}\n",
                        root.as_ref(), computed_root.as_ref()
                    ),
                )),
            },
            None => Err(jsonrpc::Error::new(
                -32700,
                format!("Proof invalid for root -> {}\n", root.as_ref()),
            )),
        }
    }

    fn verify_contract_proof(
        &self,
        contract_data: &ContractData,
        global_root: Felt,
        contract_address: Address,
    ) -> Result<(), jsonrpc::Error> {
        let state_hash = Self::calculate_contract_state_hash(contract_data)?;

        match Self::parse_proof(
            contract_address.0.as_ref(),
            state_hash,
            &self.contract_proof,
        )? {
            Some(storage_commitment) => {
                let class_commitment = self.class_commitment.as_ref().ok_or(
                    jsonrpc::Error::new(
                        -32700,
                        "No class commitment".to_string(),
                    ),
                )?;
                let parsed_global_root = Self::calculate_global_root(
                    class_commitment,
                    storage_commitment,
                )
                .map_err(|_| {
                    jsonrpc::Error::new(
                        -32700,
                        "Failed to calculate global root".to_string(),
                    )
                })?;
                let state_commitment = self.state_commitment.as_ref().ok_or(
                    jsonrpc::Error::new(
                        -32700,
                        "No state commitment".to_string(),
                    ),
                )?;
                match *state_commitment == parsed_global_root && global_root == parsed_global_root {
                    true => Ok(()),
                    false => Err(jsonrpc::Error::new(
                        -32700,
                        format!("Proof invalid:\nstate commitment -> {}\nparsed global root -> {}\n global root -> {}", 
                        state_commitment.as_ref(), parsed_global_root.as_ref(), global_root.as_ref())
                    )),
                }
            }
            None => Err(jsonrpc::Error::new(
                -32700,
                format!(
                    "Could not parse global root for root: {}",
                    global_root.as_ref()
                ),
            )),
        }
    }

    fn calculate_contract_state_hash(
        contract_data: &ContractData,
    ) -> Result<Felt, jsonrpc::Error> {
        // The contract state hash is defined as H(H(H(hash, root), nonce), CONTRACT_STATE_HASH_VERSION)
        const CONTRACT_STATE_HASH_VERSION: FieldElement = FieldElement::ZERO;
        let hash = pedersen_hash(
            &FieldElement::from_hex_be(contract_data.class_hash.as_ref())
                .map_err(|_| {
                    jsonrpc::Error::new(
                        -32701,
                        "Failed to create Field Element".to_string(),
                    )
                })?,
            &FieldElement::from_hex_be(contract_data.root.as_ref()).map_err(
                |_| {
                    jsonrpc::Error::new(
                        -32701,
                        "Failed to create Field Element".to_string(),
                    )
                },
            )?,
        );
        let hash = pedersen_hash(
            &hash,
            &FieldElement::from_hex_be(contract_data.nonce.as_ref()).map_err(
                |_| {
                    jsonrpc::Error::new(
                        -32701,
                        "Failed to create Field Element".to_string(),
                    )
                },
            )?,
        );
        let hash = pedersen_hash(&hash, &CONTRACT_STATE_HASH_VERSION);
        Felt::try_new(&format!("0x{:x}", hash)).map_err(|_| {
            jsonrpc::Error::new(
                -32701,
                "Failed to create Field Element".to_string(),
            )
        })
    }

    fn calculate_global_root(
        class_commitment: &Felt,
        storage_commitment: Felt,
    ) -> Result<Felt, jsonrpc::Error> {
        let global_state_ver = FieldElement::from_byte_slice_be(
            b"STARKNET_STATE_V0",
        )
        .map_err(|_| {
            jsonrpc::Error::new(
                -32701,
                "Failed to create Field Element".to_string(),
            )
        })?;
        let hash = poseidon_hash_many(&[
            global_state_ver,
            FieldElement::from_hex_be(storage_commitment.as_ref()).map_err(
                |_| {
                    jsonrpc::Error::new(
                        -32701,
                        "Failed to create Field Element".to_string(),
                    )
                },
            )?,
            FieldElement::from_hex_be(class_commitment.as_ref()).map_err(
                |_| {
                    jsonrpc::Error::new(
                        -32701,
                        "Failed to create Field Element".to_string(),
                    )
                },
            )?,
        ]);
        Felt::try_new(&format!("0x{:x}", hash)).map_err(|_| {
            jsonrpc::Error::new(
                -32701,
                "Failed to create Field Element".to_string(),
            )
        })
    }

    fn parse_proof(
        key: impl Into<String>,
        value: Felt,
        proof: &[Node],
    ) -> Result<Option<Felt>, jsonrpc::Error> {
        let key = felt_to_bits(
            FieldElement::from_hex_be(&key.into()).map_err(|_| {
                jsonrpc::Error::new(
                    -32701,
                    "Failed to create Field Element".to_string(),
                )
            })?,
        );
        if key.len() != 251 {
            return Ok(None);
        }
        let value =
            FieldElement::from_hex_be(value.as_ref()).map_err(|_| {
                jsonrpc::Error::new(
                    -32701,
                    "Failed to create Field Element".to_string(),
                )
            })?;
        // initialized to the value so if the last node
        // in the proof is a binary node we can still verify
        let (mut hold, mut path_len) = (value, 0);
        // reverse the proof in order to hash from the leaf towards the root
        for (i, node) in proof.iter().rev().enumerate() {
            match node {
                Node::EdgeNode(EdgeNode {
                    edge: EdgeNodeEdge { child, path },
                }) => {
                    // calculate edge hash given by provider
                    let child_felt = FieldElement::from_hex_be(child.as_ref())
                        .map_err(|_| {
                            jsonrpc::Error::new(
                                -32701,
                                "Failed to create Field Element".to_string(),
                            )
                        })?;
                    let path_value =
                        FieldElement::from_hex_be(path.value.as_ref())
                            .map_err(|_| {
                                jsonrpc::Error::new(
                                    -32701,
                                    "Failed to create Field Element"
                                        .to_string(),
                                )
                            })?;
                    let provided_hash = pedersen_hash(&child_felt, &path_value)
                        + FieldElement::from(path.len as u64);
                    if i == 0 {
                        // mask storage key
                        let computed_hash = match felt_from_bits(
                            &key,
                            Some(251 - path.len as usize),
                        ) {
                            Ok(masked_key) => {
                                pedersen_hash(&value, &masked_key)
                                    + FieldElement::from(path.len as u64)
                            }
                            Err(_) => return Ok(None),
                        };
                        // verify computed hash against provided hash
                        if provided_hash != computed_hash {
                            return Ok(None);
                        };
                    }

                    // walk up the remaining path
                    path_len += path.len;
                    hold = provided_hash;
                }
                Node::BinaryNode(BinaryNode {
                    binary: BinaryNodeBinary { left, right },
                }) => {
                    path_len += 1;
                    let left = FieldElement::from_hex_be(left.as_ref())
                        .map_err(|_| {
                            jsonrpc::Error::new(
                                -32701,
                                "Failed to create Field Element".to_string(),
                            )
                        })?;
                    let right = FieldElement::from_hex_be(right.as_ref())
                        .map_err(|_| {
                            jsonrpc::Error::new(
                                -32701,
                                "Failed to create Field Element".to_string(),
                            )
                        })?;
                    // identify path direction for this node
                    let expected_hash =
                        match Direction::from(key[251 - path_len as usize]) {
                            Direction::Left => pedersen_hash(&hold, &right),
                            Direction::Right => pedersen_hash(&left, &hold),
                        };

                    hold = pedersen_hash(&left, &right);
                    // verify calculated hash vs provided hash for the node
                    if hold != expected_hash {
                        return Ok(None);
                    };
                }
            };
        }

        Ok(Some(Felt::try_new(&format!("0x{:x}", hold))?))
    }
}
#[cfg(test)]
mod tests {
    use crate::gen::{
        Address, ContractData, Felt, GetProofResult, Node, StorageKey,
    };

    #[test]
    fn valid_one_level_parse_proof() {
        let key = "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1".to_string();
        let value =
            Felt::try_new("0x47616d65206f66204c69666520546f6b656e").unwrap();
        let edge_node_string = r#"[{
            "edge": {
                "child": "0x47616d65206f66204c69666520546f6b656e",
                "path": {
                    "len": 231,
                    "value": "0x3dfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                }
            }
        }]"#;
        let proof: Vec<Node> = serde_json::from_str(edge_node_string).unwrap();
        let ret_val = GetProofResult::parse_proof(key, value, &proof).unwrap();

        assert!(ret_val.is_some());
        let ret_val = ret_val.unwrap();
        assert_eq!(
            ret_val.as_ref(),
            "0x1e224db31dfb3e1b8c95670a12f1903d4a32ac7bb83f4b209029e14155bbca9"
        );
    }

    #[test]
    fn valid_five_level_parse_proof() {
        let key = "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1".to_string();
        let value =
            Felt::try_new("0x47616d65206f66204c69666520546f6b656e").unwrap();
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
        let proof: Vec<Node> = serde_json::from_str(proof_string).unwrap();
        let ret_val = GetProofResult::parse_proof(key, value, &proof).unwrap();

        assert!(ret_val.is_some());
        let ret_val = ret_val.unwrap();
        assert_eq!(
            ret_val.as_ref(),
            "0x6cc50a732b4256f7b642348e19bd1a8bee7ac76bed3fcee3bc34309538c00c6"
        );
    }

    #[test]
    fn invalid_one_level_parse_proof() {
        let key = "0xabc".to_string();
        let value = Felt::try_new("0xdef").unwrap();
        let proof: Vec<Node> = serde_json::from_str(
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
        assert!(GetProofResult::parse_proof(key, value, &proof)
            .unwrap()
            .is_none());
    }

    #[test]
    fn invalid_one_level_proof_last_key_byte_2_instead_of_1() {
        let key = "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be2".to_string();
        let value =
            Felt::try_new("0x47616d65206f66204c69666520546f6b656e").unwrap();
        let edge_node_string = r#"[{
            "edge": {
                "child": "0x47616d65206f66204c69666520546f6b656e",
                "path": {
                    "len": 231,
                    "value": "0x3dfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                }
            }
        }]"#;
        let proof: Vec<Node> = serde_json::from_str(edge_node_string).unwrap();
        assert!(GetProofResult::parse_proof(key, value, &proof)
            .unwrap()
            .is_none());
    }

    #[test]
    fn invalid_five_level_proof_len_7_instead_of_1() {
        let key = "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1".to_string();
        let value =
            Felt::try_new("0x47616d65206f66204c69666520546f6b656e").unwrap();
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
        let proof: Vec<Node> = serde_json::from_str(proof_string).unwrap();
        assert!(GetProofResult::parse_proof(key, value, &proof)
            .unwrap()
            .is_none());
    }

    #[test]
    fn valid_one_level_verify_storage_proof() {
        let key = StorageKey::try_new(
            "0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1",
        ).unwrap();
        let value =
            Felt::try_new("0x47616d65206f66204c69666520546f6b656e").unwrap();
        let edge_node_string = r#"[{
            "edge": {
                "child": "0x47616d65206f66204c69666520546f6b656e",
                "path": {
                    "len": 231,
                    "value": "0x3dfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                }
            }
        }]"#;

        let storage_proof = GetProofResult {
            contract_data: Some(ContractData {
                root: Felt::try_new(
                    "0x1e224db31dfb3e1b8c95670a12f1903d4a32ac7bb83f4b209029e14155bbca9",
                )
                .unwrap(),
                storage_proofs: Some(vec![serde_json::from_str(edge_node_string).unwrap()]),
                class_hash: Felt::try_new("0x0").unwrap(),
                contract_state_hash_version: Felt::try_new("0x0").unwrap(),
                nonce: Felt::try_new("0x0").unwrap(),
            }),
            class_commitment: Some(Felt::try_new("0x0").unwrap()),
            contract_proof: vec![],
            state_commitment: Some(Felt::try_new("0x0").unwrap())
        };
        let contract_data = storage_proof.contract_data.as_ref().unwrap();

        assert!(storage_proof
            .verify_storage_proofs(contract_data, key, value)
            .is_ok());
    }

    #[test]
    fn invalid_one_level_verify_storage_proof() {
        let key = StorageKey::try_new("0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1").unwrap();
        let value = Felt::try_new("0xdef").unwrap();
        let edge_node_string = r#"[{
            "edge": {
                "child": "0xbad",
                "path": {
                    "len": 231,
                    "value": "0xfaa"
                }
            }
        }]"#;

        let storage_proof = GetProofResult {
            contract_data: Some(ContractData {
                root: Felt::try_new("0x42").unwrap(),
                storage_proofs: Some(vec![serde_json::from_str(
                    edge_node_string,
                )
                .unwrap()]),
                class_hash: Felt::try_new("0x0").unwrap(),
                contract_state_hash_version: Felt::try_new("0x0").unwrap(),
                nonce: Felt::try_new("0x0").unwrap(),
            }),
            class_commitment: Some(Felt::try_new("0x0").unwrap()),
            contract_proof: vec![],
            state_commitment: Some(Felt::try_new("0x0").unwrap()),
        };
        let contract_data = storage_proof.contract_data.as_ref().unwrap();

        assert!(storage_proof
            .verify_storage_proofs(contract_data, key, value)
            .is_err());
    }

    #[test]
    fn contract_state_hash_is_valid() {
        let contract_data = ContractData {
            class_hash: Felt::try_new("0x123").unwrap(),
            root: Felt::try_new("0xabc").unwrap(),
            nonce: Felt::try_new("0xdef").unwrap(),
            contract_state_hash_version: Felt::try_new("0x0").unwrap(),
            storage_proofs: Some(vec![vec![]]),
        };
        assert_eq!(
            GetProofResult::calculate_contract_state_hash(&contract_data).unwrap(),
            Felt::try_new("0x30a3c317f49a18c65bb5d22c87172f3f60101d54425457a66237474dd2d66db")
                .unwrap()
        );
    }

    #[test]
    fn calculate_global_root_is_valid() {
        assert_eq!(
            GetProofResult::calculate_global_root(
                &Felt::try_new("0xabc").unwrap(),
                Felt::try_new("0xdef").unwrap()
            ).unwrap(),
            Felt::try_new("0x42e26eb87a82c4b4130cb6bfbd33be7788436aa66f787ede4aef9456b58939")
                .unwrap()
        );
    }

    #[test]
    fn valid_verify_contract_proof() {
        let edge_node_string = r#"[{
            "edge": {
                "child": "0x538a7653ef22e217f93066ac54784c0159a5e1e37d808f83c82d1b42d57457d",
                "path": {
                    "len": 229,
                    "value": "0x4a03bb9e744479e3298f54705a35966ab04140d3d8dd797c1f6dc49d0"
                }
            }
        }]"#;
        let storage_proof = GetProofResult {
            contract_proof: serde_json::from_str(edge_node_string).unwrap(),
            state_commitment: Some(
                Felt::try_new("0x1e2a7a7ee40c1d897c8c0a9515720ea02c8075ee9e00db277f5f8c3e4edcb54")
                    .unwrap(),
            ),
            contract_data: Some(ContractData {
                class_hash: Felt::try_new(
                    "0x4e635d495504b31ec191cbfc3d99b5d109bfcae4d0d9e16f4909a43b2e24c07",
                )
                .unwrap(),
                root: Felt::try_new(
                    "0x5826149cbab3f8538d346301869ba2742a159d1542463ce19a60a927b826a2f",
                )
                .unwrap(),
                nonce: Felt::try_new("0x0").unwrap(),
                contract_state_hash_version: Felt::try_new("0x0").unwrap(),
                storage_proofs: Some(vec![vec![]])
            }),
            class_commitment: Some(Felt::try_new("0x0").unwrap()),
        };

        let global_root = Felt::try_new(
            "0x1e2a7a7ee40c1d897c8c0a9515720ea02c8075ee9e00db277f5f8c3e4edcb54",
        )
        .unwrap();
        let contract_address = Address(Felt::try_new("0x6a05844a03bb9e744479e3298f54705a35966ab04140d3d8dd797c1f6dc49d0")
                .unwrap());
        let contract_data = storage_proof.contract_data.as_ref().unwrap();
        assert!(storage_proof
            .verify_contract_proof(contract_data, global_root, contract_address)
            .is_ok());
    }

    #[test]
    fn invalid_verify_contract_proof() {
        let invalid_storage_proof = GetProofResult {
            state_commitment: Some(Felt::try_new("0x0").unwrap()),
            class_commitment: Some(Felt::try_new("0x0").unwrap()),
            contract_data: Some(ContractData {
                class_hash: Felt::try_new("0x0").unwrap(),
                contract_state_hash_version: Felt::try_new("0x0").unwrap(),
                nonce: Felt::try_new("0x0").unwrap(),
                root: Felt::try_new("0x0").unwrap(),
                storage_proofs: Some(vec![vec![]]),
            }),
            contract_proof: vec![],
        };
        let global_root = Felt::try_new("0x0").unwrap();
        let contract_address = Address(Felt::try_new("0x0").unwrap());
        let contract_data =
            invalid_storage_proof.contract_data.as_ref().unwrap();
        assert!(invalid_storage_proof
            .verify_contract_proof(contract_data, global_root, contract_address)
            .is_err());
    }
}
