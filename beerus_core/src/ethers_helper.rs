use ethers::types::U256;
use ethers::{
    abi::{Abi, AbiError, Token, Tokenize},
    types::Bytes,
};
use eyre::{eyre, Result};
use helios::types::BlockTag;

/// Helper for ABI encoding arguments for a specific function in a contract.
/// # Arguments
/// * `args` - The arguments to encode.
/// * `abi` - The ABI of the contract.
/// * `function_name` - The name of the function to encode the arguments for.
/// # Returns
/// The encoded arguments.
/// # Errors
/// * If the ABI is invalid.
pub fn encode_function_data<T: Tokenize>(
    args: T,
    abi: Abi,
    function_name: &str,
) -> Result<Bytes, AbiError> {
    let function = abi.function(function_name)?;
    let tokens = args.into_tokens();
    Ok(function.encode_input(&tokens).map(Into::into)?)
}

/// Convert a U256 to a slice of 32 bytes.
/// # Arguments
/// * `value` - The U256 to convert.
/// # Returns
/// The slice of 32 bytes.
pub fn u256_to_bytes32_slice(value: U256) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    value.to_big_endian(&mut bytes);
    bytes
}

/// Convert a U256 to a bytes32 `Token` ethers-rs type.
/// # Arguments
/// * `value` - The U256 to convert.
/// # Returns
/// The bytes32 `Token` ethers-rs type.
pub fn u256_to_bytes32_type(value: U256) -> Token {
    Token::FixedBytes(u256_to_bytes32_slice(value).to_vec())
}

/// Helper converting block identifier string with corresponding type to a BlockTag Type
/// # Arguments
/// * `block` - The block identifier.
/// # Returns
/// The block identifier as BlockTag type.
/// # Errors
/// * If the block cannot be parsed or invalid
pub fn block_string_to_block_tag_type(block: &str) -> Result<BlockTag> {
    match block.to_lowercase().as_str() {
        "finalized" => Ok(BlockTag::Finalized),
        "latest" => Ok(BlockTag::Latest),
        _ => match block.parse::<u64>() {
            Ok(number) => Ok(BlockTag::Number(number)),
            Err(_) => return Err(eyre!("Invalid BlockTag")),
        },
    }
}

/// Checks equality between two BlockTag inputs
/// # Arguments
/// * `a` - The first block identifier.
/// * `b` - The first block identifier.
/// # Returns
/// bool - True or False
pub fn block_tag_eq(a: &BlockTag, b: &BlockTag) -> bool {
    match (a, b) {
        (BlockTag::Latest, BlockTag::Latest) => true,
        (BlockTag::Finalized, BlockTag::Finalized) => true,
        (BlockTag::Number(a), BlockTag::Number(b)) => a == b,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::block_tag_eq;
    use ethers::types::Bytes;
    use helios::types::BlockTag;
    use std::str::FromStr;

    #[test]
    fn test_encode_function_data() {
        let abi: ethers::abi::Abi = serde_json::from_str(
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
        let encoded = super::encode_function_data(args, abi, function_name).unwrap();
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
        let bytes = super::u256_to_bytes32_slice(value);
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
        let token = super::u256_to_bytes32_type(value);
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
        let result = super::block_string_to_block_tag_type(&block).unwrap();

        // Then
        let expected_result = BlockTag::Number(123);
        let equal = block_tag_eq(&result, &expected_result);
        assert_eq!(equal, true);

        // Testing for Latest type
        // Given
        let block = "latest".to_string();

        // When
        let result = super::block_string_to_block_tag_type(&block).unwrap();

        // Then
        let expected_result = BlockTag::Latest;
        let equal = block_tag_eq(&result, &expected_result);
        assert_eq!(equal, true);

        // Testing for Finalized type
        // Given
        let block = "finalized".to_string();

        // When
        let result = super::block_string_to_block_tag_type(&block).unwrap();

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
        let result = super::block_string_to_block_tag_type(&block);

        // Then
        match result {
            Err(e) => assert_eq!("Invalid BlockTag", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }
}
