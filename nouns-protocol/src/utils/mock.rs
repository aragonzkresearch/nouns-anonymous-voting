use ark_ff::PrimeField;
use babyjubjub_ark::PrivateKey;
use ethers::core::k256::elliptic_curve::bigint::Encoding;
use ethers::core::k256::U256;
use ethers::prelude::StorageProof;
use ethers::types::{Address, H256};
use rand::Rng;

use crate::voter::Voter;
use crate::{BBJJ_Ec, BBJJ_Fr, BN254_Fr, VoteChoice, BBJJ_G1};

/// Mock trait is used to generate mock data for testing
pub trait Mock {
    fn mock<R: Rng>(rng: &mut R) -> Self;
}

impl Mock for Voter {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random Vec of bytes length 32
        Voter::new(
            Address::mock(rng),
            PrivateKey::import(Vec::from(U256::mock(rng).to_be_bytes())).unwrap(),
        )
    }
}

impl Mock for VoteChoice {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..3) {
            0 => VoteChoice::Yes,
            1 => VoteChoice::No,
            2 => VoteChoice::Abstain,
            _ => panic!("Invalid vote choice"),
        }
    }
}

impl Mock for StorageProof {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        StorageProof {
            key: H256::from(rng.gen::<[u8; 32]>()),
            value: ethers::types::U256::zero(),
            proof: vec![],
        }
    }
}

impl Mock for U256 {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random Vec of bytes length 32
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);

        U256::from_be_bytes(bytes)
    }
}

impl Mock for Address {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random Vec of bytes length 20
        let mut bytes = [0u8; 20];
        rng.fill_bytes(&mut bytes);

        Address::from_slice(&bytes)
    }
}

impl Mock for BBJJ_Ec {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random BBJJ_Fr
        let fr = BBJJ_Fr::mock(rng);

        BBJJ_G1.mul_scalar(&fr)
    }
}

impl Mock for BBJJ_Fr {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random U256
        let num = U256::mock(rng);

        BBJJ_Fr::from_be_bytes_mod_order(num.to_be_bytes().as_slice())
    }
}

impl Mock for BN254_Fr {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random U256
        let num = U256::mock(rng);

        BN254_Fr::from_be_bytes_mod_order(num.to_be_bytes().as_slice())
    }
}

impl Mock for PrivateKey {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random Vec of bytes length 32
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);

        PrivateKey::import(bytes.to_vec()).unwrap()
    }
}
