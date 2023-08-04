#![allow(non_snake_case)]

pub use ark_bn254::Fr as BN254_Fr;
pub use ark_ff::PrimeField;
/// Define the reexported types from the arkworks libraries to be used in this crate
pub use babyjubjub_ark::{Fr as BBJJ_Fr, Point as BBJJ_Ec, PrivateKey, B8 as BBJJ_G1};

pub use noir::MAX_DEPTH;
pub use noir::MAX_NODE_LEN;
pub use tallier::{Tallier, TruncatedBallot};
pub use utils::wrapper::Wrapper;
pub use utils::BlockHeader;
pub use utils::StateProof;
pub use utils::VoteChoice;
pub use voter::Voter;

mod utils;

pub mod noir;

mod tallier;
pub mod voter;

// #[cfg(test)]
// mod test {
//     use babyjubjub_ark::PrivateKey;
//     use ethers::abi::Address;
//     use ethers::core::k256::U256;
//     use ethers::prelude::StorageProof;
//     use rand::Rng;

//     use crate::utils::mock::Mock;
//     use crate::{BN254_Fr, Tallier, TruncatedBallot, VoteChoice, Voter};

//     #[test]
//     fn integration_test_of_vote_and_tally() -> Result<(), String> {
//         let rng = &mut ark_std::test_rng();

//         let size = 10;

//         let process_id = U256::from(rng.gen_range(0..100u8));
//         let contract_addr = Address::mock(rng);
//         let chain_id = U256::mock(rng);
//         let tlcs_prk = PrivateKey::mock(rng);
//         let tlcs_pubk = tlcs_prk.public();

//         let mut ballots = vec![];
//         let mut vote_choices = vec![];

//         for _ in 0..size {
//             let voter = Voter::mock(rng);

//             let vote_choice = VoteChoice::mock(rng);

//             let (ballot, _proof) = voter.gen_vote(
//                 U256::from_u64(1),
//                 vote_choice,
//                 process_id,
//                 contract_addr,
//                 chain_id,
//                 tlcs_pubk.clone(),
//                 U256::mock(rng),
//                 U256::mock(rng),
//                 (StorageProof::mock(rng), StorageProof::mock(rng)),
//                 rng,
//             )?;

//             let truncated_ballot = TruncatedBallot {
//                 a: ballot.a.clone(),
//                 b: ballot.b.clone(),
//             };

//             ballots.push(truncated_ballot);
//             vote_choices.push(vote_choice);
//         }

//         let (tally, _proof) = Tallier::tally(
//             ballots,
//             tlcs_prk.scalar_key(),
//             BN254_Fr::mock(rng),
//             chain_id,
//             process_id,
//             contract_addr,
//         )?;

//         let correct_no_amount = vote_choices
//             .iter()
//             .filter(|x| **x == VoteChoice::No)
//             .count();
//         let correct_yes_amount = vote_choices
//             .iter()
//             .filter(|x| **x == VoteChoice::Yes)
//             .count();
//         let correct_abstain_amount = vote_choices
//             .iter()
//             .filter(|x| **x == VoteChoice::Abstain)
//             .count();

//         println!("Tally: {:?}", tally.vote_count);
//         println!(
//             "Expected: {:?}",
//             (
//                 correct_no_amount,
//                 correct_yes_amount,
//                 correct_abstain_amount
//             )
//         );

//         assert_eq!(tally.vote_count[0], correct_no_amount);
//         assert_eq!(tally.vote_count[1], correct_yes_amount);
//         assert_eq!(tally.vote_count[2], correct_abstain_amount);

//         Ok(())
//     }
// }
