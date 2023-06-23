use ark_ff::{BigInteger, PrimeField};
use ethers::abi::Address;
use ethers::core::k256::elliptic_curve::bigint::Encoding;
use ethers::core::k256::U256;
use ethers::prelude::U256 as EthersU256;

use crate::{BBJJ_Ec, BBJJ_Fr, BN254_Fr};

/// A wrapper type for defining traits for types that are not defined in this crate.
pub struct Wrapper<T>(pub T);

/// A macros that wraps a value in the `Wrapper`
/// It takes a value `v` and returns a `Wrapper(v)`
#[macro_export]
macro_rules! wrap {
    ($v:expr) => {
        Wrapper($v)
    };
}

/// A macros that wraps a value in the `Wrapper` and then converts it into the type `T`
#[macro_export]
macro_rules! wrap_into {
    ($v:expr) => {{
        let temp = $v;
        let temp = wrap!(temp);
        temp.clone().into()
    }};
}

impl<T: Clone> Clone for Wrapper<T> {
    fn clone(&self) -> Self {
        Wrapper(self.0.clone())
    }
}

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

impl From<Wrapper<BN254_Fr>> for U256 {
    fn from(value: Wrapper<BN254_Fr>) -> Self {
        // Convert the BN254_Fr to a U256.
        U256::from_be_slice(value.0 .0.to_bytes_be().as_slice())
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

impl From<Wrapper<BBJJ_Ec>> for [U256; 2] {
    fn from(value: Wrapper<BBJJ_Ec>) -> Self {
        let bbjj_pbk_x = U256::from_be_slice(value.0.x.0.to_bytes_be().as_slice());
        let bbjj_pbk_y = U256::from_be_slice(value.0.y.0.to_bytes_be().as_slice());
        [bbjj_pbk_x, bbjj_pbk_y]
    }
}

impl<const N: usize> From<Wrapper<[U256; N]>> for [EthersU256; N] {
    fn from(value: Wrapper<[U256; N]>) -> Self {
        let mut ethers_u256_array = [EthersU256::zero(); N];
        for i in 0..N {
            ethers_u256_array[i] = EthersU256::from_big_endian(value.0[i].to_be_bytes().as_slice());
        }
        ethers_u256_array
    }
}

impl From<Wrapper<U256>> for EthersU256 {
    fn from(value: Wrapper<U256>) -> Self {
        EthersU256::from_big_endian(value.0.to_be_bytes().as_slice())
    }
}

impl From<Wrapper<EthersU256>> for U256 {
    fn from(value: Wrapper<EthersU256>) -> Self {
        let mut bytes = [0u8; 32];
        value.0.to_big_endian(&mut bytes);

        U256::from_be_slice(bytes.as_slice())
    }
}
