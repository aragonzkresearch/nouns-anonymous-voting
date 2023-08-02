use babyjubjub_ark::Signature;
use ethers::types::{Address, H256, StorageProof};

use crate::{utils::VoteChoice, BBJJ_Ec, BBJJ_Fr, BlockHeader, BN254_Fr, StateProof};

pub mod toml;

// Maximum byte length for a state or storage proof node
pub const MAX_NODE_LEN: usize = 532;

// Maximum account state size as RLP-encoded byte array
pub const MAX_ACCOUNT_STATE_SIZE: usize = 134;

// Maximum block header size in bytes
pub const MAX_BLOCK_HEADER_SIZE: usize = 630;
    
// The maximum byte length of a node
pub const MAX_DEPTH: usize = 8; // For technical reasons, we need a fixed maximum trie proof size.

/// Input to the Noir block hash checker
pub struct BlockHashVerifierInput {
    pub block_hash: H256,
    pub block_header: BlockHeader,
    pub registry_address: Address,
    pub registry_state_proof: StateProof,
    pub registry_storage_root: H256,
    pub nft_contract_address: Address,
    pub nft_state_proof: StateProof,
    pub nft_storage_root: H256
}
/// The input to the Noir Vote Prover Circuit
pub(crate) struct VoteProverInput {
    // Public input for the circuit
    pub(crate) a: BBJJ_Ec,
    pub(crate) b: BN254_Fr,
    pub(crate) n: BN254_Fr,
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
    pub(crate) h_id: BN254_Fr,
    pub(crate) k: BBJJ_Ec,
    /// The public key of the voter's `sk` that is registered in the `BBJJ` interface in the `zkRegistry`
    pub(crate) registered_pbk: BBJJ_Ec,
    pub(crate) registry_key_sp: StorageProof,
    pub(crate) nft_ownership_proof: StorageProof,
    pub(crate) delegation_proof: StorageProof,
}

/// The input to the Noir Tally Prover Circuit
pub(crate) struct TallyProverInput {
    // Public input for the circuit
    pub(crate) b_k: BN254_Fr,
    pub(crate) process_id: BN254_Fr,
    pub(crate) contract_addr: BN254_Fr,
    pub(crate) chain_id: [BN254_Fr; 2],
    pub(crate) vote_count: [usize; 3],
    // Private inputs
    pub(crate) k: Vec<BBJJ_Ec>,
    pub(crate) v: Vec<VoteChoice>,
}

#[cfg(not(feature = "mock-prover"))]
pub fn prove_block_hash(input: BlockHashVerifierInput) -> Result<Vec<u8>, String> {
    let voter_circuit = include_str!("../../../circuits/hash_proof/src/main.nr");
    let voter_circuit_config_toml = include_str!("../../../circuits/hash_proof/Nargo.toml");

    // Serialize the input into a toml string
    let prover_input = self::toml::TomlSerializable::toml(input);

    let proof = run_singleton_noir_project(voter_circuit_config_toml, voter_circuit, prover_input).expect("Error: Failed to generate proof.");

    Ok(proof)
}

/// Generates a proof for a vote
///
/// Note: This function is incompatible with the browser.
///
/// Furthermore, the function makes use of the filesystem and shell.
/// For the future, we should consider using a Rust Library implementation of the Noir Prover
/// When such a library is available, we can remove the dependency on the filesystem and shell
#[cfg(not(feature = "mock-prover"))]
pub(crate) fn prove_vote(input: VoteProverInput) -> Result<Vec<u8>, String> {
    let voter_circuit = include_str!("../../../circuits/client-proof/src/main.nr");
    let voter_circuit_config_toml = include_str!("../../../circuits/client-proof/Nargo.toml");

    // Serialize the input into a toml string
    let prover_input = self::toml::TomlSerializable::toml(input);

    let proof = run_singleton_noir_project(voter_circuit_config_toml, voter_circuit, prover_input).expect("Error: Failed to generate proof.");

    Ok(proof)
}

#[cfg(not(feature = "mock-prover"))]
pub(crate) fn prove_tally(input: TallyProverInput) -> Result<Vec<u8>, String> {
    let mut tally_circuit = "\
    global MAX_VOTERS: Field = 16;

    fn main(b_k: pub Field, process_id: pub Field, contract_addr: pub Field, chain_id: pub [Field; 2], vote_count: pub [Field; 3], num_voters: Field, k_x: [Field; MAX_VOTERS], k_y: [Field; MAX_VOTERS], v: [Field; MAX_VOTERS])
{
    assert(verify_tally(b_k, process_id, contract_addr, chain_id, vote_count, num_voters, k_x, k_y, v));
}

".to_string();
    tally_circuit.push_str(include_str!("../../../circuits/tally/src/lib.nr"));
    let tally_circuit_config_toml = "[package]
authors = []

[dependencies]";

    let prover_input = self::toml::TomlSerializable::toml(input);

    let proof = run_singleton_noir_project(tally_circuit_config_toml, &tally_circuit, prover_input).expect("Error: Failed to generate proof.");

    Ok(proof)
}

pub fn run_singleton_noir_project(circuit_config_toml: &str, circuit: &str, prover_toml: ::toml::Value) -> Result<Vec<u8>, std::io::Error>
{
    let tmp_dir = tempdir::TempDir::new("nouns")?;
    let circuit_config_toml_path = tmp_dir.path().join("Nargo.toml");
    std::fs::write(circuit_config_toml_path, circuit_config_toml)?;
    
    std::fs::create_dir(tmp_dir.path().join("src"))?;
    let circuit_path = tmp_dir.path().join("src").join("main.nr");
    std::fs::write(circuit_path, circuit)?;

    let prover_toml_path = tmp_dir.path().join("Prover.toml");
    let prover_toml_string = ::toml::to_string_pretty(&prover_toml).expect("Failed to construct Prover.toml.");
    std::fs::write(prover_toml_path, prover_toml_string)?;

    let mut proof_string = std::process::Command::new("nargo")
        .current_dir(tmp_dir.path())
        .arg("prove")
        .output()?.stdout;
    
    proof_string.pop(); // Pop off newline
    let proof = hex::decode(proof_string).expect("Error decoding proof string");
    
    Ok(proof)
}

#[cfg(feature = "mock-prover")]
pub(crate) fn prove_vote(_input: VoteProverInput) -> Result<Vec<u8>, String> {
    let dummy_proof = vec![0; 100];

    Ok(dummy_proof)
}

#[cfg(feature = "mock-prover")]
pub(crate) fn prove_tally(_input: TallyProverInput) -> Result<Vec<u8>, String> {
    let dummy_proof = vec![0; 100];

    Ok(dummy_proof)
}
