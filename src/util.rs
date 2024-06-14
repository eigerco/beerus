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

pub fn felt_from_bits(
    bits: &BitSlice<u8, Msb0>,
    mask: Option<usize>,
) -> Result<FieldElement> {
    if bits.len() != 251 {
        return Err(eyre!("expecting 251 bits"));
    }

    let mask = match mask {
        Some(x) => {
            if x > 251 {
                return Err(eyre!("Mask cannot be bigger than 251"));
            }
            x
        }
        None => 0,
    };

    let mut bytes = [0u8; 32];
    bytes.view_bits_mut::<Msb0>()[5 + mask..].copy_from_bitslice(&bits[mask..]);

    FieldElement::from_bytes_be(&bytes).map_err(|e| eyre!(format!("{e}")))
}

pub fn simple_call_opts(addr: Address, data: Bytes) -> CallOpts {
    CallOpts {
        from: None,
        to: Some(addr),
        gas: None,
        gas_price: None,
        value: None,
        data: Some(data),
    }
}

pub fn get_balance_key(addr: FieldElement) -> FieldElement {
    get_storage_var_address(ERC20_BALANCES_BASE, &[addr]).unwrap()
}

#[cfg(test)]
mod tests {
    use bitvec::{order::Msb0, slice::BitSlice};
    use starknet_crypto::FieldElement;

    use super::{felt_from_bits, felt_to_bits, get_balance_key};

    #[test]
    fn test_felt_to_bits_three() {
        let val = FieldElement::THREE;
        let mut slice = [0u8; 32];
        let bit_slice = BitSlice::<u8, Msb0>::from_slice_mut(&mut slice);
        bit_slice.set(249, true);
        bit_slice.set(250, true);
        assert_eq!(felt_to_bits(val), &bit_slice[..251]);
    }

    #[test]
    fn test_felt_to_bits_fourteen() {
        let val = FieldElement::from_dec_str("14").unwrap();
        let mut slice = [0u8; 32];
        let bit_slice = BitSlice::<u8, Msb0>::from_slice_mut(&mut slice);
        bit_slice.set(247, true);
        bit_slice.set(248, true);
        bit_slice.set(249, true);
        assert_eq!(felt_to_bits(val), &bit_slice[..251]);
    }

    #[test]
    fn test_felt_from_bits_one() {
        let mut slice = [0u8; 32];
        let bit_slice = BitSlice::<u8, Msb0>::from_slice_mut(&mut slice);
        bit_slice.set(250, true);
        assert_eq!(
            felt_from_bits(&bit_slice[..251], None).unwrap(),
            FieldElement::ONE
        );
    }

    #[test]
    fn test_felt_from_bits_seven() {
        let mut slice = [0u8; 32];
        let bit_slice = BitSlice::<u8, Msb0>::from_slice_mut(&mut slice);
        bit_slice.set(248, true);
        bit_slice.set(249, true);
        bit_slice.set(250, true);
        assert_eq!(
            felt_from_bits(&bit_slice[..251], None).unwrap(),
            FieldElement::from_dec_str("7").unwrap()
        );
    }

    #[test]
    fn test_felt_from_bits_mask() {
        let mut slice = [0u8; 32];
        let bit_slice = BitSlice::<u8, Msb0>::from_slice_mut(&mut slice);
        bit_slice.set(0, true);
        bit_slice.set(250, true);
        assert_eq!(
            felt_from_bits(&bit_slice[..251], None).unwrap(),
            FieldElement::from_dec_str(
                "1809251394333065553493296640760748560207343510400633813116524750123642650625"
            )
            .unwrap()
        );
        assert_eq!(
            felt_from_bits(&bit_slice[..251], Some(1)).unwrap(),
            FieldElement::from_dec_str("1").unwrap()
        );
    }

    #[test]
    fn test_felt_from_bits_wrong_mask_value() {
        let mut slice = [0u8; 32];
        let bit_slice = BitSlice::<u8, Msb0>::from_slice_mut(&mut slice);
        assert!(felt_from_bits(&bit_slice[..251], Some(252)).is_err());
    }

    #[test]
    fn test_get_balance_key() {
        assert_eq!(
            get_balance_key(FieldElement::ONE),
            FieldElement::from_dec_str(
                "3488041066649332616440110253331181934927363442882040970594983370166361489161"
            )
            .unwrap()
        );
    }
}
