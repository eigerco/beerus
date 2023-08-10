#![cfg(not(target_arch = "wasm32"))]

use core::str::FromStr;

mod tests {

    use super::*;
    use beerus_core::ethers_helper::{
        block_string_to_block_tag_type, block_tag_eq, encode_function_data, u256_to_bytes32_slice,
        u256_to_bytes32_type,
    };
    use ethers::{abi::Abi, types::Bytes};
    use helios::types::BlockTag;

    #[test]
    fn test_encode_function_data() {
        let abi: Abi = serde_json::from_str(
            r#"
        [
            {
                "inputs": [
                    {
                        "internalType": "uint256",
                        "name": "x",
                        "type": "uint256"
                    }
                ],
                "name": "foo",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]
        "#,
        )
        .unwrap();
        let args = 42u64;
        let function_name = "foo";
        let encoded = encode_function_data(args, abi, function_name).unwrap();
        assert_eq!(
            encoded,
            Bytes::from_str(
                "0x2fbebd38000000000000000000000000000000000000000000000000000000000000002a"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_u256_to_bytes32_slice() {
        let value = "0x4916eedd890707a351fde79168118d8a26f14c1de33ab0ecad3116d7bcba1a23".into();
        let bytes = u256_to_bytes32_slice(value);
        assert_eq!(
            bytes,
            [
                73, 22, 238, 221, 137, 7, 7, 163, 81, 253, 231, 145, 104, 17, 141, 138, 38, 241,
                76, 29, 227, 58, 176, 236, 173, 49, 22, 215, 188, 186, 26, 35
            ]
        );
    }

    #[test]
    fn test_u256_to_bytes32_type() {
        let value = "0x4916eedd890707a351fde79168118d8a26f14c1de33ab0ecad3116d7bcba1a23".into();
        let token = u256_to_bytes32_type(value);
        assert_eq!(
            token,
            ethers::abi::Token::FixedBytes(vec![
                73, 22, 238, 221, 137, 7, 7, 163, 81, 253, 231, 145, 104, 17, 141, 138, 38, 241,
                76, 29, 227, 58, 176, 236, 173, 49, 22, 215, 188, 186, 26, 35
            ])
        );
    }

    #[test]
    fn test_block_string_to_block_tag_type() {
        // Testing for Number type
        // Given
        let block = "123".to_string();

        // When
        let result = block_string_to_block_tag_type(&block).unwrap();

        // Then
        let expected_result = BlockTag::Number(123);
        let equal = block_tag_eq(&result, &expected_result);
        assert_eq!(equal, true);

        // Testing for Latest type
        // Given
        let block = "latest".to_string();

        // When
        let result = block_string_to_block_tag_type(&block).unwrap();

        // Then
        let expected_result = BlockTag::Latest;
        let equal = block_tag_eq(&result, &expected_result);
        assert_eq!(equal, true);

        // Testing for Finalized type
        // Given
        let block = "finalized".to_string();

        // When
        let result = block_string_to_block_tag_type(&block).unwrap();

        // Then
        let expected_result = BlockTag::Finalized;
        let equal = block_tag_eq(&result, &expected_result);
        assert_eq!(equal, true);
    }

    #[test]
    fn test_invalid_block_should_return_error() {
        // Testing for invalid type
        // Given
        let block = "0x123".to_string();

        // When
        let result = block_string_to_block_tag_type(&block);

        // Then
        match result {
            Err(e) => assert_eq!("Invalid BlockTag", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }
}
