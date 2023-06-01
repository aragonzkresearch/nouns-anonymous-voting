#![allow(non_snake_case)]

extern crate core;

mod preprover;
mod voter;
mod utils;
mod election;
mod serialisation;
mod vote_aggregation;

// Useful constants for storage proofs
pub(crate) const MAX_NODE_LEN: usize = 532; // The maximum byte length of a node
pub(crate) const MAX_DEPTH: usize = 8; // For technical reasons, we need a fixed maximum trie proof size.

/// Define the reexported types from the arkworks libraries to be used in this crate
pub(crate) use babyjubjub_ark::{B8, Point as BBJJ_G1, Signature, PrivateKey as BBJJ_Pr_Key, Fr as BBJJ_Fr};
pub(crate) use ark_bn254::Fr as BN254_Fr;



use crate::election::{ElectionParams, VoteChoice};
use crate::serialisation::toml::TomlSerializable;
use crate::serialisation::Wrapper;
use crate::utils::Mock;
use crate::voter::Voter;
use ark_std::UniformRand;


use std::fs;

pub fn run() -> Result<(), String> {

    let mut rng = ark_std::test_rng();
    let voter = Voter::mock(&mut rng);

    let election_params = ElectionParams::mock(&mut rng);
    let nft_id = BN254_Fr::rand(&mut rng);
    let vote_choice = VoteChoice::Yes;

    let vote_package = voter.package_vote_for_proving(&mut rng, &election_params, &vote_choice, &nft_id)?;

    // Debug print the vote package
    println!("vote_package: {:?}", vote_package);

    // Set up Verifier and Prover Tomls
    let public_input_toml = {
        let mut map = vote_package.public_input.toml().as_table().unwrap().clone();
        map.insert("pk_t".to_string(), <Wrapper<BBJJ_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(election_params.tlock.pk_t)).toml());
        toml::Value::Table(map)
    };
    let prover_toml = {
        let mut out_toml = vote_package.private_input.toml().as_table().unwrap().clone();
        out_toml.extend::<toml::Table>(public_input_toml.clone().as_table().unwrap().clone());
        out_toml
};

    let prover_toml_string = toml::to_string_pretty(&prover_toml).map_err(|e| format!("Failed to generate Prover.toml: {}", e.to_string()))?;

    // Move to circuit directory
    std::env::set_current_dir("circuits/client-proof").map_err(|e| e.to_string())?;

    // Write Toml file
    fs::write("Prover.toml", prover_toml_string).map_err(|e| e.to_string())?;

    // Generate proof
    std::process::Command::new("nargo").arg("prove").arg("p").status().expect("Failed to generate proof.");

    Ok(())
}
