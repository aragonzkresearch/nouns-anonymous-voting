use ark_ff::{BigInteger, PrimeField};
use ethers::abi::Address;
use ethers::core::k256::elliptic_curve::bigint::Encoding;
use ethers::core::k256::U256;

use crate::{BBJJ_Fr, BN254_Fr};

/// A wrapper type for defining traits for types that are not defined in this crate.
pub struct Wrapper<T>(pub T);

impl<T> From<T> for Wrapper<T> {
    fn from(value: T) -> Self {
        Wrapper(value)
    }
}

impl From<Wrapper<U256>> for BN254_Fr {
    fn from(value: Wrapper<U256>) -> Self {
        let size_in_bytes: [u8; 32] = BN254_Fr::MODULUS
            .to_bytes_be()
            .as_slice()
            .try_into()
            .unwrap();
        // Check that the value is less than the modulus, otherwise it will panic.
        assert!(
            value.0 < U256::from_be_bytes(size_in_bytes),
            "Value is greater than modulus of the BN254 curve."
        );
        // Convert the U256 to a BN254_Fr.
        Self::from_be_bytes_mod_order(&value.0.to_be_bytes())
    }
}

impl From<Wrapper<U256>> for [BN254_Fr; 2] {
    fn from(value: Wrapper<U256>) -> Self {
        // Convert the most significant 128 bits of the U256 to a BN254_Fr.
        let mut fr1 = BN254_Fr::from_be_bytes_mod_order(&value.0.to_be_bytes()[0..16]);
        // Convert the least significant 128 bits of the U256 to a BN254_Fr.
        let mut fr2 = BN254_Fr::from_be_bytes_mod_order(&value.0.to_be_bytes()[16..32]);
        // Return the BN254_Fr array.
        [fr1, fr2]
    }
}

impl From<Wrapper<Address>> for BN254_Fr {
    fn from(value: Wrapper<Address>) -> Self {
        // Convert the U160 to a BN254_Fr.
        Self::from_be_bytes_mod_order(&value.0.as_bytes())
    }
}

impl From<BBJJ_Fr> for Wrapper<BN254_Fr> {
    fn from(value: BBJJ_Fr) -> Self {
        Wrapper(BN254_Fr::from_be_bytes_mod_order(
            value.into_bigint().to_bytes_be().as_slice(),
        ))
    }
}
