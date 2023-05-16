use ark_ff::{BigInteger, PrimeField};
use babyjubjub_ark::Signature;
use toml::Value;
use toml::value::Array;
use crate::{BN254_Fr, BBJJ_Fr, BN254_G1, BBJJ_G1, concat_vec};
use crate::election::{ElectionIdentifier, VoteChoice};
use crate::serialisation::Wrapper;
use crate::preprover::{PrivateInput, PublicInput, StorageProofPLACEHOLDER, VoteProverPackage};

pub trait TomlSerializable {
    fn toml(self) -> Value;
}

impl TomlSerializable for StorageProofPLACEHOLDER {
    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        Value::Table(map)
    }
}

impl TomlSerializable for BN254_Fr {
    fn toml(self) -> Value {
        Value::String(format!("0x{}", hex::encode(self.into_bigint().to_bytes_be())))
    }
}

impl<T: TomlSerializable + Copy> TomlSerializable for Vec<T> {
    fn toml(self) -> Value {
        Value::Array(self.iter().map(|x| x.toml()).collect())
    }
}

impl TomlSerializable for ElectionIdentifier {
    fn toml(self) -> Value {
        let mut array = Array::new();
        array.push(self.chain_id.toml());
        array.push(self.process_id.toml());
        array.push(self.contract_addr.toml());
        Value::Array(array)
    }
}

impl TomlSerializable for PublicInput {

    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        map.insert("A_i".to_string(), <Wrapper<BN254_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.A)).toml());
        map.insert("B_i".to_string(), self.B.toml());
        map.insert("N_i".to_string(), self.N.toml());
        map.insert("H_id".to_string(), self.H_id.toml());
        Value::Table(map)
    }
}

impl TomlSerializable for VoteProverPackage {

    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        map.insert("public_input".to_string(), self.public_input.toml());
        map.insert("private_input".to_string(), self.private_input.toml());
        Value::Table(map)
    }
}


impl TomlSerializable for PrivateInput {

    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        map.insert("v_i".to_string(), <VoteChoice as Into<BN254_Fr>>::into(self.v).toml());
        map.insert("SIGMA_i".to_string(), <Wrapper<Signature> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.SIGMA)).toml());
        map.insert("TAU_i".to_string(), <Wrapper<Signature> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.TAU)).toml());
        map.insert("id".to_string(), self.id.toml());
        map.insert("RCK_i".to_string(), <Wrapper<BBJJ_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.RCK)).toml());
        map.insert("p_1".to_string(), <StorageProofPLACEHOLDER as Into<Vec<BN254_Fr>>>::into(self.p_1).toml());
        map.insert("p_2".to_string(), <StorageProofPLACEHOLDER as Into<Vec<BN254_Fr>>>::into(self.p_2).toml());
        map.insert("p_3".to_string(), <StorageProofPLACEHOLDER as Into<Vec<BN254_Fr>>>::into(self.p_3).toml());
        Value::Table(map)
    }
}

