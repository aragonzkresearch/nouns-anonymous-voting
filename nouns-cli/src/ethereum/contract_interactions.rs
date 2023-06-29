use std::sync::Arc;
use std::time::Duration;

use ethers::core::k256::U256;
use ethers::core::rand;
use ethers::prelude::{abigen, Address, Http, LocalWallet, Provider, SignerMiddleware};

use nouns_protocol::{wrap, wrap_into, BBJJ_Ec, PrivateKey, VoteChoice, Voter, Wrapper};

use crate::ethereum::storage_proofs::{get_nft_ownership_proof, get_zk_registry_proof};
use crate::EthersU256;

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
            function nounsToken() view returns (address)
            function nextProcessId() view returns (uint256)
            function createProcess(uint256 blockDuration,uint256[2] calldata tlcsPublicKey) public returns(uint256)  
            function submitVote(uint256 processId,uint256[2] a,uint256 b,uint256 n,uint256 h_id,bytes calldata proof)
            function getStartBlock(uint256 processId) public view returns (uint256)
        ]"#,
);

abigen!(
    NounsToken,
    r#"[
            function ownerOf(uint256 tokenId) public view virtual returns (address)        
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
    let nouns_voting = NounsVoting::new(contract_address, client);

    // Get amount of blocks for the process duration, rounded up
    let process_duration = EthersU256::from(process_duration.as_secs() / ETH_BLOCK_TIME + 1);

    let create_process_request =
        nouns_voting.create_process(process_duration, wrap_into!(wrap_into!(tlcs_pbk)));

    let tx = create_process_request
        .send()
        .await
        .map_err(|_e| format!("Error sending createProcess tx"))?;

    let process_id = nouns_voting.next_process_id().call().await.map_err(|e| {
        format!("Error getting the next process id from the NounsVoting contract: {e:?}")
    })? - 1;

    println!("Tx Hash: {}", tx.tx_hash());
    println!("Process created successfully with id: {}.", process_id);

    Ok(())
}

/// Function that votes in an existing voting process in the NounsVoting contract.
pub(crate) async fn vote(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    eth_connection: Provider<Http>,
    nouns_voting_address: Address,
    process_id: U256,
    nft_id: U256,
    chain_id: U256,
    bbjj_private_key: PrivateKey,
    vote_choice: VoteChoice,
    tlcs_pbk: BBJJ_Ec,
) -> Result<(), String> {
    let client = Arc::new(client);
    let nouns_voting = NounsVoting::new(nouns_voting_address, client.clone());

    let nouns_token_address = nouns_voting.nouns_token().call().await.map_err(|e| {
        format!("Error getting the NounsToken address from the NounsVoting contract: {e:?}")
    })?;

    let nouns_token = NounsToken::new(nouns_token_address, client.clone());
    let nft_owner = nouns_token
        .owner_of(wrap_into!(nft_id))
        .call()
        .await
        .map_err(|e| {
            format!("Error getting the NounsTokenID from the Nouns Token contract: {e:?}")
        })?;

    let start_block_number = nouns_voting
        .get_start_block(wrap_into!(process_id))
        .call()
        .await
        .map_err(|_| format!("Error getting start block number"))?;

    let zk_registry_address = nouns_voting.zk_registry().call().await.map_err(|e| {
        format!("Error getting the ZKRegistry address from the NounsVoting contract: {e:?}")
    })?;

    let (registry_account_state_hash, registry_account_state_proof_x) = get_zk_registry_proof(
        &eth_connection,
        nft_owner,
        start_block_number,
        zk_registry_address,
    )
    .await?;

    // Check that the storage proof is correct
    let expected_value: [U256; 2] = wrap_into!(bbjj_private_key.public());
    if registry_account_state_proof_x.value != wrap_into!(expected_value[0]) {
        return Err(format!(
            "Error: The BBJJ Public Key X value in the storage proof is not the expected one."
        ));
    }

    let (nft_account_state_hash, nft_account_state_proof) = get_nft_ownership_proof(
        eth_connection,
        wrap_into!(nft_id),
        start_block_number,
        nouns_token_address,
    )
    .await?;

    // Check that the storage proof is correct
    if nft_account_state_proof.value != EthersU256::from_big_endian(&nft_owner.as_bytes()) {
        return Err(format!(
            "Error: The NFT id in the storage proof is not the expected one."
        ));
    }

    let rng = &mut rand::thread_rng();

    let voter = Voter::new(nft_owner, bbjj_private_key);

    let (ballot, proof) = voter
        .gen_vote(
            nft_id,
            vote_choice,
            process_id,
            nouns_voting_address,
            chain_id,
            tlcs_pbk,
            wrap_into!(nft_account_state_hash),
            wrap_into!(registry_account_state_hash),
            (
                nft_account_state_proof.clone(),
                registry_account_state_proof_x.clone(),
            ),
            rng,
        )
        .map_err(|e| format!("Error generating vote proof: {}", e))?;

    let a = wrap_into!(ballot.a);
    let b = wrap_into!(ballot.b);
    let n = wrap_into!(ballot.n);
    let h_id = wrap_into!(ballot.h_id);

    let submit_vote_request = nouns_voting.submit_vote(
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
