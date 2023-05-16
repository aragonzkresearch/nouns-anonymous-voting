#![allow(non_snake_case)]

extern crate core;

mod preprover;
mod voter;
mod utils;
mod election;
mod serialisation;


/// Define the reexported types from the arkworks libraries to be used in this crate
pub(crate) use babyjubjub_ark::{Point as BBJJ_G1, Signature, PrivateKey as BBJJ_Pr_Key, Fr as BBJJ_Fr};
pub(crate) use ark_bn254::Fr as BN254_Fr;



use crate::election::{ElectionParams, VoteChoice};
use crate::serialisation::toml::TomlSerializable;
use crate::utils::Mock;
use crate::voter::Voter;

use std::fs;
use std::ops::Add;

pub fn run() -> Result<(), String> {

    let mut rng = ark_std::test_rng();
    let voter = Voter::mock(&mut rng);

    let election_params = ElectionParams::mock(&mut rng);
    let nft_id = BN254_Fr::from(0u8);
    let vote_choice = VoteChoice::Yes;

    let vote_package = voter.package_vote_for_proving(&mut rng, &election_params, &vote_choice, &nft_id)?;

    // Debug print the vote package
    println!("vote_package: {:?}", vote_package);

    let toml_private_string = toml::to_string_pretty(&vote_package.private_input.toml()).map_err(|e| format!("Failed to generate toml for private_input: {}", e.to_string()))?;
    let toml_public_string = toml::to_string_pretty(&vote_package.public_input.toml()).map_err(|e| format!("Failed to generate toml for public_input: {}", e.to_string()))?;

    fs::write("Prover.toml", toml_private_string.add(&*toml_public_string.clone())).map_err(|e| e.to_string())?;
    fs::write("Verifier.toml", toml_public_string).map_err(|e| e.to_string())?;

    Ok(())
}