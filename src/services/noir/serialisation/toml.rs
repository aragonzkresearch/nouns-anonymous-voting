use ark_ff::{BigInteger, PrimeField};
use babyjubjub_ark::Signature;
use ethers::types::StorageProof;
use toml::Value;

use crate::services::noir::VoteProverInput;
use crate::MAX_DEPTH;
use crate::MAX_NODE_LEN;
use crate::{BBJJ_Ec, BBJJ_Fr, BN254_Fr};

pub trait TomlSerializable {
    fn toml(self) -> Value;
}

impl TomlSerializable for VoteProverInput {
    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        map.insert("a".to_string(), self.a.toml());
        map.insert("b".to_string(), self.b.toml());
        map.insert("n".to_string(), self.n.toml());
        map.insert("h_id".to_string(), self.h_id.toml());
        map.insert("process_id".to_string(), self.process_id.toml());
        map.insert("contract_addr".to_string(), self.contract_addr.toml());
        map.insert("chain_id".to_string(), self.chain_id.toml());
        map.insert(
            "registry_account_state".to_string(),
            self.registry_account_state.toml(),
        );
        map.insert(
            "nft_account_state".to_string(),
            self.nft_account_state.toml(),
        );
        map.insert("tcls_pk".to_string(), self.tcls_pk.toml());

        map.insert("v".to_string(), self.v.toml());
        map.insert("signed_id".to_string(), self.signed_id.toml());
        map.insert("voter_address".to_string(), self.voter_address.toml());
        map.insert("signed_v".to_string(), self.signed_v.toml());
        map.insert("nft_id".to_string(), self.nft_id.toml());
        map.insert("k".to_string(), self.k.toml());
        map.insert("registered_pbk".to_string(), self.registered_pbk.toml());
        map.insert("registry_key_sp".to_string(), self.registry_key_sp.toml());
        map.insert(
            "nft_ownership_proof".to_string(),
            self.nft_ownership_proof.toml(),
        );
        Value::Table(map)
    }
}

impl TomlSerializable for StorageProof {
    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        let depth = self.proof.len();
        ////////
        // TODO: Move these checks
        // Make sure MAX_DEPTH has not been exceeded
        assert!(
            depth <= MAX_DEPTH,
            "The maximum possible proof depth ({}) has been exceeded!",
            MAX_DEPTH
        );
        map.insert("depth".to_string(), depth.toml());

        // Make sure path is valid
        self.proof.iter().for_each(|node| {
            assert!(node.len() <= MAX_NODE_LEN, "Invalid node!");
        });
        ////////

        let path = self
            .proof
            .into_iter()
            .map(|b| b.to_vec())
            .collect::<Vec<_>>();

        // Proof path needs to be an appropriately padded flat array.
        let padded_path = path
            .into_iter()
            .chain({
                let depth_excess = MAX_DEPTH - depth;
                // Append with empty nodes to fill up to depth MAX_DEPTH
                vec![vec![]; depth_excess]
            })
            .map(|mut v| {
                let node_len = v.len();
                let len_excess = MAX_NODE_LEN - node_len;
                // Then pad each node up to length MAX_NODE_LEN
                v.append(&mut vec![0; len_excess]);
                v
            })
            .flatten()
            .collect::<Vec<u8>>(); // And flatten.
        map.insert("proof".to_string(), padded_path.toml());

        let key: [u8; 32] = self.key.into();
        let value: [u8; 32] = self.value.into();

        map.insert("key".to_string(), key.to_vec().toml());
        map.insert("value".to_string(), value.to_vec().toml());

        Value::Table(map)
    }
}

impl TomlSerializable for bool {
    fn toml(self) -> Value {
        Value::String(format!("{}", if self { 1 } else { 0 }))
    }
}

impl TomlSerializable for u8 {
    fn toml(self) -> Value {
        Value::String(format!("0x{:02x}", self))
    }
}

impl TomlSerializable for usize {
    fn toml(self) -> Value {
        Value::String(format!("0x{:02x}", self))
    }
}

impl TomlSerializable for BN254_Fr {
    fn toml(self) -> Value {
        Value::String(format!(
            "0x{}",
            hex::encode(self.into_bigint().to_bytes_be())
        ))
    }
}

impl TomlSerializable for BBJJ_Fr {
    fn toml(self) -> Value {
        Value::String(format!(
            "0x{}",
            hex::encode(self.into_bigint().to_bytes_be())
        ))
    }
}

impl<T: TomlSerializable + Copy, const N: usize> TomlSerializable for [T; N] {
    fn toml(self) -> Value {
        Value::Array(self.iter().map(|x| x.toml()).collect())
    }
}

impl<T: TomlSerializable + Copy> TomlSerializable for Vec<T> {
    fn toml(self) -> Value {
        Value::Array(self.iter().map(|x| x.toml()).collect())
    }
}

impl TomlSerializable for Signature {
    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        map.insert("r".to_string(), self.r_b8.toml());
        map.insert("s".to_string(), self.s.toml());

        Value::Table(map)
    }
}

impl TomlSerializable for BBJJ_Ec {
    fn toml(self) -> Value {
        Value::Array(vec![self.x.toml(), self.y.toml()])
    }
}
