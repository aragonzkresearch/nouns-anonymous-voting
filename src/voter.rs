use ark_ff::BigInteger;
use ark_std::rand::Rng;
use ark_std::UniformRand;
use babyjubjub_ark::PrivateKey;
use poseidon_ark::Poseidon;

use crate::{BN254_Fr, BBJJ_G1, BBJJ_Pr_Key, BBJJ_Fr, concat_vec};
use crate::election::{ElectionParams, VoteChoice};
use crate::utils::Mock;
use crate::preprover::{PrivateInput, PublicInput, StorageProof, VoteProverPackage};
use crate::serialisation::Wrapper;


/// Represents the Voter Account
pub struct Voter {
    rck: BBJJ_Pr_Key, // Secret Registry Key of voter i
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
            rck: private_key
        }
    }

    pub fn package_vote_for_proving<R: Rng>(&self, rng: &mut R, election_params: &ElectionParams, v: &VoteChoice, nft_id: &BN254_Fr) -> Result<VoteProverPackage, String> {

        let poseidon = Poseidon::new();

        // Generate signatures for the vote and nullifier using the voter's registry key
        // Note that we first hash the messages and only then sign them
        let id_hash = poseidon.hash(vec![*nft_id, election_params.id.chain_id, election_params.id.process_id, election_params.id.contract_addr])?;
        let sigma = self.rck.sign(id_hash)?; // DS.Sign(registry_key, election_params.identifier);
        let vote_choice_message = poseidon.hash(vec![v.clone().into()])?;
        let tau = self.rck.sign(vote_choice_message)?; // DS.Sign(registry_key, vote_choice);

        // Generate the nullifier
        let nullifier = poseidon.hash(Wrapper(sigma.clone()).into())?; // Poseidon(sigma, election_params.identifier);

        let r = BBJJ_Fr::rand(rng);
        let a: BBJJ_G1 = PrivateKey::import(r.0.to_bytes_be().to_vec()).expect("Failed to import r_i").public(); // A = g^r_i in multiplicative notation
        let k: BBJJ_G1 = election_params.tlock.PK_t.mul_scalar(&r); // K = PK_t^r_i in multiplicative notation

        let b = poseidon.hash(concat_vec![<Wrapper<BBJJ_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(k)), vec![v.clone().into(), election_params.id.chain_id, election_params.id.process_id, election_params.id.contract_addr]])?; // Poseidon(K_i, vote_choice, election_params.identifier);

        let p_1 = StorageProof::new(vec![]); // TODO // Storage Prove of NFT ownership by voter address
        let p_2 = StorageProof::new(vec![]); // TODO // Storage Prove that NFT has not been delegated
        let p_3 = StorageProof::new(vec![]); // TODO // Storage Prove that g^RK_i is in the registry under the voter's address

        let proverPackage = VoteProverPackage {
            public_input: PublicInput {
                a,
                b,
                nullifier,
                id_hash,
                election_id: election_params.id.clone()
            },
            private_input: PrivateInput {
                v: v.clone(),
                sigma,
                tau,
                rck: self.rck.public(),
                p_1,
                p_2,
                p_3,
            }
        };

        return Ok(proverPackage);
    }


}