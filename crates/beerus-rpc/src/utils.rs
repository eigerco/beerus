use ethers::types::U256;

/// Converts a u64 integer to it's hexadecimal string.
pub fn u64_to_hex_string(val: u64) -> String {
    format!("0x{val:x}")
}

/// Converts a u256 integer to it's hexadecimal string.
pub fn u256_to_hex_string(val: U256) -> String {
    format!("0x{val:x}")
}
