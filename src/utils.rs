mod mock;

use ark_std::rand::Rng;

/// Mock trait is used to generate mock data for testing
pub trait Mock {
    fn mock<R: Rng>(rng: &mut R) -> Self;
}

#[derive(Debug, Clone, EnumIter, Eq, PartialEq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}



/// Represents the Election Parameters
pub struct ElectionParams {
    pub(crate) id: ElectionIdentifier,
    pub(crate) tlock: TLockParams,
}


/// Represents the Election Identifiers that uniquely identify an election
#[derive(Clone, Debug)]
pub struct ElectionIdentifier {
    pub(crate) chain_id: BN254_Fr,
    pub(crate) process_id: BN254_Fr,
    pub(crate) contract_addr: BN254_Fr,
}