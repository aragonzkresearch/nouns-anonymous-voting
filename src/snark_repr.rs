use std::ops::Deref;
use ark_ff::{BigInt, BigInteger, PrimeField};
use babyjubjub_ark::{Signature};
use toml::map::Values;
use toml::Value;
use crate::{BN254_Fr, BN254_G1, BBJJ_Fr, BBJJ_G1, VoteChoice};



type TargetType = BN254_Fr;


#[macro_export] macro_rules! concat_vec {
        ($($arr:expr),*) => {
            {
                let mut result = Vec::new();
                $(
                    result.extend($arr.iter());
                )*
                result
            }
        };
    }

pub struct Wrapper<T> (pub T);


impl Into<TargetType> for Wrapper<BBJJ_Fr> {
    fn into(self) -> TargetType {
        // As BN254_Fr == BBJJ_Fq > BBJJ_Fr the value fits in the new field
        BN254_Fr::new(self.0.0)
    }
}


impl Into<Vec<TargetType>> for Wrapper<Signature> {
    fn into(self) -> Vec<TargetType> {
        vec![self.0.r_b8.x, self.0.r_b8.y, Wrapper(self.0.s).into()]
    }
}

impl Into<Vec<TargetType>> for Wrapper<BN254_G1> {
    fn into(self) -> Vec<TargetType> {
        let x : Vec<TargetType> = Wrapper(self.0.x.0).into();
        let y : Vec<TargetType> = Wrapper(self.0.y.0).into();
        let z : Vec<TargetType> = Wrapper(self.0.z.0).into();
        concat_vec!(x, y, z)
    }
}

impl Into<Vec<TargetType>> for Wrapper<BigInt<4>> {
    fn into(self) -> Vec<TargetType> {
        // Split the BigInt into two BN254_Frs and return them as an array
        let mut bytes = self.0.to_bits_be();
        let mut x = &bytes[..128];
        let mut y = &bytes[128..];
        let x = BN254_Fr::new(BigInt::from_bits_be(x));
        let y = BN254_Fr::new(BigInt::from_bits_be(y));
        vec![x, y]
    }
}

impl Into<Vec<TargetType>> for Wrapper<BBJJ_G1> {
    fn into(self) -> Vec<TargetType> {
        vec![self.0.x, self.0.y]
    }
}

pub trait TomlSerialisable {
    fn toml(self) -> Value;
}

impl TomlSerialisable for TargetType {
    fn toml(self) -> Value {
        Value::String(format!("0x{}", hex::encode(self.into_bigint().to_bytes_be())))
    }
}

impl<T: TomlSerialisable + Copy> TomlSerialisable for Vec<T> {
    fn toml(self) -> Value {
        Value::Array(self.iter().map(|x| x.toml()).collect())
    }
}