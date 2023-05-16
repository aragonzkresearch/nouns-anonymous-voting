use ark_ec::Group;
use ark_std::rand::Rng;
use ark_std::UniformRand;
use poseidon_ark::Poseidon;
use babyjubjub_ark::PrivateKey as PrivateBBJJKey;

use crate::{BN254_Fr, BN254_G1, concat_vec};
use crate::election::{ElectionParams, VoteChoice};
use crate::utils::Mock;
use crate::preprover::{PrivateInput, PublicInput, StorageProofPLACEHOLDER, VoteProverPackage};
use crate::serialisation::Wrapper;


/// Represents the Voter Account
pub struct Voter {
    RK: PrivateBBJJKey, // Secret Registry Key of voter i
}

impl Mock for Voter {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random Vec of bytes length 32
        let mut RK = vec![0u8; 32];
        rng.fill_bytes(&mut RK);
        Voter::new(RK)
    }
}

impl Voter {
    pub fn new(RK: Vec<u8>) -> Self {
        let RK = PrivateBBJJKey::import(RK).unwrap();
        Voter {
            RK,
        }
    }

    pub fn package_vote_for_proving<R: Rng>(&self, rng: &mut R, election_params: &ElectionParams, v: &VoteChoice, nft_id: &BN254_Fr) -> Result<VoteProverPackage, String> {

        let poseidon = Poseidon::new();

        // Generate signatures for the vote and nullifier using the voter's registry key
        // Note that we first hash the messages and only then sign them
        let H_id = poseidon.hash(vec![*nft_id, election_params.identifier.chain_id, election_params.identifier.process_id, election_params.identifier.contract_addr])?;
        let SIGMA= self.RK.sign(H_id)?; // DS.Sign(registry_key, election_params.identifier);
        let vote_choice_message = poseidon.hash(vec![v.clone().into()])?;
        let TAU= self.RK.sign(vote_choice_message)?; // DS.Sign(registry_key, vote_choice);

        // Generate the nullifier
        let N = poseidon.hash(Wrapper(SIGMA.clone()).into())?; // Poseidon(sigma, election_params.identifier);

        let r : BN254_Fr = BN254_Fr::rand(rng); // random value
        let A = BN254_G1::generator() * r; // A = g^r in multiplicative notation
        let K = election_params.tlock.PK_t * r; // K = PK_t^r in multiplicative notation

        let B = poseidon.hash(concat_vec![<Wrapper<BN254_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(K)), vec![v.clone().into(), election_params.identifier.chain_id, election_params.identifier.process_id, election_params.identifier.contract_addr]])?; // Poseidon(K, vote_choice, election_params.identifier);

        let p_1 = StorageProofPLACEHOLDER {}; // TODO // Storage Prove of NFT ownership by voter address
        let p_2 = StorageProofPLACEHOLDER {}; // TODO // Storage Prove that NFT has not been delegated
        let p_3 = StorageProofPLACEHOLDER {}; // TODO // Storage Prove that g^RK is in the registry under the voter's address

        let proverPackage = VoteProverPackage {
            public_input: PublicInput {
                A,
                B,
                N,
                H_id
            },
            private_input: PrivateInput {
                v: v.clone(),
                SIGMA,
                TAU,
                id: election_params.identifier.clone(),
                RCK: self.RK.public(),
                p_1,
                p_2,
                p_3,
            }
        };

        return Ok(proverPackage);
    }


}