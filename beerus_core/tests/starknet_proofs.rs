use beerus_core::lightclient::starknet::storage_proof::{GetProofOutput, Membership};
use serde::Deserialize;
use starknet::core::types::FieldElement;
use std::fs;

#[derive(Debug, Deserialize)]
struct JsonOutput {
    result: GetProofOutput,
}

#[test]
fn non_membership() {
    let path = "tests/data/data.json";
    let s = fs::read_to_string(path).unwrap();

    let state_root = FieldElement::from_hex_be(
        "0x47f25798a804800b657d4e1508776e3c3c70f0d7587d125a558208f88570aa7",
    )
    .unwrap();
    let j: JsonOutput = serde_json::from_str(&s).unwrap();
    let output = j.result;

    let storage_keys = [FieldElement::ONE];
    let storage_values = [FieldElement::TWO];
    let contract_address = FieldElement::from_hex_be(
        "0x4d4e07157aeb54abeb64f5792145f2e8db1c83bda01a8f06e050be18cfb8153",
    )
    .unwrap();
    let memberships = output.verify(state_root, contract_address, &storage_keys, &storage_values);

    assert_eq!(memberships, Some(vec![Some(Membership::NonMember)]));
}

#[test]
fn invalid_contract_address() {
    // Read in the data.json file
    let path = "tests/data/data.json";
    let s = fs::read_to_string(path).unwrap();

    // Deserialize the file into a GetProofOutput struct
    let j: JsonOutput = serde_json::from_str(&s).unwrap();
    let output = j.result;

    // Set the state root to a different value than what is stored in the contract_proof list
    let state_root = FieldElement::from_hex_be(
        "0x56f25798a804800b657d4e1508776e3c3c70f0d7587d125a558208f88550aa7",
    )
    .unwrap();
    let storage_keys = [FieldElement::ONE];
    let storage_values = [FieldElement::TWO];
    let contract_address = FieldElement::from_hex_be(
        "0x4d4e07157aeb54abeb64f5792145f2e8db1c83bda01a8f06e050be18cfb8153",
    )
    .unwrap();

    // Verify the contract membership and storage values
    let memberships = output.verify(state_root, contract_address, &storage_keys, &storage_values);

    // The verify function should return None, since the contract proof is invalid
    assert_eq!(Option::None, memberships);
}

#[test]
fn storage_key_and_storage_values_different_array_lengths() {
    let path = "tests/data/data.json";
    let s = fs::read_to_string(path).unwrap();

    // Deserialize the JSON file into a GetProofOutput struct
    let j: JsonOutput = serde_json::from_str(&s).unwrap();
    let output = j.result;

    let contract_address = FieldElement::from_hex_be(
        "0x4d4e07157aeb54abeb64f5792145f2e8db1c83bda01a8f06e050be18cfb8153",
    )
    .unwrap();
    let state_root = FieldElement::from_hex_be(
        "0x47f25798a804800b657d4e1508776e3c3c70f0d7587d125a558208f88570aa7",
    )
    .unwrap();

    // The storage_keys and storage_values arrays are of different lengths, this should return an Err variant
    let storage_keys = [FieldElement::ONE];
    let storage_values = [FieldElement::TWO, FieldElement::THREE];
    let memberships = output.verify(state_root, contract_address, &storage_keys, &storage_values);
    assert_eq!(Option::None, memberships);
}

#[test]
fn contract_proof_incorrect_number_of_elements() {
    let path = "tests/data/data.json";
    let s = fs::read_to_string(path).unwrap();

    // Deserialize the JSON file into a GetProofOutput struct
    let j: JsonOutput = serde_json::from_str(&s).unwrap();
    let mut output = j.result;

    let contract_address = FieldElement::from_hex_be(
        "0x4d4e07157aeb54abeb64f5792145f2e8db1c83bda01a8f06e050be18cfb8153",
    )
    .unwrap();
    let state_root = FieldElement::from_hex_be(
        "0x47f25798a804800b657d4e1508776e3c3c70f0d7587d125a558208f88570aa7",
    )
    .unwrap();

    // Modify the contract_proof list to have an incorrect number of elements
    output.contract_proof = output.contract_proof[..4].to_vec();

    let storage_keys = [FieldElement::ONE];
    let storage_values = [FieldElement::TWO];
    let memberships = output.verify(state_root, contract_address, &storage_keys, &storage_values);
    // The contract_proof list has an incorrect number of elements, this should return None
    assert_eq!(memberships, None);
}
