use core::str::FromStr;

use ethabi::Uint as U256;

use ethers::{
    abi::{Abi, AbiError, Token, Tokenize},
    types::{Address, Bytes},
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

/// Parses an ethereum hash
/// # Arguments
/// * `hash` - The &str to convert.
/// # Returns
/// The hash as a Vec<u8>
pub fn parse_eth_hash(hash: &str) -> Result<Vec<u8>> {
    let stripped = hash.strip_prefix("0x").unwrap_or(hash);
    Ok(hex::decode(stripped)?)
}

/// Parses an ethereum address
/// # Arguments
/// * `address` - The &str to convert.
/// # Returns
/// The address as an Address
pub fn parse_eth_address(address: &str) -> Result<Address> {
    Ok(Address::from_str(address)?)
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
            Err(_) => Err(eyre!("Invalid BlockTag")),
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
