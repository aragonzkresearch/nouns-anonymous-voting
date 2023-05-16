use ark_ff::{BigInteger, PrimeField};
use babyjubjub_ark::Signature;
use toml::Value;
use toml::value::Array;
use crate::{BN254_Fr, BBJJ_Fr, BN254_G1, BBJJ_G1, concat_vec};
use crate::election::{ElectionIdentifier, VoteChoice};
use crate::serialisation::Wrapper;
use crate::preprover::{PrivateInput, PublicInput, StorageProof, VoteProverPackage};
use crate::MAX_NODE_LEN;
use crate::MAX_DEPTH;

pub trait TomlSerializable {
    fn toml(self) -> Value;
}

impl TomlSerializable for StorageProof {
    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        let depth = self.depth;
        map.insert("depth".to_string(),depth.toml());

        let path = self.path;

        // Proof path needs to be an appropriately padded flat array.
        let padded_path = path.into_iter().chain({
            let depth_excess = MAX_DEPTH - depth;
            // Append with empty nodes to fill up to depth MAX_DEPTH
            vec![vec![]; depth_excess]
        }).map(
            |mut v| {
                let node_len = v.len();
                let len_excess = MAX_NODE_LEN - node_len;
                // Then pad each node up to length MAX_NODE_LEN
                v.append(&mut vec![0; len_excess]);
                v
            }
        ).flatten().collect::<Vec<u8>>(); // And flatten.
        map.insert("path".to_string(), padded_path.toml());
        
        Value::Table(map)
    }
}

impl TomlSerializable for u8 {
    fn toml(self) -> Value {
        Value::String(format!("0x{:x}", self))
    }
}

impl TomlSerializable for usize {
    fn toml(self) -> Value {
        Value::String(format!("0x{:x}", self))
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
        map.insert("a".to_string(), <Wrapper<BN254_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.A_i)).toml());
        map.insert("b".to_string(), self.B_i.toml());
        map.insert("nullifier".to_string(), self.N_i.toml());
        map.insert("id_hash".to_string(), self.H_id.toml());
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
        map.insert("v".to_string(), <VoteChoice as Into<BN254_Fr>>::into(self.v_i).toml());
        map.insert("sigma".to_string(), <Wrapper<Signature> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.SIGMA_i)).toml());
        map.insert("tau".to_string(), <Wrapper<Signature> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.TAU_i)).toml());
        map.insert("id".to_string(), self.id.toml());
        map.insert("rck".to_string(), <Wrapper<BBJJ_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.RCK_i)).toml());

        for (name,data) in [("p1", self.p_1.toml()), ("p2", self.p_2.toml()), ("p3", self.p_3.toml())]
        {            
            map.insert(name.to_string(), data);
        }
        
        Value::Table(map)
    }
}

