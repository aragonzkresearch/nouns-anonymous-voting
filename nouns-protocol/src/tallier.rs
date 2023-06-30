use ark_std::iterable::Iterable;
use babyjubjub_ark::PrivateKey;
use ethers::{core::k256::U256, prelude::Address};
use poseidon_ark::Poseidon;
use strum::IntoEnumIterator;

use crate::noir::TallyProverInput;
use crate::{noir, utils::VoteChoice, wrap, wrap_into, BBJJ_Ec, BBJJ_Fr, BN254_Fr, Wrapper};

/// Results of the tally
pub struct Tally {
    pub vote_count: [usize; 3],
}

/// Represents a tallying authority
/// Note that the tallying authority is stateless as anyone can tally
pub struct Tallier;

/// Truncated Voter Ballot stored on the blockchain
/// Contains key information to decrypt the vote
#[derive(Clone, Debug)]
pub struct TruncatedBallot {
    pub a: BBJJ_Ec,
    pub b: BN254_Fr,
}

impl Tallier {
    /// Function that tallies the votes and returns the [Tally] results
    /// @param ballots: The truncated ballots of the voters who voted
    /// @param tlcs_prk: The TLCS secret key for time T used to decrypt the votes
    /// @param ballot_hash: The hash of the chain of ballots, used to verify that no votes were missed
    /// @param chain_id: The chain id of the blockchain
    /// @param process_id: The process id of the process
    /// @param contract_addr: The address of the contract
    pub fn tally(
        ballots: Vec<TruncatedBallot>,
        tlcs_prk: PrivateKey,
        ballot_hash: BN254_Fr,
        chain_id: U256,
        process_id: U256,
        contract_addr: Address,
    ) -> Result<(Tally, Vec<u8>), String> {
        let process_id: BN254_Fr = wrap_into!(process_id);
        let contract_addr: BN254_Fr = wrap_into!(contract_addr);
        let chain_id: [BN254_Fr; 2] = wrap_into!(chain_id);

        let (vote_choices, tally) = Self::gen_tally_with_hints(
            &ballots,
            &tlcs_prk.scalar_key(),
            process_id,
            contract_addr,
            chain_id,
        )?;

        // Generate a proof
        let noir_input = TallyProverInput {
            // Public inputs
            b_k: ballot_hash,
            process_id,
            contract_addr,
            chain_id,
            vote_count: tally.vote_count,
            // Private inputs
            k: ballots
                .iter()
                .map(|ballot| ballot.a.mul_scalar(&tlcs_prk.scalar_key()))
                .collect(),
            v: vote_choices,
        };

        let proof = noir::prove_tally(noir_input)?;

        Ok((tally, proof))
    }

    fn gen_tally_with_hints(
        ballots: &Vec<TruncatedBallot>,
        tlcs_prk: &BBJJ_Fr,
        process_id: BN254_Fr,
        contract_addr: BN254_Fr,
        chain_id: [BN254_Fr; 2],
    ) -> Result<(Vec<VoteChoice>, Tally), String> {
        let poseidon = Poseidon::new();

        let vote_options = VoteChoice::iter().collect::<Vec<VoteChoice>>();

        // Attempt to decrypt the votes
        let vote_choices = ballots.iter().map(|ballot| {
            let k = ballot.a.mul_scalar(tlcs_prk);

            for vote_candidate in vote_options.clone() {
                let candidate_b = poseidon.hash(vec![
                    k.x,
                    k.y,
                    vote_candidate.into(),
                    chain_id[0],
                    chain_id[1],
                    process_id,
                    contract_addr,
                ])?;

                if candidate_b == ballot.b {
                    return Ok(vote_candidate);
                } else {
                    continue;
                }
            }

            Err("Error: No matching vote!".to_string())
        });

        // Check if any of the votes failed to decrypt and return an error
        let failed_voters: Vec<usize> = vote_choices
            .clone()
            .enumerate()
            .filter(|(_, v)| v.is_err())
            .map(|(i, _)| i)
            .collect();

        if failed_voters.len() > 0 {
            return Err(format!(
                "Failed to decrypt votes for voters: {:?}",
                failed_voters
            ));
        }

        // Calculate the vote count for each vote option
        let vote_choices: Vec<VoteChoice> = vote_choices.map(|v| v.unwrap()).collect();

        let result: Vec<(VoteChoice, usize)> = vote_options
            .iter()
            .map(|v| {
                let amount = vote_choices.iter().filter(|&choice| choice == v).count();

                (*v, amount)
            })
            .collect();

        let vote_count = if vote_options.len() == 3 {
            [result[0].1, result[1].1, result[2].1]
        } else {
            return Err("Error: Invalid number of vote options! Should be 3!".to_string());
        };

        let tally = Tally { vote_count };
        Ok((vote_choices, tally))
    }
}

