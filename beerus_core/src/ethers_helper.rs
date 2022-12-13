use ethers::{
    abi::{Abi, AbiError, Tokenize},
    types::Bytes,
};

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
