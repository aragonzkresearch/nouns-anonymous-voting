use ark_ec::Group;
use ark_std::rand::Rng;
use ark_std::UniformRand;
use poseidon_ark::Poseidon;
use babyjubjub_ark::{PrivateKey as PrivateBBJJKey};

use crate::{BN254_Fr, BN254_G1, concat_vec};
use crate::election::{ElectionParams, VoteChoice};
use crate::utils::Mock;
use crate::preprover::{PrivateInput, PublicInput, StorageProof, VoteProverPackage};
use crate::serialisation::Wrapper;


/// Represents the Voter Account
pub struct Voter {
    RK_i: PrivateBBJJKey, // Secret Registry Key of voter i
}

impl Mock for Voter {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random Vec of bytes length 32
        let mut RK_i = vec![0u8; 32];
        rng.fill_bytes(&mut RK_i);
        Voter::new(RK_i)
    }
}

impl Voter {
    pub fn new(RK_i: Vec<u8>) -> Self {
        let RK_i = PrivateBBJJKey::import(RK_i).unwrap();
        Voter {
            RK_i,
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

        let r_i : BN254_Fr = BN254_Fr::rand(rng); // random value
        let A_i = BN254_G1::generator() * r_i; // A = g^r_i in multiplicative notation
        let K_i = election_params.tlock.PK_t * r_i; // K = PK_t^r_i in multiplicative notation

        let B_i = poseidon.hash(concat_vec![<Wrapper<BN254_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(K_i)), vec![v_i.clone().into(), election_params.identifier.chain_id, election_params.identifier.process_id, election_params.identifier.contract_addr]])?; // Poseidon(K_i, vote_choice, election_params.identifier);

        let p_1 = StorageProof::new(vec![]); // TODO // Storage Prove of NFT ownership by voter address
        let p_2 = StorageProof::new(vec![]); // TODO // Storage Prove that NFT has not been delegated
        let p_3 = StorageProof::new(vec![]); // TODO // Storage Prove that g^RK_i is in the registry under the voter's address

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
