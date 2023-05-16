#![allow(non_snake_case)]

use std::ops::Mul;
use ark_bn254::{Fr, G1Projective};
use ark_bn254::g1::Config;
use ark_ec::Group;
use ark_ec::twisted_edwards::Projective;
use ark_ff::{BigInt, BigInteger};
use ark_std::rand::Rng;
use ark_std::UniformRand;
use babyjubjub_ark::Signature;
use poseidon_ark::Poseidon;
use toml::{Table, Value};
use crate::{concat_vec, BN254_G1, Voter, BN254_Fr, BBJJ_Fr, BBJJ_G1};
use crate::election::{ElectionIdentifier, VoteChoice};
use crate::MAX_NODE_LEN;
use crate::MAX_DEPTH;

#[derive(Debug)]
pub struct StorageProof {
    pub(crate) path: Vec<Vec<u8>>,
    pub(crate) depth: usize
}

impl StorageProof {
    pub fn new(path: Vec<Vec<u8>>) -> Self
    {
        let depth = path.len();
        // More checks necessary in reality, but these will catch obviously invalid proofs.
        assert!(depth <= MAX_DEPTH, "The maximum possible proof depth ({}) has been exceeded!", MAX_DEPTH);
        path.iter().for_each(|node| {assert!(node.len() <= MAX_NODE_LEN, "Invalid node!");});
        
        StorageProof {path, depth}
    }
}

#[derive(Debug)]
pub struct PublicInput {
    pub(crate) A_i: BN254_G1,
    pub(crate) B_i: BN254_Fr,
    pub(crate) N_i: BN254_Fr,
    pub(crate) H_id: BN254_Fr
}

#[derive(Debug)]
pub struct PrivateInput {
    pub(crate) v_i: VoteChoice,
    pub(crate) SIGMA_i: Signature,
    pub(crate) TAU_i: Signature,
    pub(crate) id: ElectionIdentifier,
    pub(crate) RCK_i: BBJJ_G1,
    pub(crate) p_1: StorageProof,
    pub(crate) p_2: StorageProof,
    pub(crate) p_3: StorageProof,
}

#[derive(Debug)]
pub struct VoteProverPackage {
    pub(crate) public_input: PublicInput,
    pub(crate) private_input: PrivateInput,
}

// #[derive(Debug)]
// pub struct SerialisedPublicInput {
//     A_i: Vec<BN254_Fr>,
//     B_i: BN254_Fr,
//     N_i: BN254_Fr,
//     H_id: BN254_Fr
// }
//
// impl From<PublicInput> for SerialisedPublicInput {
//     fn from(public_input: PublicInput) -> Self {
//         SerialisedPublicInput {
//             A_i: Wrapper(public_input.A_i).into(),
//             B_i: public_input.B_i.into(),
//             N_i: public_input.N_i.into(),
//             H_id: public_input.H_id.into(),
//         }
//     }
// }



// #[derive(Debug)]
// pub struct SerialisedPrivateInput {
//     v_i: BN254_Fr,
//     SIGMA_i: Vec<BN254_Fr>,
//     TAU_i: Vec<BN254_Fr>,
//     id: ElectionIdentifier,
//     RCK_i: Vec<BN254_Fr>,
//     p_1: Vec<BN254_Fr>,
//     p_2: Vec<BN254_Fr>,
//     p_3: Vec<BN254_Fr>,
// }
//
// impl From<PrivateInput> for SerialisedPrivateInput {
//     fn from(value: PrivateInput) -> Self {
//         SerialisedPrivateInput {
//             v_i: value.v_i.into(),
//             SIGMA_i: Wrapper(value.SIGMA_i).into(),
//             TAU_i: Wrapper(value.TAU_i).into(),
//             id: value.id,
//             RCK_i: Wrapper(value.RCK_i).into(),
//             p_1: Wrapper(value.p_1).into(),
//             p_2: Wrapper(value.p_2).into(),
//             p_3: Wrapper(value.p_3).into(),
//         }
//     }
// }



// #[derive(Debug)]
// pub struct SerialisedVoteProverPackage {
//     public_input: SerialisedPublicInput,
//     private_input: SerialisedPrivateInput,
// }
//
// impl From<VoteProverPackage> for SerialisedVoteProverPackage {
//     fn from(value: VoteProverPackage) -> Self {
//         SerialisedVoteProverPackage {
//             public_input: value.public_input.into(),
//             private_input: value.private_input.into(),
//         }
//     }
// }




