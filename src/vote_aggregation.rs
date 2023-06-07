use ark_ff::PrimeField;
use ark_std::rand::Rng;
use poseidon_ark::Poseidon;
use std::str::FromStr;
use strum::IntoEnumIterator;
use toml::Value;

use crate::voter::Voter;
pub(crate) use crate::{
    concat_vec,
    election::{ElectionIdentifier, ElectionParams, TLockParams, VoteChoice},
    preprover::VoteProverPackage,
    serialisation::{toml::TomlSerializable, Wrapper},
    utils::Mock,
    BBJJ_Fr, BBJJ_Pr_Key, BN254_Fr, UniformRand, B8, BBJJ_G1,
};

#[derive(Debug)]
pub struct VoteAggregation {
    pub(crate) b_k: BN254_Fr,
    pub(crate) election_id: ElectionIdentifier,
    pub(crate) vote_count: Vec<usize>,
    pub(crate) k: Vec<BBJJ_G1>,
    pub(crate) v: Vec<VoteChoice>,
}

impl VoteAggregation {
    fn new(
        // Public inputs
        b_k: BN254_Fr,
        election_id: ElectionIdentifier,
        vote_count: Vec<usize>,
        // Witnesses
        k: Vec<BBJJ_G1>,
        v: Vec<VoteChoice>,
    ) -> Self {
        // Ensure `k` and `v` have the same length
        assert!(k.len() == v.len(), "Vector size mismatch");

        // Ensure `vote_count` has length equal to the number of possible choices
        assert!(vote_count.len() == VoteChoice::iter().len());

        Self {
            b_k,
            election_id,
            vote_count,
            k,
            v,
        }
    }

    fn proof_package(
        election_id: ElectionIdentifier,
        a: Vec<BBJJ_G1>,
        b: Vec<BN254_Fr>,
        sk_t: BBJJ_Fr, // TODO
        b_k: BN254_Fr,
    ) -> Self {
        // Compute `k`
        let k = a
            .iter()
            .map(|x| x.mul_scalar(&sk_t))
            .collect::<Vec<BBJJ_G1>>();

        // Determine votes
        let poseidon = Poseidon::new();
        let v = b
            .iter()
            .zip(k.iter())
            .map(|(b, k)| {
                VoteChoice::iter() // TODO: Use strum::EnumIter
                    .filter(|v| {
                        poseidon
                            .hash(concat_vec![
                                <Wrapper<BBJJ_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(k.clone())),
                                vec![
                                    v.clone().into(),
                                    election_id.chain_id,
                                    election_id.process_id,
                                    election_id.contract_addr
                                ]
                            ])
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
        Self::new(b_k, election_id, vote_count, k, v)
    }
    // Mock aggregation with 2000 voters
    fn simulate<R: Rng>(rng: &mut R, num_voters: usize) -> Self {
        let poseidon = Poseidon::new();

        let sk_t = BBJJ_Fr::from_be_bytes_mod_order(&BBJJ_Pr_Key::mock(rng).key);
        let election_params = ElectionParams {
            id: ElectionIdentifier::mock(rng),
            tlock: TLockParams {
                pk_t: B8.mul_scalar(&sk_t),
            },
        };
        let nft_id = BN254_Fr::rand(rng);
        let vote_choice = VoteChoice::Yes;

        let voter_pkg = (0..num_voters)
            .map(|_| {
                let voter = Voter::mock(rng);
                voter
                    .package_vote_for_proving(rng, &election_params, &vote_choice, &nft_id)
                    .expect("Error generating voter proof package.")
            })
            .collect::<Vec<VoteProverPackage>>();

        let a = voter_pkg
            .iter()
            .map(|pkg| pkg.public_input.a.clone())
            .collect::<Vec<BBJJ_G1>>();
        let b = voter_pkg
            .iter()
            .map(|pkg| pkg.public_input.b)
            .collect::<Vec<BN254_Fr>>();
        let election_id = election_params.id;
        let b_k = b
            .clone()
            .into_iter()
            .reduce(|acc, x| {
                poseidon
                    .hash(vec![x, acc])
                    .expect("Error computing Poseidon hash!")
            })
            .unwrap();

        Self::proof_package(election_id, a, b, sk_t, b_k)
    }
}

impl TomlSerializable for VoteAggregation {
    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();

        let num_voters = self.v.len();
        // Figure out whether num_voters is <= 16, 256, 512, 1024, 2048
        let padded_len: usize = [16, 256, 512, 1024, 2048]
            .into_iter()
            .filter(|x| x >= &num_voters)
            .next()
            .expect("Error: There are too many voters.");

        let pad_vec = |v: Vec<BN254_Fr>| {
            v.into_iter()
                .chain(
                    std::iter::repeat(BN254_Fr::from_str("0").unwrap())
                        .take(padded_len - num_voters),
                )
                .collect::<Vec<_>>()
        };

        map.insert("num_voters".to_string(), self.v.len().toml());
        map.insert("b_k".to_string(), self.b_k.toml());
        map.insert("election_id".to_string(), self.election_id.toml());
        map.insert("vote_count".to_string(), self.vote_count.toml());
        map.insert(
            "k_x".to_string(),
            pad_vec(self.k.iter().map(|p| p.x).collect::<Vec<BN254_Fr>>()).toml(),
        );
        map.insert(
            "k_y".to_string(),
            pad_vec(self.k.iter().map(|p| p.y).collect::<Vec<BN254_Fr>>()).toml(),
        );
        map.insert(
            "v".to_string(),
            pad_vec(
                self.v
                    .into_iter()
                    .map(|v| <VoteChoice as Into<BN254_Fr>>::into(v))
                    .collect::<Vec<_>>(),
            )
            .toml(),
        );

        Value::Table(map)
    }
}

#[test]
fn tally10() -> Result<(), String> {
    let mut rng = ark_std::test_rng();
    let va = VoteAggregation::simulate(&mut rng, 10);

    println!("vote_aggregation: {:?}", va);

    let prover_toml_string = toml::to_string_pretty(&va.toml())
        .map_err(|e| format!("Failed to generate Prover.toml: {}", e.to_string()))?;

    // Move to circuit directory
    std::env::set_current_dir("circuits/16_voters").map_err(|e| e.to_string())?;

    // Write Toml file
    std::fs::write("Prover.toml", prover_toml_string).map_err(|e| e.to_string())?;

    // Generate proof
    // std::process::Command::new("nargo")
    //     .arg("prove")
    //     .arg("p")
    //     .status()
    //     .expect("Failed to generate proof.");

    Ok(())
}

#[test]
fn tally200() -> Result<(), String> {
    let mut rng = ark_std::test_rng();
    let va = VoteAggregation::simulate(&mut rng, 200);

    println!("vote_aggregation: {:?}", va);

    let prover_toml_string = toml::to_string_pretty(&va.toml())
        .map_err(|e| format!("Failed to generate Prover.toml: {}", e.to_string()))?;

    // Move to circuit directory
    std::env::set_current_dir("circuits/256_voters").map_err(|e| e.to_string())?;

    // Write Toml file
    std::fs::write("Prover.toml", prover_toml_string).map_err(|e| e.to_string())?;

    // Generate proof
    // std::process::Command::new("nargo")
    //     .arg("prove")
    //     .arg("p")
    //     .status()
    //     .expect("Failed to generate proof.");

    Ok(())
}
