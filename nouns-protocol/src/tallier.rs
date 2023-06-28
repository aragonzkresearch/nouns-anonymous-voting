use crate::{BN254_Fr, BBJJ_Fr, BBJJ_Ec, utils::VoteChoice};

use ark_ff::PrimeField;
use ark_std::rand::Rng;
use ethers::{core::k256::U256, prelude::Address};
use poseidon_ark::Poseidon;
use std::str::FromStr;
use strum::IntoEnumIterator;
use toml::Value;

use crate::{utils::wrapper::Wrapper, voter::Voter};

// TODO: id considered separately

/// Results of the tally to be fed to the tally verification circuit.
pub struct Tally {
    pub(crate) b_k: BN254_Fr,
    pub(crate) vote_count: Vec<usize>,
    pub(crate) k: Vec<BBJJ_Ec>,
    pub(crate) v: Vec<VoteChoice>,
}

/// Data necessary to compute the tally
pub struct Tallier {
    pub(crate) a: Vec<BBJJ_Ec>,
    pub(crate) b: Vec<BN254_Fr>,
    pub(crate) sk_t: BBJJ_Fr, // TODO
    pub(crate) b_k: BN254_Fr,
}

impl Tallier {
    fn new(
        a: Vec<BBJJ_Ec>,
        b: Vec<BN254_Fr>,
        sk_t: BBJJ_Fr,
        b_k: BN254_Fr
    ) -> Self {
        // Ensure the vectors have the same length
        assert!(a.len() == b.len());

        Self { a, b, sk_t, b_k }
    }

    fn tally(
        &self,
        chain_id: U256,
        process_id: U256,
        contract_addr: Address
    ) -> Tally {
        // Compute `k`
        let k = self.a
            .iter()
            .map(|x| x.mul_scalar(&self.sk_t))
            .collect::<Vec<BBJJ_Ec>>();

        // Determine votes
        let poseidon = Poseidon::new();
        let v = self.b
            .iter()
            .zip(k.iter())
            .map(|(b, k)| {
                VoteChoice::iter()
                    .filter(|v| {
                        poseidon
                            .hash(
                                vec![
                                    k.x,
                                    k.y,
                                    BN254_Fr::from(*v as u8),
                                    Wrapper(chain_id).into(), // TODO
                                    Wrapper(process_id).into(),
                                    Wrapper(contract_addr).into()
                                ]
                            )
                            .expect("Error computing Poseidon hash!")
                            .eq(b)
                    })
                    .next()
                    .expect("Error: No matching vote!")
            })
            .collect::<Vec<VoteChoice>>();
        let vote_count = VoteChoice::iter()
            .map(|choice| v.iter().map(|v| if *v == choice { 1 } else { 0 }).sum())
            .collect::<Vec<usize>>();
        Tally
        {
            b_k: self.b_k, vote_count, k, v
        }
    }
}

