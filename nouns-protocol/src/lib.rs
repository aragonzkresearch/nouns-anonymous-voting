#![allow(non_snake_case)]

pub use ark_bn254::Fr as BN254_Fr;
pub use ark_ff::PrimeField;
/// Define the reexported types from the arkworks libraries to be used in this crate
pub use babyjubjub_ark::{Fr as BBJJ_Fr, Point as BBJJ_Ec, PrivateKey, B8 as BBJJ_G1};

pub use noir::MAX_DEPTH;
pub use noir::MAX_NODE_LEN;
pub use tallier::{Tallier, TruncatedBallot};
pub use utils::wrapper::Wrapper;
pub use utils::VoteChoice;
pub use voter::Voter;

mod utils;

pub mod noir;

mod tallier;
pub mod voter;