#[cfg(test)]
mod test {
    use ark_ff::PrimeField;
    use ethers::core::k256::U256;
    use ethers::prelude::Address;
    use poseidon_ark::Poseidon;
    use rand::Rng;

    use crate::tallier::{Tallier, Tally, TruncatedBallot};
    use crate::utils::{mock::Mock, VoteChoice, wrapper::Wrapper};
    use crate::voter::Voter;
    use crate::{BBJJ_Ec, BBJJ_Fr, BBJJ_G1, BN254_Fr, PrivateKey};

    fn gen_tally<R: Rng>(rng: &mut R, num_voters: usize) -> Result<(Tally, Vec<u8>), String> {
        let poseidon = Poseidon::new();

        let nft_id = (0..num_voters).map(|_| { U256::mock(rng) }).collect::<Vec<_>>();
        let process_id = U256::from(rng.gen_range(0..100u8));
        let contract_addr = Address::mock(rng);
        let chain_id = U256::mock(rng);
        let tlcs_prk = BBJJ_Fr::from_be_bytes_mod_order(&PrivateKey::mock(rng).key);
        let tlcs_pk = BBJJ_G1.mul_scalar(&tlcs_prk);

        let voter = (0..num_voters)
            .map(|_| {
                Voter::mock(rng)
            })
            .collect::<Vec<Voter>>();
        
        let v = (0..num_voters)
            .map(|_| { VoteChoice::mock(rng) }).collect::<Vec<_>>();
        
        let ballot = std::iter::zip(voter,std::iter::zip(nft_id, v))
            .map(|(voter, (nft_id, v))| {
                voter.gen_ballot_with_hints(
                    Wrapper(nft_id).into(),
                    (v as u32).into(),
                    Wrapper(process_id).into(),
                    Wrapper(contract_addr).into(),
                    Wrapper(chain_id).into(),
                    tlcs_pk.clone(),
                    rng,
                ).unwrap().0
            }).collect::<Vec<_>>();
        
        let truncated_ballot = ballot
            .iter()
            .map(|ballot| TruncatedBallot {a: ballot.a.clone(), b: ballot.b.clone() })
            .collect::<Vec<_>>();
        
        
        let b_k = truncated_ballot
            .clone()
            .into_iter()
            .map(|tb| {tb.b})
            .reduce(|acc, x| {
                poseidon
                    .hash(vec![x, acc])
                    .expect("Error computing Poseidon hash!")
            })
            .unwrap();

        Tallier::tally(truncated_ballot, tlcs_prk, b_k, chain_id, process_id, contract_addr)
            
//        Tallier::new(a, b, tlcs_sk, b_k)
    }

    #[test]
    fn test10()
    {
        let rng = &mut ark_std::test_rng();

        gen_tally(rng, 10).expect("!?");
    }
    
    #[test]
    fn test_tally_gen() -> Result<(), String> {
        let rng = &mut ark_std::test_rng();

        // Test that the function fails as with random inputs it is unlikely that the votes will be decrypted
        let res = Tallier::tally(
            vec![
                TruncatedBallot {
                    a: BBJJ_Ec::mock(rng),
                    b: BN254_Fr::mock(rng),
                },
                TruncatedBallot {
                    a: BBJJ_Ec::mock(rng),
                    b: BN254_Fr::mock(rng),
                },
                TruncatedBallot {
                    a: BBJJ_Ec::mock(rng),
                    b: BN254_Fr::mock(rng),
                },
            ],
            PrivateKey::mock(rng),
            BN254_Fr::mock(rng),
            U256::mock(rng),
            U256::from(rng.gen_range(0..100u8)),
            Address::mock(rng),
        )
        .is_err();

        assert!(res);

        Ok(())
    }
}
