#![allow(non_snake_case)]


extern crate core;

mod vote_proof;
mod snark_repr;

use std::fs;
use std::ops::Mul;
use ark_ff::One;
use ark_std::rand::Rng;
use ark_std::UniformRand;

pub(crate) use ark_bn254::{G1Projective as BN254_G1, Fr as BN254_Fr};
pub(crate) use babyjubjub_ark::{Point as BBJJ_G1, Signature, Fr as BBJJ_Fr};


use ark_ec::{Group};
use babyjubjub_ark::{Point as PublicBBJJKey, PrivateKey as PrivateBBJJKey};
use toml::Value;
use crate::snark_repr::TomlSerialisable;

// Mock trait is used to generate mock data for testing
pub trait Mock {
    fn mock<R: Rng>(rng: &mut R) -> Self;
}

pub struct Voter {
    RK_i: PrivateBBJJKey, // Secret Registry Key of voter i
}

#[derive(Clone, Debug)]
pub struct ElectionIdentifier {
    chain_id: BN254_Fr,
    process_id: BN254_Fr,
    contract_addr: BN254_Fr
}

impl Into<Vec<BN254_Fr>> for ElectionIdentifier {
    fn into(self) -> Vec<BN254_Fr> {
        vec![self.chain_id, self.process_id, self.contract_addr]
    }
}

impl TomlSerialisable for ElectionIdentifier {
    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        map.insert("chain_id".to_string(), self.chain_id.toml());
        map.insert("process_id".to_string(), self.chain_id.toml());
        map.insert("contract_addr".to_string(), self.chain_id.toml());
        Value::Table(map)
    }
}

impl Mock for ElectionIdentifier {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        ElectionIdentifier {
            chain_id: BN254_Fr::from(0u8),
            process_id: BN254_Fr::from(4u8),
            contract_addr: BN254_Fr::rand(rng),
        }
    }
}



pub struct TLockParams {
    PK_t: BN254_G1, // The TLCS public encryption key for time T
    // PK: , // PK_t, TLCS public encryption key
    // space for other TLock parameters
}

impl Mock for TLockParams {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        TLockParams {
            PK_t: BN254_G1::rand(rng),
        }
    }
}

pub struct ElectionParams {
    identifier: ElectionIdentifier,
    tlock: TLockParams,
}

impl Mock for ElectionParams {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        ElectionParams {
            identifier: ElectionIdentifier::mock(rng),
            tlock: TLockParams::mock(rng),
        }
    }
}


#[derive(Debug, Clone)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}



// Implement conversion trait for Vote to F
impl Into<BN254_Fr> for VoteChoice {

    fn into(self) -> BN254_Fr {
        match self {
            VoteChoice::Yes => BN254_Fr::from(0u8),
            VoteChoice::No => BN254_Fr::from(1u8),
            VoteChoice::Abstain => BN254_Fr::from(2u8),
        }
    }
}


impl Voter {
    pub fn new(RK_i: Vec<u8>) -> Self {
        let RK_i = PrivateBBJJKey::import(RK_i).unwrap();
        Voter {
            RK_i,
        }
    }
}

impl Mock for Voter {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random Vec of bytes length 32
        let mut RK_i = vec![0u8; 32];
        rng.fill_bytes(&mut RK_i);
        Voter::new(RK_i)
    }
}


pub fn run() {

    let mut rng = ark_std::test_rng();
    let voter = Voter::mock(&mut rng);

    let election_params = ElectionParams::mock(&mut rng);
    let nft_id = BN254_Fr::from(0u8);
    let vote_choice = VoteChoice::Yes;

    let vote_package = voter.package_vote_for_proving(&mut rng, &election_params, &vote_choice, &nft_id);

    // Debug print the vote package
    println!("vote_package: {:?}", vote_package);

    let toml_string = vote_package.unwrap().toml().to_string();
    fs::write("Prover.toml", toml_string).expect("Could not write to file!");
}