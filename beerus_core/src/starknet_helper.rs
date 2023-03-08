use eyre::{eyre, Result};
use serde_json::{json, Value};
use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::models::{
    BlockId, BlockTag, BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV1,
    BroadcastedTransaction, ContractAbiEntry, ContractClass, ContractEntryPoint, EmittedEvent,
    EntryPointsByType, EventsPage, StructAbiEntry, StructAbiType, StructMember, SyncStatus,
    SyncStatusType,
};
// use std::str::FromStr;

// #[cfg(not(feature = "std"))]
// #[allow(unused_imports)]
// #[macro_use]
// extern crate alloc;

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

/// Helper converting block identifier string with corresponding type to a BlockId Type
/// # Arguments
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// # Returns
/// The block identifier as BlockId type.
/// # Errors
/// * If the block_id_type is not in ('hash', 'number', tag)
/// * If the block_id cannot be parsed or invalid
pub fn block_id_string_to_block_id_type(block_id_type: &str, block_id: &str) -> Result<BlockId> {
    match block_id_type.to_lowercase().as_str() {
        "hash" => Ok(BlockId::Hash(FieldElement::from_str(block_id)?)),
        "number" => Ok(BlockId::Number(block_id.parse::<u64>()?)),
        "tag" => match block_id.to_lowercase().as_str() {
            "pending" => Ok(BlockId::Tag(BlockTag::Pending)),
            "latest" => Ok(BlockId::Tag(BlockTag::Latest)),
            _ => return Err(eyre!("Invalid Tag")),
        },
        _ => return Err(eyre!("Invalid BlockId Type")),
    }
}

/// Helper to create a ContractClass object for testing
/// # Returns
/// Tuple of a mock ContractClass object and its equivalent JSON Value
pub fn create_mock_contract_class() -> (ContractClass, Value) {
    let mock_contract_class = ContractClass {
        program: vec![1, 2, 3],
        entry_points_by_type: EntryPointsByType {
            constructor: vec![ContractEntryPoint {
                offset: 123,
                selector: FieldElement::from_str("123").unwrap(),
            }],
            external: vec![ContractEntryPoint {
                offset: 456,
                selector: FieldElement::from_str("456").unwrap(),
            }],
            l1_handler: vec![ContractEntryPoint {
                offset: 789,
                selector: FieldElement::from_str("789").unwrap(),
            }],
        },
        abi: Some(vec![ContractAbiEntry::Struct(StructAbiEntry {
            r#type: StructAbiType::Struct,
            name: "Uint256".to_string(),
            size: 2,
            members: vec![
                StructMember {
                    name: "low".to_string(),
                    r#type: "felt".to_string(),
                    offset: 0,
                },
                StructMember {
                    name: "high".to_string(),
                    r#type: "felt".to_string(),
                    offset: 1,
                },
            ],
        })]),
    };
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
        ]
    });
    (mock_broadcasted_tx, mock_broadcasted_tx_json)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use starknet::core::types::FieldElement;
    use starknet::providers::jsonrpc::models::{BlockId, BlockTag};

    #[tokio::test]
    async fn test_block_id_string_to_block_id_type() {
        // Testing for hash type
        // Given
        let block_id_type = "hash".to_string();
        let block_id = "0x123".to_string();

        // When
        let result = super::block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();

        // Then
        let expected_result = BlockId::Hash(FieldElement::from_str("0x123").unwrap());
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            serde_json::to_string(&expected_result).unwrap()
        );

        // Testing for number type
        // Given
        let block_id_type = "number".to_string();
        let block_id = "123".to_string();

        // When
        let result = super::block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();

        // Then
        let expected_result = BlockId::Number(123);
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            serde_json::to_string(&expected_result).unwrap()
        );

        // Testing for tag type
        // Given
        let block_id_type = "tag".to_string();
        let block_id = "latest".to_string();

        // When
        let result = super::block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();

        // Then
        let expected_result = BlockId::Tag(BlockTag::Latest);
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            serde_json::to_string(&expected_result).unwrap()
        );
    }

    #[tokio::test]
    async fn test_invalid_block_id_or_type_should_return_error() {
        // Testing for invalid type
        // Given
        let block_id_type = "other".to_string();
        let block_id = "0x123".to_string();

        // When
        let result = super::block_id_string_to_block_id_type(&block_id_type, &block_id);

        // Then
        match result {
            Err(e) => assert_eq!("Invalid BlockId Type", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }

        // Testing for invalid tag
        // Given
        let block_id_type = "tag".to_string();
        let block_id = "other".to_string();

        // When
        let result = super::block_id_string_to_block_id_type(&block_id_type, &block_id);

        // Then
        match result {
            Err(e) => assert_eq!("Invalid Tag", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }

        // Testing for invalid block number
        // Given
        let block_id_type = "number".to_string();
        let block_id = "other".to_string();

        // When
        let result = super::block_id_string_to_block_id_type(&block_id_type, &block_id);

        // Then
        match result {
            Err(e) => assert_eq!("invalid digit found in string", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }

        // Testing for invalid block hash
        // Given
        let block_id_type = "hash".to_string();
        let block_id = "other".to_string();

        // When
        let result = super::block_id_string_to_block_id_type(&block_id_type, &block_id);

        // Then
        match result {
            Err(e) => assert_eq!("invalid character", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }
}
