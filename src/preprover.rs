#![allow(non_snake_case)]

use crate::{BN254_Fr, BBJJ_Fr, BBJJ_G1, Signature};
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
    pub(crate) a: BBJJ_G1,
    pub(crate) b: BN254_Fr,
    pub(crate) nullifier: BN254_Fr,
    pub(crate) id_hash: BN254_Fr,
    pub(crate) election_id: ElectionIdentifier,
    pub(crate) r: BBJJ_Fr
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



