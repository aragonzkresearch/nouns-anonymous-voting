use ark_ff::{BigInteger, PrimeField};
use babyjubjub_ark::Signature;
use ethers::types::{Address, H256, StorageProof};
use toml::Value;

use crate::noir::{TallyProverInput, VoteProverInput, MAX_DEPTH, MAX_NODE_LEN};
use crate::{utils::VoteChoice, BBJJ_Ec, BBJJ_Fr, BN254_Fr};

pub trait TomlSerializable {
    fn toml(self) -> Value;
}

impl TomlSerializable for TallyProverInput {
    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();

        // Need to include the number of voters in output
        let num_voters = self.v.len();

        // Figure out whether num_voters is <= 16, 256, 512, 1024, 2048
        let padded_len: usize = [16, 256, 512, 1024, 2048]
            .into_iter()
            .filter(|x| x >= &num_voters)
            .next()
            .expect("Error: There are too many voters.");

        let pad_vec = |v: Vec<BN254_Fr>| {
            v.into_iter()
                .chain(std::iter::repeat(BN254_Fr::from(0)).take(padded_len - num_voters))
                .collect::<Vec<_>>()
        };

        map.insert("num_voters".to_string(), self.v.len().toml());
        map.insert("b_k".to_string(), self.b_k.toml());
        map.insert("process_id".to_string(), self.process_id.toml());
        map.insert("contract_addr".to_string(), self.contract_addr.toml());
        map.insert("chain_id".to_string(), self.chain_id.toml());
        map.insert("vote_count".to_string(), self.vote_count.toml());
        map.insert(
            "k_x".to_string(),
            pad_vec(self.k.iter().map(|p| p.x).collect::<Vec<_>>()).toml(),
        );
        map.insert(
            "k_y".to_string(),
            pad_vec(self.k.iter().map(|p| p.y).collect::<Vec<_>>()).toml(),
        );
        map.insert(
            "v".to_string(),
            pad_vec(
                self.v
                    .into_iter()
                    .map(|v| BN254_Fr::from(v as u8))
                    .collect::<Vec<_>>(),
            )
            .toml(),
        );
        Value::Table(map)
    }
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
        map.insert("tlcs_pk".to_string(), self.tlcs_pk.toml());

        map.insert("v".to_string(), self.v.toml());
        map.insert("blinding_factor".to_string(), self.blinding_factor.toml());
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
        map.insert("depth".to_string(), depth.toml());

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

impl TomlSerializable for VoteChoice {
    fn toml(self) -> Value {
        Value::String(format!("0x{:02x}", (self as usize)))
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
        let mut r_map = toml::map::Map::new();

        r_map.insert("x".to_string(), self.r_b8.x.toml());
        r_map.insert("y".to_string(), self.r_b8.y.toml());

        map.insert("r_b8".to_string(), Value::Table(r_map));
        map.insert("s".to_string(), self.s.toml());

        Value::Table(map)
    }
}

impl TomlSerializable for BBJJ_Ec {
    fn toml(self) -> Value {
        Value::Array(vec![self.x.toml(), self.y.toml()])
    }
}

impl TomlSerializable for H256 {
    fn toml(self) -> Value {
        <[u8; 32]>::from(self).to_vec().toml()
    }
}

impl TomlSerializable for Address {
    fn toml(self) -> Value {
        BN254_Fr::from_be_bytes_mod_order(&<[u8; 20]>::from(self)).toml()
    }
}
