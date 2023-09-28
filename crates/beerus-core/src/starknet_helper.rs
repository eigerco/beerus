use serde_json::{json, Value};
use starknet::core::types::{
    BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV1, BroadcastedTransaction,
    CompressedLegacyContractClass, ContractClass, EmittedEvent, EventsPage, FieldElement,
    LegacyContractAbiEntry, LegacyContractEntryPoint, LegacyEntryPointsByType,
    LegacyStructAbiEntry, LegacyStructAbiType, LegacyStructMember, SyncStatus, SyncStatusType,
};

#[cfg(feature = "std")]
use std::vec;

#[cfg(not(feature = "std"))]
use alloc::vec;

#[cfg(feature = "std")]
use std::str::FromStr;

#[cfg(not(feature = "std"))]
use alloc::str::FromStr;

#[cfg(default = "std")]
use std::string::ToString;

#[cfg(not(feature = "std"))]
use alloc::string::ToString;

/// Helper to create a ContractClass object for testing
/// # Returns
/// Tuple of a mock ContractClass object and its equivalent JSON Value
pub fn create_mock_contract_class() -> (ContractClass, Value) {
    let mock_contract_class = ContractClass::Legacy(CompressedLegacyContractClass {
        program: vec![1, 2, 3],
        entry_points_by_type: LegacyEntryPointsByType {
            constructor: vec![LegacyContractEntryPoint {
                offset: 123,
                selector: FieldElement::from_str("123").unwrap(),
            }],
            external: vec![LegacyContractEntryPoint {
                offset: 456,
                selector: FieldElement::from_str("456").unwrap(),
            }],
            l1_handler: vec![LegacyContractEntryPoint {
                offset: 789,
                selector: FieldElement::from_str("789").unwrap(),
            }],
        },
        abi: Some(vec![LegacyContractAbiEntry::Struct(LegacyStructAbiEntry {
            r#type: LegacyStructAbiType::Struct,
            name: "Uint256".to_string(),
            size: 2,
            members: vec![
                LegacyStructMember {
                    name: "low".to_string(),
                    r#type: "felt".to_string(),
                    offset: 0,
                },
                LegacyStructMember {
                    name: "high".to_string(),
                    r#type: "felt".to_string(),
                    offset: 1,
                },
            ],
        })]),
    });
    let mock_contract_class_json = json!({
        "program": "AQID", // base64 encoding of [1, 2 ,3]
        "entry_points_by_type": {
          "CONSTRUCTOR": [
            {
              "offset": "0x7b",
              "selector": "0x7b"
            }
          ],
          "EXTERNAL": [
            {
              "offset": "0x1c8",
              "selector": "0x1c8"
            }
          ],
          "L1_HANDLER": [
            {
              "offset": "0x315",
              "selector": "0x315"
            }
          ]
        },
        "abi": [
          {
            "members": [
              {
                "name": "low",
                "offset": 0,
                "type": "felt"
              },
              {
                "name": "high",
                "offset": 1,
                "type": "felt"
              }
            ],
            "name": "Uint256",
            "size": 2,
            "type": "struct"
          }
        ]
      }
    );
    (mock_contract_class, mock_contract_class_json)
}

