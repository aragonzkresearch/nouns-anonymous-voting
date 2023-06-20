use babyjubjub_ark::Signature;

use crate::services::ethereum::StateProof;
use crate::{BBJJ_Ec, BN254_Fr};

mod preprover;
mod serialisation;

/// The input to the Noir Vote Prover Circuit
pub(crate) struct VoteProverInput {
    // Public input for the circuit
    pub(crate) a: BBJJ_Ec,
    pub(crate) b: BN254_Fr,
    pub(crate) n: BN254_Fr,
    pub(crate) h_id: BN254_Fr,
    pub(crate) process_id: BN254_Fr,
    pub(crate) contract_addr: BN254_Fr,
    pub(crate) chain_id: BN254_Fr,
    pub(crate) eth_block_hash: BN254_Fr,
    pub(crate) tcls_pk: BBJJ_Ec,

    // Private input for the circuit
    pub(crate) v: BN254_Fr,
    pub(crate) signed_id: Signature,
    // `sigma`
    pub(crate) voter_address: BN254_Fr,
    pub(crate) signed_v: Signature,
    // `tau`
    pub(crate) nft_id: BN254_Fr,
    pub(crate) k: BBJJ_Ec,
    /// The public key of the voter's `sk` that is registered in the `BBJJ` interface in the `zkRegistry`
    pub(crate) registered_pbk: BBJJ_Ec,
    pub(crate) registry_key_sp: StateProof,
    pub(crate) nft_ownership_proof: StateProof,
}

pub(crate) async fn prove_vote(input: VoteProverInput) -> Result<Vec<u8>, String> {
    // Serialize the input into a toml file
    let input = toml::to_string(&input).unwrap();
}
