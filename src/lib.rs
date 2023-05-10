#![allow(non_snake_case)]


mod vote_proof;

use std::ops::Mul;
use ark_ff::One;
use ark_std::rand::Rng;
use ark_std::UniformRand;

pub use ark_bn254::Fr as ScalarField; // TODO - check if this should be BBJJ or BN254
pub use ark_bn254::G1Projective as G1;

use ark_ec::{AffineRepr, Group};
use crate::vote_proof::VoteProverPackage; // TODO - check if this should be BBJJ or BN254

// Mock trait is used to generate mock data for testing
pub trait Mock {
    fn mock<R: Rng>(rng: &mut R) -> Self;
}

pub struct Voter {
    RK_i: ScalarField, // Secret Registry Key of voter i
    RCK_i: G1, // Public Registry Key of voter i

}

#[derive(Clone, Debug)]
pub struct ElectionIdentifier {
    chain_id: ScalarField,
    process_id: ScalarField,
    contract_addr: ScalarField
}

impl Mock for ElectionIdentifier {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        ElectionIdentifier {
            chain_id: ScalarField::from(0u8),
            process_id: ScalarField::from(4u8),
            contract_addr: ScalarField::rand(rng),
        }
    }
}



pub struct TLockParams {
    PK_t: G1, // The TLCS public encryption key for time T
    // PK: , // PK_t, TLCS public encryption key
    // space for other TLock parameters
}

impl Mock for TLockParams {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        TLockParams {
            PK_t: G1::rand(rng),
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

#[derive(Debug)]
pub struct StorageProofPLACEHOLDER {
    // TODO - parametrise this with Ahmad's work
}

#[derive(Debug)]
pub struct SignaturePLACEHOLDER {
    // TODO - parametrise this with Ahmad's work
}

pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}



// Implement conversion trait for Vote to F
impl From<VoteChoice> for ScalarField {
    fn from(vote: VoteChoice) -> Self {
        match vote {
            VoteChoice::Yes => ScalarField::from(0u8),
            VoteChoice::No => ScalarField::from(1u8),
            VoteChoice::Abstain => ScalarField::from(2u8),
        }
    }
}


impl Voter {
    pub fn new(RK_i: ScalarField) -> Self {
        Voter {
            RK_i,
            RCK_i: G1::generator().mul(RK_i),
        }
    }
}

impl Mock for Voter {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        let RK_i = ark_bn254::Fr::rand(rng);
        Voter::new(RK_i)
    }
}


pub fn run() {

    let mut rng = ark_std::test_rng();
    let voter = Voter::mock(&mut rng);

    let election_params = ElectionParams::mock(&mut rng);

    let vote_package = voter.package_vote_for_proving(&election_params, VoteChoice::Yes);

    // Debug print the vote package
    println!("vote_package: {:?}", vote_package);
}