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
