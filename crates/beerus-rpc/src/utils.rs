/// Prints hex string of the given value.
/// Works only if the argument implements LowerHex trait.
#[macro_export]
macro_rules! hex_string {
    ($e:expr) => {
        format!("0x{:x}", $e)
    };
}
