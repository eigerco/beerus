use serde_json::{json, Value};
use starknet::core::types::{
    BroadcastedInvokeTransaction, BroadcastedTransaction, CompressedLegacyContractClass, ContractClass, EmittedEvent,
    EventsPage, FieldElement, LegacyContractAbiEntry, LegacyContractEntryPoint, LegacyEntryPointsByType,
    LegacyStructAbiEntry, LegacyStructAbiType, LegacyStructMember, SyncStatus, SyncStatusType,
};

/// Helper to create an object for testing
/// # Returns
/// Tuple of a mock BroadcastedTransaction object and its equivalent JSON Value
pub fn create_mock_broadcasted_transaction() -> (BroadcastedTransaction, Value) {
    let mock_broadcasted_tx = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
        max_fee: FieldElement::ZERO,
        signature: vec![
            FieldElement::from_hex_be("156a781f12e8743bd07e20a4484154fd0baccee95d9ea791c121c916ad44ee0").unwrap(),
            FieldElement::from_hex_be("7228267473c670cbb86a644f8696973db978c51acde19431d3f1f8f100794c6").unwrap(),
        ],
        nonce: FieldElement::ZERO,
        sender_address: FieldElement::from_hex_be("5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0")
            .unwrap(),
        calldata: vec![
            FieldElement::from_hex_be("1").unwrap(),
            FieldElement::from_hex_be("7394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10").unwrap(),
            FieldElement::from_hex_be("2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354").unwrap(),
            FieldElement::from_hex_be("0").unwrap(),
            FieldElement::from_hex_be("3").unwrap(),
            FieldElement::from_hex_be("3").unwrap(),
            FieldElement::from_hex_be("5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0").unwrap(),
            FieldElement::from_hex_be("3635c9adc5dea00000").unwrap(),
            FieldElement::from_hex_be("0").unwrap(),
        ],
        is_query: true,
    });
    let mock_broadcasted_tx_json = json!({
        "type": "INVOKE",
        "max_fee": "0x0",
        "version": "0x1",
        "signature": [
            "0x156a781f12e8743bd07e20a4484154fd0baccee95d9ea791c121c916ad44ee0",
            "0x7228267473c670cbb86a644f8696973db978c51acde19431d3f1f8f100794c6"
        ],
        "nonce": "0x0",
        "sender_address": "0x5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0",
        "calldata": [
            "0x1",
            "0x7394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
            "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
            "0x0",
            "0x3",
            "0x3",
            "0x5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0",
            "0x3635c9adc5dea00000",
            "0x0"
        ],
        "is_query": "true",
    });
    (mock_broadcasted_tx, mock_broadcasted_tx_json)
}
