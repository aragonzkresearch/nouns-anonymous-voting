#![allow(non_snake_case)]

pub use ark_bn254::Fr as BN254_Fr;
/// Define the reexported types from the arkworks libraries to be used in this crate
pub use babyjubjub_ark::{Fr as BBJJ_Fr, Point as BBJJ_Ec, B8 as BBJJ_G1};

pub use utils::wrapper::Wrapper;
pub use utils::VoteChoice;
pub use voter::Voter;

mod utils;

mod noir;

mod tallier;
mod voter;
