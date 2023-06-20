use babyjubjub_ark::Signature;

use crate::services::ethereum::StateProof;
use crate::services::noir::serialisation::toml::TomlSerializable;
use crate::{BBJJ_Ec, BN254_Fr};

mod serialisation;

/// The input to the Noir Vote Prover Circuit
pub(crate) struct VoteProverInput {
    // Public input for the circuit
    pub(crate) a: BBJJ_Ec,
    pub(crate) b: BN254_Fr,
    pub(crate) n: BN254_Fr,
    pub(crate) h_id: BN254_Fr,
    pub(crate) process_id: BN254_Fr,
    pub(crate) contract_addr: BN254_Fr,
    pub(crate) chain_id: [BN254_Fr; 2],
    pub(crate) eth_block_hash: [BN254_Fr; 2],
    pub(crate) tcls_pk: BBJJ_Ec,

    // Private input for the circuit
    pub(crate) v: BN254_Fr,
    pub(crate) signed_id: Signature,
    // `sigma`
    pub(crate) voter_address: BN254_Fr,
    pub(crate) signed_v: Signature,
    // `tau`
    pub(crate) nft_id: [BN254_Fr; 2],
    pub(crate) k: BBJJ_Ec,
    /// The public key of the voter's `sk` that is registered in the `BBJJ` interface in the `zkRegistry`
    pub(crate) registered_pbk: BBJJ_Ec,
    pub(crate) registry_key_sp: StateProof,
    pub(crate) nft_ownership_proof: StateProof,
}

pub(crate) fn prove_vote(input: VoteProverInput) -> Result<Vec<u8>, String> {
    // Serialize the input into a toml string
    let prover_input = input.toml();

    let prover_input = prover_input
        .as_table()
        .map_or(Err("Failed to serialize input to toml!".to_string()), |t| {
            Ok(t)
        })?;

    let prover_input_as_string = toml::to_string_pretty(&prover_input)
        .map_err(|e| format!("Failed to serialize input to toml! Error {}", e.to_string()))?;

    // Save the input to a file for the prover to read
    let file_path = "Prover.toml";
    std::fs::write(file_path, prover_input_as_string)
        .map_err(|e| format!("Failed to write input to file! Error: {}", e.to_string()))?;

    // Run the prover as a shell command `noir prove` in a `noir` subdirectory
    let output = std::process::Command::new("noir")
        .current_dir(".")
        .arg("prove")
        .arg("prover_input.toml")
        .output()
        .map_err(|e| format!("Failed to run noir prover! Error: {}", e.to_string()))?;

    // Check if the prover succeeded
    if !output.status.success() {
        return Err(format!(
            "Noir prover failed! Error: {}",
            String::from_utf8(output.stderr).unwrap()
        ));
    }

    // Read the proof from the file
    let proof = std::fs::read("proof.json")
        .map_err(|e| format!("Failed to read proof from file! Error: {}", e.to_string()))?;

    Ok(proof)
}
