use beerus_core::lightclient::starknet::storage_proof::{
    felt_to_bits_be, verify_full_proof, GetProofOutput, Membership, ProofRequest,
};
use serde::Deserialize;
use starknet::core::{crypto::pedersen_hash, types::FieldElement};
use std::fs;

#[derive(Debug, Deserialize)]
struct JsonOutput {
    result: GetProofOutput,
}

#[test]
fn non_membership() {
    let path = "tests/data.json";
    let s = fs::read_to_string(path).unwrap();

    let state_root = FieldElement::from_hex_be(
        "0x47f25798a804800b657d4e1508776e3c3c70f0d7587d125a558208f88570aa7",
    )
    .unwrap();
    let j: JsonOutput = serde_json::from_str(&s).unwrap();
    let contract_proof = j.result.contract_proof;
    let contract_address = FieldElement::from_hex_be(
        "0x4d4e07157aeb54abeb64f5792145f2e8db1c83bda01a8f06e050be18cfb8153",
    )
    .unwrap();
    let contract_data = j.result.contract_data.unwrap();
    let class_hash = contract_data.class_hash;
    let contract_nonce = contract_data.nonce;
    let contract_root = contract_data.root;
    let version = contract_data.contract_state_hash_version;

    let a = pedersen_hash(&class_hash, &contract_root);
    let b = pedersen_hash(&a, &contract_nonce);
    let contract_value = pedersen_hash(&b, &version);

    let mut contract_key = contract_address.to_bits_le();
    contract_key.reverse();
    let contract_request = ProofRequest::new(
        state_root,
        &contract_key[contract_key.len() - 251..],
        contract_value,
        &contract_proof,
    );
    let key1 = felt_to_bits_be(FieldElement::ONE);
    let value1 = FieldElement::TWO;
    let req1 = ProofRequest::new(
        contract_root,
        &key1[key1.len() - 251..],
        value1,
        &contract_data.storage_proofs[0],
    );
    let storage_requests = [req1];
    let memberships = verify_full_proof(contract_request, &storage_requests);
    assert_eq!(memberships, Some(vec![Some(Membership::NonMember)]));
}
