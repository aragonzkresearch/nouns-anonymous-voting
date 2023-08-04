use ark_ff::PrimeField;
use ark_std::rand::Rng;
use ark_std::UniformRand;
use babyjubjub_ark::Signature;
use ethers::core::k256::U256;
use ethers::prelude::{Address, StorageProof};
use poseidon_ark::Poseidon;

use crate::utils::VoteChoice;
use crate::{noir, wrap, wrap_into, BBJJ_Ec, BBJJ_Fr, BN254_Fr, Wrapper, BBJJ_G1};

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
#[derive(Clone, Debug)]
pub struct Ballot {
    /// The first part of the encrypted vote
    pub a: BBJJ_Ec,
    /// The second part of the encrypted vote
    pub b: BN254_Fr,
    /// The nullifier of the encrypted vote, to prevent double voting
    pub n: BN254_Fr,
}

/// Represents the hints that were generated while constructing the ballot
/// that the prover needs to generate the proof for ballot correctness
pub(crate) struct BallotHints {
    /// `sigma` representing the signature over the id of the vote
    signed_id: Signature,
    /// `id_hash` is the hash of the NFT ID and election identifiers
    id_hash: BN254_Fr,
    /// `tau` representing the signature over the vote choice
    signed_v: Signature,
    /// `k` representing the blinding factor of the vote
    k: BBJJ_Ec,
    blinding_factor: BBJJ_Fr,
}

impl Voter {
    /// Generate a vote for given parameters
    pub fn gen_vote<R: Rng>(
        &self,
        nft_id: U256,
        v: VoteChoice,
        process_id: U256,
        contract_addr: Address,
        chain_id: U256,
        tlcs_pk: BBJJ_Ec,
        nft_account_state: U256,
        registry_account_state: U256,
        storage_proofs: (StorageProof, StorageProof, StorageProof),
        rng: &mut R,
    ) -> Result<(Ballot, Vec<u8>), String> {
        // Convert the parameters to the correct field
        let nft_id: [BN254_Fr; 2] = wrap_into!(nft_id);
        let process_id: BN254_Fr = wrap_into!(process_id);
        let contract_addr: BN254_Fr = wrap_into!(contract_addr);
        let chain_id: [BN254_Fr; 2] = wrap_into!(chain_id);
        let v = v.into();

        // Prepare the inputs for the Noir circuit vote prover circuit
        let (ballot, ballot_hints) = self.gen_ballot_with_hints(
            nft_id,
            v,
            process_id,
            contract_addr,
            chain_id,
            tlcs_pk.clone(),
            rng,
        )?;

        let noir_input = noir::VoteProverInput {
            // Public inputs
            a: ballot.a.clone(),
            b: ballot.b,
            n: ballot.n,
            process_id,
            contract_addr,
            chain_id,
            registry_account_state: Wrapper(registry_account_state).into(),
            nft_account_state: Wrapper(nft_account_state).into(),
            tlcs_pk: tlcs_pk.clone(),
            // Private inputs
            v,
            blinding_factor: ballot_hints.blinding_factor,
            signed_id: ballot_hints.signed_id,
            voter_address: Wrapper(self.eth_addr).into(),
            signed_v: ballot_hints.signed_v,
            nft_id,
            h_id: ballot_hints.id_hash,
            k: ballot_hints.k,
            registered_pbk: self.registered_sk.public(),
            registry_key_sp: storage_proofs.1,
            nft_ownership_proof: storage_proofs.0,
            delegation_proof: storage_proofs.2,
        };

        let proof = noir::prove_vote(noir_input)?;

        Ok((ballot, proof))
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
            },
            BallotHints {
                signed_id,
                id_hash,
                signed_v,
                k: k.clone(),
                blinding_factor,
            },
        ));
    }
}

#[cfg(test)]
mod test {
    use ethers::core::k256::U256;
    use ethers::types::{Address, StorageProof};
    use rand::Rng;

    use crate::utils::mock::Mock;
    use crate::utils::VoteChoice;
    use crate::voter::Voter;
    use crate::BBJJ_Ec;

    #[test]
    fn test_vote_gen() -> Result<(), String> {
        let rng = &mut ark_std::test_rng();

        let voter = Voter::mock(rng);

        let (ballot, proof) = voter.gen_vote(
            U256::from_u64(1),
            VoteChoice::mock(rng),
            U256::from(rng.gen_range(0..100u8)),
            Address::mock(rng),
            U256::mock(rng),
            BBJJ_Ec::mock(rng),
            U256::mock(rng),
            U256::mock(rng),
            (
                StorageProof::mock(rng),
                StorageProof::mock(rng),
                StorageProof::mock(rng),
            ),
            rng,
        )?;

        println!("Ballot: {:?}", ballot);
        println!("Proof: {:?}", proof);

        Ok(())
    }
}
