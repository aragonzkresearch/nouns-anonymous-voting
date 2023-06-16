use ark_ff::{BigInteger, PrimeField};
use ark_std::rand::Rng;
use ark_std::UniformRand;
use poseidon_ark::Poseidon;
use ethers::prelude::{Address, Bytes};

use crate::election::{ElectionParams, VoteChoice};
use crate::preprover::{PrivateInput, PublicInput, StorageProof, VoteProverPackage};
use crate::serialisation::Wrapper;
use crate::utils::Mock;
use crate::{concat_vec, BBJJ_Fr, BBJJ_Pr_Key, BN254_Fr, B8, BBJJ_G1};

/// Represents the Voter Account
pub struct Voter {
    address: Address, // TODO
    private_key: Bytes, // TODO
    rck: BBJJ_Pr_Key, // Secret Registry Key of voter i
}

impl Mock for Voter {
    fn mock<R: Rng>(rng: &mut R) -> Self {
        // Generate a random Vec of bytes length 32
        Voter::new("0x0000000000000000000000000000000000000000".parse().unwrap(), "0x00".parse().unwrap(), BBJJ_Pr_Key::mock(rng)) // TODO
    }
}

impl Voter {
    pub fn new(address: Address, private_key: Bytes, rck: BBJJ_Pr_Key) -> Self {
        Voter { address, private_key, rck}
    }

    pub async fn package_vote_for_proving<R: Rng>(
        &self,
        rng: &mut R,
        election_params: &ElectionParams,
        v: &VoteChoice,
        nft_id: &BN254_Fr,
    ) -> Result<VoteProverPackage, String> {
        let poseidon = Poseidon::new();

        // Generate signatures for the vote and nullifier using the voter's registry key
        // Note that we first hash the messages and only then sign them
        let id_hash = poseidon.hash(vec![
            *nft_id,
            election_params.id.chain_id,
            election_params.id.process_id,
            election_params.id.contract_addr,
        ])?;
        let sigma = self.rck.sign(id_hash)?; // DS.Sign(registry_key, election_params.identifier);
        let vote_choice_message = poseidon.hash(vec![v.clone().into()])?;
        let tau = self.rck.sign(vote_choice_message)?; // DS.Sign(registry_key, vote_choice);

        // Generate the nullifier
        let nullifier = poseidon.hash(vec![
            sigma.r_b8.x,
            sigma.r_b8.y,
            BN254_Fr::from_be_bytes_mod_order(&sigma.s.into_bigint().to_bytes_be()),
        ])?; // Poseidon(sigma, election_params.identifier);

        let r = BBJJ_Fr::rand(rng);
        let a: BBJJ_G1 = B8.mul_scalar(&r); // A = g^r_i in multiplicative notation
        let k: BBJJ_G1 = election_params.tlock.pk_t.mul_scalar(&r); // K = PK_t^r_i in multiplicative notation

        let b = poseidon.hash(concat_vec![
            <Wrapper<BBJJ_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(k.clone())),
            vec![
                v.clone().into(),
                election_params.id.chain_id,
                election_params.id.process_id,
                election_params.id.contract_addr
            ]
        ])?; // Poseidon(K_i, vote_choice, election_params.identifier);

        let p_1 = StorageProof {proof: vec![[0].into_iter().collect::<ethers::prelude::Bytes>()], key: [0u8;32].into(), value: 0.into()}; // TODO // Storage Prove of NFT ownership by voter address
        let p_2 = p_1.clone(); // TODO // Storage Prove that NFT has not been delegated
        let p_3 = p_1.clone(); // TODO // Storage Prove that g^RK_i is in the registry under the voter's address

        let proverPackage = VoteProverPackage {
            public_input: PublicInput {
                a,
                b,
                nullifier,
                id_hash,
                election_id: election_params.id.clone(),
                r,
            },
            private_input: PrivateInput {
                k,
                nft_id: nft_id.clone(),
                v: v.clone(),
                sigma,
                tau,
                rck: self.rck.public(),
                p_1,
                p_2,
                p_3,
            },
        };

        return Ok(proverPackage);
    }
}