/// Helper to create a EventsPage object for testing
/// # Returns
/// Tuple of a mock EventsPage object and its equivalent JSON Value
pub fn create_mock_get_events() -> (EventsPage, Value) {
    let mock_get_events = EventsPage {
        continuation_token: Some("6".to_string()),
        events: vec![EmittedEvent {
            from_address: FieldElement::from_str(
                "0x47cfd9582fc4c7543d55d6853e8edee02ff72e233b4b2d4d42568ed4a68f9c0",
            )
            .unwrap(),
            keys: vec![FieldElement::from_str(
                "0xa46e8cb36cba031930583bca557e67f6b89b525640d324bc2208cc04b8ca8e",
            )
            .unwrap()],
            data: vec![
                FieldElement::from_str(
                    "0x2c03d22f43898f146e026a72f4cf37b9e898b70a11c4731665e0d75ce87700d",
                )
                .unwrap(),
                FieldElement::from_str("0x61e7b068").unwrap(),
            ],
            block_hash: FieldElement::from_str(
                "0x796ca96ef3c55c6e124f313c9252122248af6e754d31cd47579e0a9e5328409",
            )
            .unwrap(),
            block_number: 47538,
            transaction_hash: FieldElement::from_str(
                "0x76f1260a26ed41a350a432395c73043489cde7db85b8b16897e7a734aca5f14",
            )
            .unwrap(),
        }],
    };
    let mock_get_events_json = json!({
        "continuation_token": "6",
        "events": [{
            "block_hash": "0x796ca96ef3c55c6e124f313c9252122248af6e754d31cd47579e0a9e5328409",
            "block_number": 47538,
            "data": [
                "0x2c03d22f43898f146e026a72f4cf37b9e898b70a11c4731665e0d75ce87700d",
                "0x61e7b068"
            ],
            "from_address": "0x47cfd9582fc4c7543d55d6853e8edee02ff72e233b4b2d4d42568ed4a68f9c0",
            "keys": [
                "0xa46e8cb36cba031930583bca557e67f6b89b525640d324bc2208cc04b8ca8e"
                ],
                "transaction_hash": "0x76f1260a26ed41a350a432395c73043489cde7db85b8b16897e7a734aca5f14"
        }]
    });
    (mock_get_events, mock_get_events_json)
}

/// Helper to create a  object for testing
/// # Returns
/// Tuple of a mock  object and its equivalent JSON Value
pub fn create_mock_syncing_case_syncing() -> (SyncStatusType, Value, Value) {
    let mock_syncing = SyncStatusType::Syncing(SyncStatus {
        starting_block_hash: FieldElement::from_str("123").unwrap(),
        starting_block_num: 123,
        current_block_hash: FieldElement::from_str("456").unwrap(),
        current_block_num: 456,
        highest_block_hash: FieldElement::from_str("789").unwrap(),
        highest_block_num: 789,
    });
    let mock_syncing_json = json!({
        "status": "Syncing",
        "data": {
            "starting_block_hash": "0x7b",
            "starting_block_num": "0x7b",
            "current_block_hash": "0x1c8",
            "current_block_num": "0x1c8",
            "highest_block_hash": "0x315",
            "highest_block_num": "0x315"
        }
    });
    let mock_syncing_data_json = json!({
        "starting_block_hash": "0x7b",
        "starting_block_num": "0x7b",
        "current_block_hash": "0x1c8",
        "current_block_num": "0x1c8",
        "highest_block_hash": "0x315",
        "highest_block_num": "0x315"
    });
    (mock_syncing, mock_syncing_json, mock_syncing_data_json)
}

/// Helper to create a  object for testing
/// # Returns
/// Tuple of a mock  object and its equivalent JSON Value
pub fn create_mock_syncing_case_not_syncing() -> (SyncStatusType, Value) {
    let mock_syncing = SyncStatusType::NotSyncing;
    let mock_syncing_json = json!({
        "status": "NotSyncing",
        "data": null
    });
    (mock_syncing, mock_syncing_json)
}

/// Helper to create an object for testing
/// # Returns
/// Tuple of a mock BroadcastedTransaction object and its equivalent JSON Value
pub fn create_mock_broadcasted_transaction() -> (BroadcastedTransaction, Value) {
    let mock_broadcasted_tx = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
        BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::ZERO,
            signature: vec![
                FieldElement::from_hex_be(
                    "156a781f12e8743bd07e20a4484154fd0baccee95d9ea791c121c916ad44ee0",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "7228267473c670cbb86a644f8696973db978c51acde19431d3f1f8f100794c6",
                )
                .unwrap(),
            ],
            nonce: FieldElement::ZERO,
            sender_address: FieldElement::from_hex_be(
                "5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0",
            )
            .unwrap(),
            calldata: vec![
                FieldElement::from_hex_be("1").unwrap(),
                FieldElement::from_hex_be(
                    "7394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
                )
                .unwrap(),
                FieldElement::from_hex_be("0").unwrap(),
                FieldElement::from_hex_be("3").unwrap(),
                FieldElement::from_hex_be("3").unwrap(),
                FieldElement::from_hex_be(
                    "5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0",
                )
                .unwrap(),
                FieldElement::from_hex_be("3635c9adc5dea00000").unwrap(),
                FieldElement::from_hex_be("0").unwrap(),
            ],
            is_query: true,
        },
    ));
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
