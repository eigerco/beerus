use bitvec::prelude::{BitSlice, BitVec, Msb0};
use bitvec::view::BitView;
use ethers::types::{Address, Bytes};
use eyre::{eyre, Result};
use helios::types::CallOpts;
use starknet::core::types::FieldElement;
use starknet::core::utils::get_storage_var_address;

const ERC20_BALANCES_BASE: &str = "ERC20_balances";

pub fn felt_to_bits(felt: FieldElement) -> BitVec<u8, Msb0> {
    felt.to_bytes_be().view_bits::<Msb0>()[5..].to_bitvec()
}

pub fn felt_from_bits(bits: &BitSlice<u8, Msb0>, mask: Option<usize>) -> Result<FieldElement> {
    if bits.len() != 251 {
        return Err(eyre!("expecting 251 bits"));
    }

    let mask = if let Some(x) = mask { x } else { 0 };

    let mut bytes = [0u8; 32];
    bytes.view_bits_mut::<Msb0>()[5 + mask..].copy_from_bitslice(&bits[mask..]);

    FieldElement::from_bytes_be(&bytes).map_err(|e| eyre!(format!("{e}")))
}

pub fn simple_call_opts(addr: Address, data: Bytes) -> CallOpts {
    CallOpts { from: None, to: Some(addr), gas: None, gas_price: None, value: None, data: Some(data) }
}

pub fn get_balance_key(addr: FieldElement) -> FieldElement {
    get_storage_var_address(ERC20_BALANCES_BASE, &[addr]).unwrap()
}
