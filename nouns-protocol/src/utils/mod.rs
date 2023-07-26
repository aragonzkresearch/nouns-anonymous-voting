use strum_macros::EnumIter;
use ethers::types::{Address, Block, Bytes, H256, U64};

use crate::BN254_Fr;

#[cfg(test)]
pub(crate) mod mock;

pub(crate) mod wrapper;

// Type for state proofs
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StateProof {
    pub key: Address,
    pub proof: Vec<Bytes>,
    pub value: Vec<u8>,
}

// Wrapper for block header
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BlockHeader(pub Vec<u8>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum VoteChoice {
    No,
    Yes,
    Abstain,
}

impl From<VoteChoice> for u8 {
    fn from(value: VoteChoice) -> Self {
        match value {
            VoteChoice::No => 0,
            VoteChoice::Yes => 1,
            VoteChoice::Abstain => 2,
        }
    }
}

impl From<&str> for VoteChoice {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "no" => Self::No,
            "yes" => Self::Yes,
            "abstain" => Self::Abstain,
            "n" => Self::No,
            "y" => Self::Yes,
            "a" => Self::Abstain,
            _ => panic!("Invalid vote choice"),
        }
    }
}

impl From<VoteChoice> for BN254_Fr {
    fn from(value: VoteChoice) -> Self {
        match value {
            VoteChoice::No => Self::from(u8::from(VoteChoice::No)),
            VoteChoice::Yes => Self::from(u8::from(VoteChoice::Yes)),
            VoteChoice::Abstain => Self::from(u8::from(VoteChoice::Abstain)),
        }
    }
}
