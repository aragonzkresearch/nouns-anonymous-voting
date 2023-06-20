use ark_ff::{BigInteger, PrimeField};
use ark_std::rand::Rng;
use ark_std::UniformRand;
use babyjubjub_ark::{Signature, B8};
use ethers::core::k256::U256;
use ethers::prelude::Address;
use poseidon_ark::Poseidon;

use crate::services::ethereum::StateProof;
use crate::services::noir;
use crate::utils::wrapper::Wrapper;
use crate::utils::{ProcessParameters, VoteChoice};
use crate::{BBJJ_Ec, BBJJ_Fr, BN254_Fr, BBJJ_G1};

/// Represents the Voter who votes in the process
/// Note that we do not need to know that private key corresponding to the eth_addr
/// By finding the `pbk` registered under the `address` it implies that the
/// Owner of the address (and corresponding NFTs) delegated the voting right
/// to whoever knows the `sk` for the `pbk` in the `zkRegistry`
pub struct Voter {
    /// The Ethereum Address that the `pbk` is registered under
    eth_addr: Address,
    /// Secret Key (`sk`) of the Public Key (`pbk`) that is registered under the `address` in zkRegistry
    /// As we are using BBJJ this key should be registered in the `BBJJ` interface.
    registered_sk: babyjubjub_ark::PrivateKey,
}

impl Voter {
    pub fn new(address: Address, rck: babyjubjub_ark::PrivateKey) -> Self {
        Voter {
            eth_addr: address,
            registered_sk: rck,
        }
    }
}

/// Represents the ballot that the voter casts
pub struct Ballot {
    /// The first part of the encrypted vote
    pub(crate) a: BBJJ_Ec,
    /// The second part of the encrypted vote
    pub(crate) b: BN254_Fr,
    /// The nullifier of the encrypted vote, to prevent double voting
    pub(crate) n: BN254_Fr,
    /// The hash of the id of the vote, to prevent malleability
    pub(crate) h_id: BN254_Fr,
}

/// Represents the ballot that the voter casts
/// With an attached proof that the ballot is valid
pub struct BallotWithProof {
    /// The ballot that the voter casts
    ballot: Ballot,
    /// Ballot correctness proof, represented as a vector of bytes
    proof: Vec<u8>,
}

/// Represents the hints that were generated while constructing the ballot
/// that the prover needs to generate the proof for ballot correctness
pub(crate) struct BallotHints {
    /// `sigma` representing the signature over the id of the vote
    signed_id: Signature,
    /// `tau` representing the signature over the vote choice
    signed_v: Signature,
    /// `k` representing the bliding factor of the vote
    k: BBJJ_Ec,
}

impl Voter {
    /// Generate a vote for given parameters
    pub(crate) fn gen_vote<R: Rng>(
        &self,
        nft_id: U256,
        v: VoteChoice,
        process_params: &ProcessParameters,
        state_proofs: (StateProof, StateProof),
        rng: &mut R,
    ) -> Result<BallotWithProof, String> {
        // Convert the parameters to the correct field
        let nft_id: [BN254_Fr; 2] = Wrapper(nft_id).into();
        let v: BN254_Fr = Wrapper(U256::from(u8::from(v))).into();
        let process_id: BN254_Fr = Wrapper(process_params.process_id).into();
        let contract_addr: BN254_Fr = Wrapper(process_params.contract_addr).into();
        let chain_id: [BN254_Fr; 2] = Wrapper(process_params.chain_id).into();

        // Prepare the inputs for the Noir circuit vote prover circuit
        let (ballot, ballot_hints) = self.gen_ballot_with_hints(
            nft_id,
            v,
            process_id,
            contract_addr,
            chain_id,
            process_params.tcls_pk.clone(),
            rng,
        )?;

        let noir_input = noir::VoteProverInput {
            // Public inputs
            a: ballot.a.clone(),
            b: ballot.b,
            n: ballot.n,
            h_id: ballot.h_id,
            process_id,
            contract_addr,
            chain_id,
            eth_block_hash: Wrapper(process_params.eth_block_hash).into(),
            tcls_pk: process_params.tcls_pk.clone(),
            // Private inputs
            v,
            signed_id: ballot_hints.signed_id,
            voter_address: Wrapper(self.eth_addr).into(),
            signed_v: ballot_hints.signed_v,
            nft_id,
            k: ballot_hints.k,
            registered_pbk: self.registered_sk.public(),
            registry_key_sp: state_proofs.0,
            nft_ownership_proof: state_proofs.1,
        };

        let proof = noir::prove_vote(noir_input)?;

        let ballot_with_proof = BallotWithProof { ballot, proof };

        Ok(ballot_with_proof)
    }

