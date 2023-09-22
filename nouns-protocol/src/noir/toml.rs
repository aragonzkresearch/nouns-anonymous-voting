use ark_ff::{BigInteger, PrimeField};
use babyjubjub_ark::Signature;
use ethers::types::{Address, StorageProof, H256, U64};
use toml::Value;

use crate::noir::{
    BlockHashVerifierInput, TallyProverInput, VoteProverInput, MAX_ACCOUNT_STATE_SIZE,
    MAX_BLOCK_HEADER_SIZE, MAX_DEPTH, MAX_NODE_LEN,
};
use crate::{utils::VoteChoice, BBJJ_Ec, BBJJ_Fr, BN254_Fr, BlockHeader, StateProof};

pub trait TomlSerializable {
    fn toml(self) -> Value;
}

impl TomlSerializable for BlockHashVerifierInput {
    fn toml(self) -> Value {
        let mut toml_map = toml::map::Map::new();

        toml_map.insert("block_hash".to_string(), self.block_hash.toml());
        toml_map.insert("block_number".to_string(), self.block_number.toml());
        toml_map.insert("block_header".to_string(), self.block_header.toml());
        toml_map.insert("registry_address".to_string(), self.registry_address.toml());
        toml_map.insert(
            "registry_state_proof".to_string(),
            self.registry_state_proof.toml(),
        );
        toml_map.insert(
            "registry_storage_root".to_string(),
            self.registry_storage_root.toml(),
        );
        toml_map.insert(
            "nft_contract_address".to_string(),
            self.nft_contract_address.toml(),
        );
        toml_map.insert("nft_state_proof".to_string(), self.nft_state_proof.toml());
        toml_map.insert("nft_storage_root".to_string(), self.nft_storage_root.toml());

        Value::Table(toml_map)
    }
}

impl TomlSerializable for TallyProverInput {
    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();

        // Need to include the number of voters in output
        let num_voters = self.v.len();

        // Maximum number of voters is fixed by the underlying circuit,
        // so we have to pad. First obtain this value.
        let padded_len: usize = crate::noir::max_num_voters();

        assert!(num_voters <= padded_len, "Number of voters exceeds maximum!");

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
        map.insert(
            "registry_key_proof".to_string(),
            self.registry_key_sp.toml(),
        );
        map.insert(
            "nft_ownership_proof".to_string(),
            self.nft_ownership_proof.toml(),
        );
        map.insert("delegation_proof".to_string(), self.delegation_proof.toml());

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
        let self_bytes = <[u8; 32]>::from(self);
        let self_l = BN254_Fr::from_be_bytes_mod_order(&self_bytes[0..16]);
        let self_r = BN254_Fr::from_be_bytes_mod_order(&self_bytes[16..]);
        vec![self_l, self_r].toml()
    }
}

impl TomlSerializable for U64 {
    fn toml(self) -> Value {
        Value::String(format!("0x{:02x}", self.as_u64()))
    }
}

impl TomlSerializable for Address {
    fn toml(self) -> Value {
        BN254_Fr::from_be_bytes_mod_order(&<[u8; 20]>::from(self)).toml()
    }
}

impl TomlSerializable for StateProof {
    fn toml(self) -> toml::Value {
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

        let key: [u8; 20] = self.key.into();

        let value_len = self.value.len();

        assert!(value_len <= MAX_ACCOUNT_STATE_SIZE);

        let mut value = vec![0u8; MAX_ACCOUNT_STATE_SIZE - value_len];

        value.append(&mut self.value.clone());

        map.insert("key".to_string(), key.to_vec().toml());
        map.insert("value".to_string(), value.toml());
        map.insert("depth".to_string(), depth.toml());

        toml::Value::Table(map)
    }
}

impl TomlSerializable for BlockHeader {
    fn toml(self) -> toml::Value {
        let mut value = self.0.clone();
        let value_len = value.len();
        assert!(value_len <= MAX_BLOCK_HEADER_SIZE);
        value.append(&mut vec![0; MAX_BLOCK_HEADER_SIZE - value_len]);

        value.toml()
    }
}
