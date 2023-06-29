use strum_macros::EnumIter;

use crate::BN254_Fr;

#[cfg(test)]
pub(crate) mod mock;

pub(crate) mod wrapper;

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
