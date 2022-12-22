use eyre::{eyre, Result};
use serde_json::{json, Value};
use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::models::{
    BlockId, BlockTag, ContractAbiEntry, ContractClass, ContractEntryPoint, EntryPointsByType,
    StructAbiEntry, StructAbiType, StructMember,
};
use std::str::FromStr;

/// Helper converting block identifier string with corresponding type to a BlockId Type
/// # Arguments
/// * `block_id_type` - The arguments to encode.
/// * `block_id` - The ABI of the contract.
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
/// A mock ContractClass object equivalent to MOCK_CONTRACT_CLASS_STRING
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
