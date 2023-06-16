#![allow(non_snake_case)]

use crate::election::{ElectionIdentifier, VoteChoice};
use crate::{BBJJ_Fr, BN254_Fr, Signature, BBJJ_G1};
pub use ethers::prelude::StorageProof;

#[derive(Debug)]
pub struct PublicInput {
    pub(crate) a: BBJJ_G1,
    pub(crate) b: BN254_Fr,
    pub(crate) nullifier: BN254_Fr,
    pub(crate) id_hash: BN254_Fr,
    pub(crate) election_id: ElectionIdentifier,
    pub(crate) r: BBJJ_Fr,
}

#[derive(Debug)]
pub struct PrivateInput {
    pub(crate) k: BBJJ_G1,
    pub(crate) nft_id: BN254_Fr, // Really uint256
    pub(crate) v: VoteChoice,
    pub(crate) sigma: Signature,
    pub(crate) tau: Signature,
    pub(crate) rck: BBJJ_G1,
    pub(crate) p_1: StorageProof,
    pub(crate) p_2: StorageProof,
    pub(crate) p_3: StorageProof,
}

#[derive(Debug)]
pub struct VoteProverPackage {
    pub(crate) public_input: PublicInput,
    pub(crate) private_input: PrivateInput,
}