    /// Generate a vote ballot with prover hints for given vote parameters
    /// The prover hints will be used by the Noir Vote Prover to generate the proof of ballot correctness
    pub(crate) fn gen_ballot_with_hints<R: Rng>(
        &self,
        nft_id: [BN254_Fr; 2],
        v: BN254_Fr,
        process_id: BN254_Fr,
        contract_addr: BN254_Fr,
        chain_id: [BN254_Fr; 2],
        tlcs_pk: BBJJ_Ec,
        rng: &mut R,
    ) -> Result<(Ballot, BallotHints), String> {
        let poseidon = Poseidon::new();

        // Generate the hash of the id of the vote and then sign it to prevent malleability
        let id_hash = poseidon.hash(vec![
            nft_id[0],
            nft_id[1],
            chain_id[0],
            chain_id[1],
            process_id,
            contract_addr,
        ])?;
        let signed_id = self.registered_sk.sign(id_hash)?; // `sigma = DS.Sign(registry_key, election_params.identifier)`

        // Sign the hashed vote choice to prevent malleability
        let vote_choice_message = poseidon.hash(vec![v])?;
        let signed_v = self.registered_sk.sign(vote_choice_message)?; // `tau = DS.Sign(registry_key, vote_choice)`

        // Generate the nullifier from the signed id hash to prevent double voting
        let nullifier = poseidon.hash(vec![
            signed_id.r_b8.x,
            signed_id.r_b8.y,
            signed_id.s.into_bigint().into(),
        ])?; // `n = Poseidon(sigma, election_params.identifier)`

        // Generate a random value r that will be used to generate A and B
        // It is important to keep this value secret as it is used to keep the vote choice secret until the reveal phase
        let blinding_factor = BBJJ_Fr::rand(rng);
        // Generate A as a point on the curve corresponding to the random value r
        let a: BBJJ_Ec = BBJJ_G1.mul_scalar(&blinding_factor); // `A = g^r in multiplicative notation`

        // Generate K as a TLock public key to the power of r (in multiplicative notation)
        let k: BBJJ_Ec = tlcs_pk.mul_scalar(&blinding_factor); // `K = PK_t^r in multiplicative notation`

        // Generate B as a hash of the point K, the vote choice and the id of the vote
        // Note that the id of the vote is public, so the moment `k` is revealed, the vote choice can be bruteforced
        let b = poseidon.hash(vec![
            k.x,
            k.y,
            v,
            chain_id[0],
            chain_id[1],
            process_id,
            contract_addr,
        ])?; // `B = Poseidon(K_i, vote_choice, election_params.identifier)`

        return Ok((
            Ballot {
                a: a.clone(),
                b,
                n: nullifier,
                h_id: id_hash,
            },
            BallotHints {
                signed_id,
                signed_v,
                k: k.clone(),
            },
        ));
    }
}

// /// Voters key actions
// impl Voter {
//     /// Registers the voter's public BabyJubJub key on the registry contract
//     pub(crate) fn register_bbjj_key() {
//         unimplemented!()
//     }
//
//     /// Submits a vote to the voting contract
//     pub(crate) fn submit_vote() {
//         unimplemented!()
//     }
// }

#[cfg(test)]
mod test {
    use ethers::core::k256::U256;
    use ethers::types::StorageProof;

    use crate::services::ethereum::StateProof;
    use crate::utils::mock::Mock;
    use crate::utils::{ProcessParameters, VoteChoice};
    use crate::voter::Voter;

    #[test]
    fn test() {
        println!("Hello world")
    }

    #[test]
    fn test_vote_gen() {
        let rng = &mut ark_std::test_rng();

        let voter = Voter::mock(rng);

        let proof = voter.gen_vote(
            U256::from_u64(1),
            VoteChoice::mock(rng),
            &ProcessParameters::mock(rng),
            (StateProof::mock(rng), StateProof::mock(rng)),
            rng,
        );
    }
}
