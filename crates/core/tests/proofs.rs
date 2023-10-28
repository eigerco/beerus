use beerus_core::storage_proofs::StorageProof;
use rstest::{fixture, rstest};
use starknet_crypto::FieldElement;

const TESTING_STATE_ROOT: &str = "11d7289401f12bdbbfcf890cf531dd13e215d68fa700b82b08220dc75c24f54";
const TESTING_CONTRACT_ADDR: &str = "49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
const TESTING_STORAGE_KEY: &str = "d4daccb5bc077d40279ee559dc950ff0e5a7d1e139b3e3ab7e1b8dd8b997a7";
const TESTING_BALANCE: &str = "17e3b52ef2aa6a";

struct ProofData {
    root: FieldElement,
    addr: FieldElement,
    key: FieldElement,
    value: FieldElement,
}

#[fixture]
fn proof_data() -> ProofData {
    ProofData {
        root: FieldElement::from_hex_be(TESTING_STATE_ROOT).unwrap(),
        addr: FieldElement::from_hex_be(TESTING_CONTRACT_ADDR).unwrap(),
        key: FieldElement::from_hex_be(TESTING_STORAGE_KEY).unwrap(),
        value: FieldElement::from_hex_be(TESTING_BALANCE).unwrap(),
    }
}

#[rstest]
fn verify_valid_storage_proof(proof_data: ProofData) {
    let proof_raw = include_bytes!("common/data/proof.json");
    let mut proof: StorageProof = serde_json::from_reader(proof_raw.as_slice()).unwrap();

    let res = proof.verify(proof_data.root, proof_data.addr, proof_data.key, proof_data.value);
    assert!(res.is_ok());
}

#[rstest]
fn invalid_value_storage_proof(proof_data: ProofData) {
    let proof_raw = include_bytes!("common/data/proof.json");
    let mut proof: StorageProof = serde_json::from_reader(proof_raw.as_slice()).unwrap();

    let bad_value = proof_data.value + FieldElement::ONE;
    let res = proof.verify(proof_data.root, proof_data.addr, proof_data.key, bad_value);
    assert!(res.is_err());
}

#[rstest]
fn invalid_key_storage_proof(proof_data: ProofData) {
    let proof_raw = include_bytes!("common/data/proof.json");
    let mut proof: StorageProof = serde_json::from_reader(proof_raw.as_slice()).unwrap();

    let bad_key = proof_data.key + FieldElement::ONE;
    let res = proof.verify(proof_data.root, proof_data.addr, bad_key, proof_data.value);
    assert!(res.is_err());
}

#[rstest]
fn invalid_addr_storage_proof(proof_data: ProofData) {
    let proof_raw = include_bytes!("common/data/proof.json");
    let mut proof: StorageProof = serde_json::from_reader(proof_raw.as_slice()).unwrap();

    let bad_addr = proof_data.addr + FieldElement::ONE;
    let res = proof.verify(proof_data.root, bad_addr, proof_data.key, proof_data.value);
    assert!(res.is_err());
}

#[rstest]
fn invalid_root_storage_proof(proof_data: ProofData) {
    let proof_raw = include_bytes!("common/data/proof.json");
    let mut proof: StorageProof = serde_json::from_reader(proof_raw.as_slice()).unwrap();

    let bad_root = proof_data.root + FieldElement::ONE;
    let res = proof.verify(bad_root, proof_data.addr, proof_data.key, proof_data.value);
    assert!(res.is_err());
}
