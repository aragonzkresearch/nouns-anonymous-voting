use ark_ff::BigInteger;
use ark_std::rand::Rng;
use ark_std::UniformRand;
use babyjubjub_ark::PrivateKey;
use poseidon_ark::Poseidon;

use crate::{BN254_Fr, BBJJ_G1, BBJJ_Pr_Key, BBJJ_Fr, concat_vec};
use crate::election::{ElectionParams, VoteChoice};
use crate::utils::Mock;
use crate::preprover::{PrivateInput, PublicInput, StorageProofPLACEHOLDER, VoteProverPackage};
use crate::serialisation::Wrapper;


/// Represents the Voter Account
pub struct Voter {
    RK_i: BBJJ_Pr_Key, // Secret Registry Key of voter i
}

impl Mock for Voter {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random Vec of bytes length 32
        Voter::new(
            BBJJ_Pr_Key::mock(rng)
        )
    }
}

impl Voter {
    pub fn new(private_key: BBJJ_Pr_Key) -> Self {
        Voter {
            RK_i: private_key
        }
    }

    pub fn package_vote_for_proving<R: Rng>(&self, rng: &mut R, election_params: &ElectionParams, v_i: &VoteChoice, nft_id: &BN254_Fr) -> Result<VoteProverPackage, String> {

        let poseidon = Poseidon::new();

        // Generate signatures for the vote and nullifier using the voter's registry key
        // Note that we first hash the messages and only then sign them
        let H_id = poseidon.hash(vec![*nft_id, election_params.identifier.chain_id, election_params.identifier.process_id, election_params.identifier.contract_addr])?;
        let SIGMA_i= self.RK_i.sign(H_id)?; // DS.Sign(registry_key, election_params.identifier);
        let vote_choice_message = poseidon.hash(vec![v_i.clone().into()])?;
        let TAU_i= self.RK_i.sign(vote_choice_message)?; // DS.Sign(registry_key, vote_choice);

        // Generate the nullifier
        let N_i = poseidon.hash(Wrapper(SIGMA_i.clone()).into())?; // Poseidon(sigma, election_params.identifier);

        let r_i = BBJJ_Fr::rand(rng);
        let A_i : BBJJ_G1 = PrivateKey::import(r_i.0.to_bytes_be().to_vec()).expect("Failed to import r_i").public(); // A = g^r_i in multiplicative notation
        let K_i : BBJJ_G1 = election_params.tlock.PK_t.mul_scalar(&r_i); // K = PK_t^r_i in multiplicative notation

        let B_i = poseidon.hash(concat_vec![<Wrapper<BBJJ_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(K_i)), vec![v_i.clone().into(), election_params.identifier.chain_id, election_params.identifier.process_id, election_params.identifier.contract_addr]])?; // Poseidon(K_i, vote_choice, election_params.identifier);

        let p_1 = StorageProofPLACEHOLDER {}; // TODO // Storage Prove of NFT ownership by voter address
        let p_2 = StorageProofPLACEHOLDER {}; // TODO // Storage Prove that NFT has not been delegated
        let p_3 = StorageProofPLACEHOLDER {}; // TODO // Storage Prove that g^RK_i is in the registry under the voter's address

        let proverPackage = VoteProverPackage {
            public_input: PublicInput {
                A_i,
                B_i,
                N_i,
                H_id
            },
            private_input: PrivateInput {
                v_i: v_i.clone(),
                SIGMA_i,
                TAU_i,
                id: election_params.identifier.clone(),
                RCK_i: self.RK_i.public(),
                p_1,
                p_2,
                p_3,
            }
        };

        return Ok(proverPackage);
    }


}