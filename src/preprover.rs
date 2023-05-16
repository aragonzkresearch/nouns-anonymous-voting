#![allow(non_snake_case)]

use crate::{BN254_Fr, BBJJ_G1, Signature};
use crate::election::{ElectionIdentifier, VoteChoice};


#[derive(Debug)]
pub struct StorageProofPLACEHOLDER {
    // TODO - parametrise this with Ahmad's work
}





#[derive(Debug)]
pub struct PublicInput {
    pub(crate) A_i: BBJJ_G1,
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
    pub(crate) p_1: StorageProofPLACEHOLDER,
    pub(crate) p_2: StorageProofPLACEHOLDER,
    pub(crate) p_3: StorageProofPLACEHOLDER,
}

#[derive(Debug)]
pub struct VoteProverPackage {
    pub(crate) public_input: PublicInput,
    pub(crate) private_input: PrivateInput,
}