//     // Mock aggregation with 2000 voters
//     fn simulate<R: Rng>(rng: &mut R, num_voters: usize) -> Self {
//         // TODO: async
//         let poseidon = Poseidon::new();
//
//         let sk_t = BBJJ_Fr::from_be_bytes_mod_order(&BBJJ_Pr_Key::mock(rng).key);
//         let election_params = ElectionParams {
//             id: ElectionIdentifier::mock(rng),
//             tlock: TLockParams {
//                 pk_t: B8.mul_scalar(&sk_t),
//             },
//         };
//         let nft_id = BN254_Fr::rand(rng);
//         let vote_choice = VoteChoice::Yes;
//
//         let voter_pkg = (0..num_voters)
//             .map(|_| {
//                 let voter = Voter::mock(rng);
//                 futures::executor::block_on(voter // TODO: async
//                     .package_vote_for_proving(rng, &election_params, &vote_choice, &nft_id))
//                     .expect("Error generating voter proof package.")
//             })
//             .collect::<Vec<VoteProverPackage>>();
//
//         let a = voter_pkg
//             .iter()
//             .map(|pkg| pkg.public_input.a.clone())
//             .collect::<Vec<BBJJ_Ec>>();
//         let b = voter_pkg
//             .iter()
//             .map(|pkg| pkg.public_input.b)
//             .collect::<Vec<BN254_Fr>>();
//         let election_id = election_params.id;
//         let b_k = b
//             .clone()
//             .into_iter()
//             .reduce(|acc, x| {
//                 poseidon
//                     .hash(vec![x, acc])
//                     .expect("Error computing Poseidon hash!")
//             })
//             .unwrap();
//
//         Self::proof_package(election_id, a, b, sk_t, b_k)
//     }
// }
//
// impl TomlSerializable for VoteAggregation {
//     fn toml(self) -> Value {
//         let mut map = toml::map::Map::new();
//
//         let num_voters = self.v.len();
//         // Figure out whether num_voters is <= 16, 256, 512, 1024, 2048
//         let padded_len: usize = [16, 256, 512, 1024, 2048]
//             .into_iter()
//             .filter(|x| x >= &num_voters)
//             .next()
//             .expect("Error: There are too many voters.");
//
//         let pad_vec = |v: Vec<BN254_Fr>| {
//             v.into_iter()
//                 .chain(
//                     std::iter::repeat(BN254_Fr::from_str("0").unwrap())
//                         .take(padded_len - num_voters),
//                 )
//                 .collect::<Vec<_>>()
//         };
//
//         map.insert("num_voters".to_string(), self.v.len().toml());
//         map.insert("b_k".to_string(), self.b_k.toml());
//         map.insert("election_id".to_string(), self.election_id.toml());
//         map.insert("vote_count".to_string(), self.vote_count.toml());
//         map.insert(
//             "k_x".to_string(),
//             pad_vec(self.k.iter().map(|p| p.x).collect::<Vec<BN254_Fr>>()).toml(),
//         );
//         map.insert(
//             "k_y".to_string(),
//             pad_vec(self.k.iter().map(|p| p.y).collect::<Vec<BN254_Fr>>()).toml(),
//         );
//         map.insert(
//             "v".to_string(),
//             pad_vec(
//                 self.v
//                     .into_iter()
//                     .map(|v| <VoteChoice as Into<BN254_Fr>>::into(v))
//                     .collect::<Vec<_>>(),
//             )
//             .toml(),
//         );
//
//         Value::Table(map)
//     }
// }
//
// #[test]
// fn tally10() -> Result<(), String> {
//     let mut rng = ark_std::test_rng();
//     let va = VoteAggregation::simulate(&mut rng, 10);
//
//     println!("vote_aggregation: {:?}", va);
//
//     let prover_toml_string = toml::to_string_pretty(&va.toml())
//         .map_err(|e| format!("Failed to generate Prover.toml: {}", e.to_string()))?;
//
//     // Move to circuit directory
//     std::env::set_current_dir("circuits/16_voters").map_err(|e| e.to_string())?;
//
//     // Write Toml file
//     std::fs::write("Prover.toml", prover_toml_string).map_err(|e| e.to_string())?;
//
//     // Generate proof
//     // std::process::Command::new("nargo")
//     //     .arg("prove")
//     //     .arg("p")
//     //     .status()
//     //     .expect("Failed to generate proof.");
//
//     Ok(())
// }
//
// #[test]
// fn tally200() -> Result<(), String> {
//     let mut rng = ark_std::test_rng();
//     let va = VoteAggregation::simulate(&mut rng, 200);
//
//     println!("vote_aggregation: {:?}", va);
//
//     let prover_toml_string = toml::to_string_pretty(&va.toml())
//         .map_err(|e| format!("Failed to generate Prover.toml: {}", e.to_string()))?;
//
//     // Move to circuit directory
//     std::env::set_current_dir("circuits/256_voters").map_err(|e| e.to_string())?;
//
//     // Write Toml file
//     std::fs::write("Prover.toml", prover_toml_string).map_err(|e| e.to_string())?;
//
//     // Generate proof
//     // std::process::Command::new("nargo")
//     //     .arg("prove")
//     //     .arg("p")
//     //     .status()
//     //     .expect("Failed to generate proof.");
//
//     Ok(())
// }
//
// struct Tallier {}
//
// impl Tallier {}
