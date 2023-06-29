use ::toml::to_string_pretty;
use babyjubjub_ark::Signature;
use ethers::types::StorageProof;

use crate::noir::toml::TomlSerializable;
use crate::{BBJJ_Ec, BBJJ_Fr, BN254_Fr, utils::VoteChoice};

mod toml;

// Useful constants for storage proofs
pub(crate) const MAX_NODE_LEN: usize = 532;
// The maximum byte length of a node
pub(crate) const MAX_DEPTH: usize = 8; // For technical reasons, we need a fixed maximum trie proof size.

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
    pub(crate) registry_account_state: [BN254_Fr; 2],
    pub(crate) nft_account_state: [BN254_Fr; 2],
    pub(crate) tlcs_pk: BBJJ_Ec,

    // Private input for the circuit
    pub(crate) v: BN254_Fr,
    pub(crate) blinding_factor: BBJJ_Fr,
    pub(crate) signed_id: Signature,
    // `sigma`
    pub(crate) voter_address: BN254_Fr,
    pub(crate) signed_v: Signature,
    // `tau`
    pub(crate) nft_id: [BN254_Fr; 2],
    pub(crate) k: BBJJ_Ec,
    /// The public key of the voter's `sk` that is registered in the `BBJJ` interface in the `zkRegistry`
    pub(crate) registered_pbk: BBJJ_Ec,
    pub(crate) registry_key_sp: StorageProof,
    pub(crate) nft_ownership_proof: StorageProof,
}

/// Input to Noir tally circuit
pub(crate) struct TallyProverInput {
    // Public inputs
    pub(crate) b_k: BN254_Fr,
    pub(crate) process_id: BN254_Fr,
    pub(crate) contract_addr: BN254_Fr,
    pub(crate) chain_id: [BN254_Fr; 2],
    pub(crate) vote_count: Vec<usize>, // Deduce num_voters from this
    // Private inputs
    pub(crate) k: Vec<BBJJ_Ec>,
    pub(crate) v: Vec<VoteChoice>
}

/// Generates a proof for a vote
///
/// NOTE: This function is currently reliant on the prover being run in the root of the repo
/// This function is incompatible with the Browser.
///
/// Furthermore, the function makes use of the Filesystem and Shell.
/// For the future, we should consider using a Rust Library implementation of the Noir Prover
/// When such a library is available, we can remove the dependency on the filesystem and shell
pub(crate) fn prove_tally(input: TallyProverInput) -> Result<Vec<u8>, String> {
    let num_voters = input.k.len();
    assert!(num_voters <= 256, "Support for more than 256 voters coming soon™");
    let vote_prover_dir = "circuits/256_voters";

    // Serialize the input into a toml string
    let prover_input = input.toml();

    let prover_input = prover_input
        .as_table()
        .map_or(Err("Failed to serialize input to toml!".to_string()), |t| {
            Ok(t)
        })?;

    let prover_input_as_string = to_string_pretty(&prover_input)
        .map_err(|e| format!("Failed to serialize input to toml! Error {}", e.to_string()))?;

    // Save the input to a file for the prover to read
    let file_path = format!("{}/Prover.toml", vote_prover_dir);
    // If the file does not exist, create it
    if !std::path::Path::new(&file_path).exists() {
        std::fs::File::create(&file_path)
            .map_err(|e| format!("Failed to create input file! Error: {}", e.to_string()))?;
    }
    std::fs::write(file_path, prover_input_as_string)
        .map_err(|e| format!("Failed to write input to file! Error: {}", e.to_string()))?;

    // Run the prover as a shell command `noir prove` in a `noir` subdirectory
    let output = std::process::Command::new("nargo")
        .current_dir(vote_prover_dir)
        .arg("prove")
        .arg("p")
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
    let proof = std::fs::read(vote_prover_dir.to_owned() + "/proofs/p.proof")
        .map_err(|e| format!("Failed to read proof from file! Error: {}", e.to_string()))?;

    Ok(proof)
}

pub(crate) fn prove_vote(input: VoteProverInput) -> Result<Vec<u8>, String> {
    let vote_prover_dir = "circuits/client-proof";

    // Serialize the input into a toml string
    let prover_input = input.toml();

    let prover_input = prover_input
        .as_table()
        .map_or(Err("Failed to serialize input to toml!".to_string()), |t| {
            Ok(t)
        })?;

    let prover_input_as_string = to_string_pretty(&prover_input)
        .map_err(|e| format!("Failed to serialize input to toml! Error {}", e.to_string()))?;

    // Save the input to a file for the prover to read
    let file_path = format!("{}/Prover.toml", vote_prover_dir);
    // If the file does not exist, create it
    if !std::path::Path::new(&file_path).exists() {
        std::fs::File::create(&file_path)
            .map_err(|e| format!("Failed to create input file! Error: {}", e.to_string()))?;
    }
    std::fs::write(file_path, prover_input_as_string)
        .map_err(|e| format!("Failed to write input to file! Error: {}", e.to_string()))?;

    // Run the prover as a shell command `noir prove` in a `noir` subdirectory
    let output = std::process::Command::new("nargo")
        .current_dir(vote_prover_dir)
        .arg("prove")
        .arg("p")
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
    let proof = std::fs::read(vote_prover_dir.to_owned() + "/proofs/p.proof")
        .map_err(|e| format!("Failed to read proof from file! Error: {}", e.to_string()))?;

    Ok(proof)
}
