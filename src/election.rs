use ark_std::rand::Rng;
use ark_std::UniformRand;
use strum_macros::EnumIter;
use crate::utils::Mock;
use crate::{BN254_Fr, BBJJ_G1, BBJJ_Pr_Key};

/// Represents the Election Identifiers that uniquely identify an election
#[derive(Clone, Debug)]
pub struct ElectionIdentifier {
    pub(crate) chain_id: BN254_Fr,
    pub(crate) process_id: BN254_Fr,
    pub(crate) contract_addr: BN254_Fr
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


/// Represents the Time Lock Service Parameters
pub struct TLockParams {
    pub(crate) pk_t: BBJJ_G1, // The TLCS public encryption key for time T
    // PK: , // PK_t, TLCS public encryption key
    // space for other TLock parameters
}

impl Mock for TLockParams {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        TLockParams {
            pk_t: BBJJ_Pr_Key::mock(rng).public()
        }
    }
}


/// Represents the Election Parameters
pub struct ElectionParams {
    pub(crate) id: ElectionIdentifier,
    pub(crate) tlock: TLockParams,
}


impl Mock for ElectionParams {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        ElectionParams {
            id: ElectionIdentifier::mock(rng),
            tlock: TLockParams::mock(rng),
        }
    }
}


#[derive(Debug, Clone, EnumIter, Eq, PartialEq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
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

impl Mock for BBJJ_Pr_Key {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        let mut RK_i = vec![0u8; 32];
        rng.fill_bytes(&mut RK_i);
        BBJJ_Pr_Key::import(RK_i).expect("Could not generate a mock BBJJ Private Key.")
    }
}
