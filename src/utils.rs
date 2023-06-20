use ethers::core::k256::U256;
use ethers::types::Address;

use crate::BBJJ_Ec;

#[cfg(test)]
pub(crate) mod mock;

pub(crate) mod wrapper;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VoteChoice {
    Yes,
    No,
    Abstain,
}

impl From<u8> for VoteChoice {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Yes,
            1 => Self::No,
            2 => Self::Abstain,
            _ => panic!("Invalid vote choice"),
        }
    }
}

impl From<VoteChoice> for u8 {
    fn from(value: VoteChoice) -> Self {
        match value {
            VoteChoice::Yes => 0,
            VoteChoice::No => 1,
            VoteChoice::Abstain => 2,
        }
    }
}

/// Represents the parameters of the process that the voter is voting in
pub(crate) struct ProcessParameters {
    /// The id of the process
    pub(crate) process_id: U256,
    /// The address of the contract that represents the process
    pub(crate) contract_addr: Address,
    /// The chain id of the chain that the process is running on
    pub(crate) chain_id: U256,
    /// The public key of the tcls for the process decryption time
    pub(crate) tcls_pk: BBJJ_Ec,
    /// The hash of the block that the process checkpointed on
    pub(crate) eth_block_hash: U256,
}
