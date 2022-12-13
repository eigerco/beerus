use ethers::{
    abi::{Abi, AbiError, Token, Tokenize},
    types::Bytes,
};
use primitive_types::U256;

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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ethers::types::Bytes;

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
}
