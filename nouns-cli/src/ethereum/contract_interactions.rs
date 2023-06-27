use std::sync::Arc;
use std::time::Duration;

use crate::EthersU256;
use crate::PrivateKey;
use crate::Wrapper;
use ethers::core::k256::U256;
use ethers::core::rand;
use ethers::prelude::{
    abigen, Address, Http, LocalWallet, Provider, SignerMiddleware, StorageProof,
};

use nouns_protocol::voter::Voter;
use nouns_protocol::{wrap, wrap_into, BBJJ_Ec, VoteChoice};

abigen!(
    ZKRegistry,
    r#"[
            function register(uint8 interface_id, uint256 value)
        ]"#,
);

abigen!(
    NounsVoting,
    r#"[
            function zkRegistry() view returns (address)
            function createProcess(uint256 nounsTokenStorageRoot,uint256 zkRegistryStorageRoot,uint256 blockDuration,uint256[2] calldata tlcsPublicKey) public returns(uint256)  
            function submitVote(uint256 processId,uint256[2] a,uint256 b,uint256 n,uint256 h_id,bytes calldata proof)
        ]"#,
);

/// Function that registers a new BBJJ Public Key in the ZKRegistry contract.
pub(crate) async fn reg_key(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    nouns_voting_address: Address,
    bbjj_private_key: PrivateKey,
) -> Result<(), String> {
    /// The BBJJ Public Key interface ID in ZKRegistry
    const INTERFACE_ID: u8 = 0x00;

    let client = Arc::new(client);

    let nouns_voting = NounsVoting::new(nouns_voting_address, client.clone());
    let zk_registry_address = nouns_voting.zk_registry().call().await.map_err(|e| {
        format!("Error getting the ZKRegistry address from the NounsVoting contract: {e:?}")
    })?;

    let contract = ZKRegistry::new(zk_registry_address, client);

    let bbjj_pbk = bbjj_private_key.public();
    let bbjj_pbk: [U256; 2] = wrap_into!(bbjj_pbk);

    let x_register_request = contract.register(INTERFACE_ID, wrap_into!(bbjj_pbk[0]));

    let x_tx = x_register_request
        .send()
        .await
        .map_err(|e| format!("Error sending X coordinate registration tx {e:?}"))?;
    println!(
        "Registered X Part on of the BBJJ Public Key  Interface. Tx Hash: {}",
        x_tx.tx_hash()
    );

    let y_register_request = contract.register(INTERFACE_ID + 1, wrap_into!(bbjj_pbk[1]));

    let y_tx = y_register_request
        .send()
        .await
        .map_err(|_e| format!("Error sending Y coordinate registration tx"))?;

    println!(
        "Registered Y Part on of the BBJJ Public Key Interface. Tx Hash: {}",
        y_tx.tx_hash()
    );

    println!("BBJJ Public Key registered successfully!");
    Ok(())
}

/// Function that creates a new voting process in the NounsVoting contract.
pub(crate) async fn create_process(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    contract_address: Address,
    process_duration: Duration,
    tlcs_pbk: BBJJ_Ec,
) -> Result<(), String> {
    const ETH_BLOCK_TIME: u64 = 12;

    let client = Arc::new(client);
    let contract = NounsVoting::new(contract_address, client);

    let nouns_token_storage_root = U256::from_u8(0);
    let zk_registry_storage_root = U256::from_u8(0);

    // Get amount of blocks for the process duration, rounded up
    let process_duration = EthersU256::from(process_duration.as_secs() / ETH_BLOCK_TIME + 1);

    let create_process_request = contract.create_process(
        wrap_into!(nouns_token_storage_root),
        wrap_into!(zk_registry_storage_root),
        process_duration,
        wrap_into!(wrap_into!(tlcs_pbk)),
    );

    let tx = create_process_request
        .send()
        .await
        .map_err(|_e| format!("Error sending createProcess tx"))?;

    println!("Tx Hash: {}", tx.tx_hash());
    println!("Process created successfully!");

    Ok(())
}

/// Function that votes in an existing voting process in the NounsVoting contract.
pub(crate) async fn vote(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    contract_address: Address,
    process_id: U256,
    nft_id: U256,
    chain_id: U256,
    nft_owner: Address,
    bbjj_private_key: PrivateKey,
    vote_choice: VoteChoice,
    tlcs_pbk: BBJJ_Ec,
) -> Result<(), String> {
    let client = Arc::new(client);
    let contract = NounsVoting::new(contract_address, client);

    let voter = Voter::new(nft_owner, bbjj_private_key);

    let nft_account_state_hash = U256::from_u8(0); // TODO - get real value
    let registry_account_state_hash = U256::from_u8(0); // TODO - get real value
    let registry_account_state_proof = StorageProof::default(); // TODO - get real value
    let nft_account_state_proof = StorageProof::default(); // TODO - get real value

    let rng = &mut rand::thread_rng();

    let (ballot, proof) = voter
        .gen_vote(
            nft_id,
            vote_choice,
            process_id,
            contract_address,
            chain_id,
            tlcs_pbk,
            nft_account_state_hash,
            registry_account_state_hash,
            (nft_account_state_proof, registry_account_state_proof),
            rng,
        )
        .map_err(|e| format!("Error generating vote proof: {}", e))?;

    let a = wrap_into!(ballot.a);
    let b = wrap_into!(ballot.b);
    let n = wrap_into!(ballot.n);
    let h_id = wrap_into!(ballot.h_id);

    let submit_vote_request = contract.submit_vote(
        wrap_into!(process_id),
        wrap_into!(a),
        wrap_into!(b),
        wrap_into!(n),
        wrap_into!(h_id),
        proof.into(),
    );

    let tx = submit_vote_request
        .send()
        .await
        .map_err(|_e| format!("Error sending vote tx"))?;

    println!("Tx Hash: {}", tx.tx_hash());
    println!("Vote submitted successfully!");

    Ok(())
}
