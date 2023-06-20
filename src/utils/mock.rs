// // TODO - only compile this when testing
//
// use crate::voter::Voter;
// use rand::Rng;
// use crate::utils::VoteChoice;
//
// /// Mock trait is used to generate mock data for testing
// pub trait Mock {
//     fn mock<R: Rng>(rng: &mut R) -> Self;
// }
//
// impl Mock for Voter {
//     fn mock<R: Rng>(rng: &mut R) -> Self {
//         // Generate a random Vec of bytes length 32
//         Voter::new(
//             "0x0000000000000000000000000000000000000000"
//                 .parse()
//                 .unwrap(),
//             "0x00".parse().unwrap(),
//             BBJJ_Pr_Key::mock(rng),
//         ) // TODO
//     }
// }
//
// impl Mock for ElectionParams {
//     fn mock<R: Rng>(rng: &mut R) -> Self {
//         ElectionParams {
//             id: ElectionIdentifier::mock(rng),
//             tlock: TLockParams::mock(rng),
//         }
//     }
// }
//
// impl Mock for BBJJ_Pr_Key {
//     fn mock<R: Rng>(rng: &mut R) -> Self {
//         let mut RK_i = vec![0u8; 32];
//         rng.fill_bytes(&mut RK_i);
//         BBJJ_Pr_Key::import(RK_i).expect("Could not generate a mock BBJJ Private Key.")
//     }
// }
//
// impl Mock for VoteChoice {
//     fn mock<R: Rng>(rng: &mut R) -> Self {
//         match rng.gen_range(0..3) {
//             0 => VoteChoice::Yes,
//             1 => VoteChoice::No,
//             2 => VoteChoice::Abstain,
//             _ => panic!("Invalid vote choice"),
//         }
//     }
// }
//
// impl Mock for ElectionIdentifier {
//     fn mock<R: Rng>(rng: &mut R) -> Self {
//         ElectionIdentifier {
//             chain_id: BN254_Fr::from(0u8),
//             process_id: BN254_Fr::from(4u8),
//             contract_addr: BN254_Fr::rand(rng),
//         }
//     }
// }
