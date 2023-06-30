use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::k256::U256;
use ethers::core::rand;
use ethers::prelude::{
    abigen, Address, BigEndianHash, Http, LocalWallet, Middleware, Provider, SignerMiddleware,
    TransactionRequest, Wallet,
};
use ethers::types::U64;

use nouns_protocol::{
    wrap, wrap_into, BBJJ_Ec, BBJJ_Fr, PrivateKey, Tallier, TruncatedBallot, VoteChoice, Voter,
    Wrapper,
};

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
            function createProcess(uint64 blockDuration,uint256[2] calldata tlcsPublicKey) public returns(uint256)  
            function submitVote(uint256 processId,uint256[2] a,uint256 b,uint256 n,uint256 h_id,bytes calldata proof)
            function submitTallyResult(uint256 processId,uint256[3] memory tallyResult,bytes calldata proof) public
            function getStartBlock(uint256 processId) public view returns (uint64)
            function getEndBlock(uint256 processId) public view returns (uint64)
            function getBallotsHash(uint256 processId) public view returns (uint256) 
            function getTallyResult(uint256 processId) public view returns (uint256[3] memory)
            
            event BallotCast(uint256 indexed processId, uint256 indexed a_x, uint256 indexed a_y, uint256 indexed b)
        ]"#,
);

abigen!(
    NounsToken,
    r#"[
            function ownerOf(uint256 tokenId) public view virtual returns (address)      
            function balanceOf(address owner) public view virtual returns (uint256)
            function tokenOfOwnerByIndex(address owner, uint256 index) public view virtual returns (uint256)
            function mint() public returns (uint256)   
            function minter() public view returns (address)
        ]"#,
);

/// Function that registers a new BBJJ Public Key in the ZKRegistry contract.
pub async fn reg_key(
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
pub async fn create_process(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    contract_address: Address,
    process_duration: Duration,
    tlcs_pbk: BBJJ_Ec,
) -> Result<(), String> {
    const ETH_BLOCK_TIME: u64 = 12;

    let client = Arc::new(client);
    let nouns_voting = NounsVoting::new(contract_address, client);

    // Get amount of blocks for the process duration, rounded up
    let process_duration = U64::from(process_duration.as_secs() / ETH_BLOCK_TIME + 1);

    let create_process_request =
        nouns_voting.create_process(process_duration.as_u64(), wrap_into!(wrap_into!(tlcs_pbk)));

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
pub async fn vote(
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
        U64::from(start_block_number),
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
        U64::from(start_block_number),
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

/// Function to tally the votes in an existing voting process in the NounsVoting contract.
pub async fn tally(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    nouns_voting_address: Address,
    chain_id: U256,
    process_id: U256,
    tlcs_prk: BBJJ_Fr,
) -> Result<(), String> {
    let client = Arc::new(client);
    let nouns_voting = NounsVoting::new(nouns_voting_address, client.clone());

    // Get all the ballots casted in the voting process
    let process_start_block = nouns_voting
        .get_start_block(wrap_into!(process_id))
        .call()
        .await
        .map_err(|_| format!("Error getting start block number"))
        .unwrap();

    let filter = nouns_voting
        .ballot_cast_filter()
        .filter
        .from_block(U64::from(process_start_block));

    let logs = client.get_logs(&filter).await.map_err(|_| {
        format!(
            "Error getting the logs for the voting process with id {}",
            process_id
        )
    })?;

    let mut ballots: Vec<TruncatedBallot> = Vec::new();
    for log in logs {
        // Check that the log is of correct form:
        if log.topics.len() != 4 {
            return Err(format!(
                "Error: The log with transaction hash {:?} is not of the correct form.",
                log.transaction_hash
            ));
        }

        let a_x: U256 = wrap_into!(log.topics[1].into_uint());
        let a_y: U256 = wrap_into!(log.topics[2].into_uint());
        let b: U256 = wrap_into!(log.topics[3].into_uint());

        let truncated_ballot = TruncatedBallot {
            a: wrap_into!([a_x, a_y]),
            b: wrap_into!(b),
        };

        ballots.push(truncated_ballot);
    }

    // Get the ballot hash
    let ballot_hash = nouns_voting
        .get_ballots_hash(wrap_into!(process_id))
        .call()
        .await
        .map_err(|_| format!("Error getting ballot hash"))
        .unwrap();

    let ballot_hash: U256 = wrap_into!(ballot_hash);

    let (tally, proof) = Tallier::tally(
        ballots,
        tlcs_prk,
        wrap_into!(ballot_hash),
        chain_id,
        process_id,
        nouns_voting_address,
    )?;

    let submit_tally_result_request = nouns_voting.submit_tally_result(
        wrap_into!(process_id),
        tally.vote_count.map(|val| EthersU256::from(val)),
        proof.into(),
    );

    let tx = submit_tally_result_request
        .send()
        .await
        .map_err(|_| format!("Error sending tally tx"))?;

    println!("Tx Hash: {}", tx.tx_hash());
    println!(
        "Tally submitted successfully! The result is: Against: {}, For: {}, Abstain: {}",
        tally.vote_count[0], tally.vote_count[1], tally.vote_count[2]
    );

    Ok(())
}

/// This function will try to help mine the blocks until the specified block number
/// It will do transactions to increase the block number, only valid for local testing
pub async fn mine_blocks_until(
    eth_connection: Provider<Http>,
    wallet_address: Address,
    client: &SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    target_block: u64,
) -> Result<(), String> {
    // Get the current block number
    let mut current_block = eth_connection
        .get_block_number()
        .await
        .expect("Error getting current block number");

    if current_block.as_u64() < target_block {
        println!(
            "Target block not yet reached! Going to mine the next {} blocks.",
            target_block - current_block.as_u64()
        );
    }

    // Check that the process has ended
    while current_block.as_u64() < target_block {
        // Do transactions to increase the block number

        let tx = TransactionRequest::pay(wallet_address, 10);
        let sent_tx = client
            .send_transaction(tx, None)
            .await
            .map_err(|_e| format!("Error sending transaction to increase block number"))?;
        let _receipt = sent_tx.await.map_err(|_e| {
            format!(
                "Error waiting for transaction to increase block number: {:?}",
                _e
            )
        })?;

        current_block = eth_connection
            .get_block_number()
            .await
            .expect("Error getting current block number");
    }
    println!("Target block reached! Mining finished.");
    Ok(())
}

/// Function to obtain the token ids that the user can vote with
/// If the user has no tokens, it will try to mint new ones
pub async fn obtain_token_ids_to_vote(
    wallet_address: Address,
    nouns_voting: NounsVoting<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    client: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
) -> Result<Vec<EthersU256>, String> {
    let client = Arc::new(client);
    // Request from voting contract the nouns token address
    let nouns_token_address = nouns_voting.nouns_token().call().await.unwrap();
    let nouns_token = NounsToken::new(nouns_token_address, client);

    // Get the balance of the account and check if user has any tokens or can mint new ones
    let mut balance = nouns_token.balance_of(wallet_address).call().await.unwrap();
    let minter = nouns_token.minter().call().await.unwrap();

    if minter != wallet_address && balance == EthersU256::zero() {
        Err(
            "User has no tokens and cannot mint new ones. Thus will not be able to vote."
                .to_string(),
        )?;
    }

    // If balance is zero, but can mint, call mint function for nouns token
    if balance == EthersU256::zero() && minter == wallet_address {
        let request = nouns_token.mint();
        let _tx = request.send().await.unwrap();
        balance = nouns_token.balance_of(wallet_address).call().await.unwrap();
    }

    // Print the id of all owned tokens
    let mut token_ids = vec![];
    let mut index = EthersU256::zero();
    while balance > index {
        let token_id = nouns_token
            .token_of_owner_by_index(wallet_address, index)
            .call()
            .await
            .unwrap();

        token_ids.push(token_id);
        index = index.add(EthersU256::from(1));
    }

    Ok(token_ids)
}

#[cfg(test)]
mod test {
    use std::ops::Sub;
    use std::sync::Arc;
    use std::time::Duration;

    use ethers::abi::Address;
    use ethers::core::k256::ecdsa::SigningKey;
    use ethers::middleware::SignerMiddleware;
    use ethers::prelude::{Wallet, H160};
    use ethers::providers::Http;
    use ethers::providers::Provider;
    use ethers::types::U256;

    use nouns_protocol::{wrap, wrap_into, PrivateKey, VoteChoice, Wrapper};

    use crate::ethereum::contract_interactions::{
        create_process, mine_blocks_until, obtain_token_ids_to_vote, reg_key, tally, vote,
        NounsVoting,
    };
    use crate::{setup_connection, setup_env_parameters};

    #[tokio::test]
    async fn integration_test_of_contract_interactions() -> Result<(), String> {
        //// Set the simulation parameters

        let (tx_private_key, rpc_url, voting_address) = setup_env_parameters();

        // Get the TLCS key pair
        let tlcs_prk = PrivateKey::import(vec![0; 32]).expect("Error importing TLCS private key");
        let tlcs_pubk = tlcs_prk.public();

        // Generate the Voter BBJJ Private Key
        // We do it in a function as PrivateKey does not implement Clone
        fn voter_bbjj_prk() -> PrivateKey {
            PrivateKey::import(vec![1; 32]).expect("Error importing Voter BBJJ private key")
        }

        // Set the process duration
        let duration = Duration::from_secs(120); // 1 block confirmation time

        let vote_choice = VoteChoice::Abstain;

        //// Configure the System Parameters

        let (eth_connection, wallet_address, client, chain_id) =
            setup_connection(tx_private_key, rpc_url).await;

        let nouns_voting = NounsVoting::new(voting_address, Arc::new(client.clone()));

        //// Obtain the TokenIds owned by the wallet (possibly minting some)

        let token_ids =
            obtain_token_ids_to_vote(wallet_address, nouns_voting.clone(), client.clone())
                .await
                .map_err(|e| format!("Error obtaining token ids: {}", e))?;

        println!("Token ids: {:?}", token_ids);

        //// 1. Register in zk Registry

        println!("\n\n1. Registering key...\n");

        // Register the key
        reg_key(client.clone(), voting_address, voter_bbjj_prk())
            .await
            .map_err(|e| format!("Error registering key: {}", e))?;

        println!("\nKey registered successfully!\n\n\n");

        //// 2. Create process

        println!("2. Creating process...\n");

        // Start the voting process
        create_process(client.clone(), voting_address, duration, tlcs_pubk.clone())
            .await
            .map_err(|e| format!("Error creating process: {}", e))?;

        // Get the process id
        let next_process_id = nouns_voting
            .next_process_id()
            .call()
            .await
            .expect("Error getting process id");
        let process_id = next_process_id.sub(U256::one());

        //// 3. Submit ballot

        println!(
            "\nProcess created successfully with id {}!\n\n\n",
            process_id
        );
        println!("3. Submitting ballot...\n");

        vote(
            client.clone(),
            eth_connection.clone(),
            voting_address,
            wrap_into!(process_id),
            wrap_into!(token_ids[0]),
            wrap_into!(chain_id),
            voter_bbjj_prk(),
            vote_choice,
            tlcs_pubk,
        )
        .await
        .map_err(|e| format!("Error voting: {}", e))?;

        println!(
            "\nBallot submitted successfully for tokenid {}!\n\n\n",
            token_ids[0]
        );

        //// 4. Tally the results

        println!("4. Tallying...\n");

        let result = simulate_tally(
            voting_address,
            tlcs_prk,
            eth_connection,
            wallet_address,
            chain_id,
            client,
            nouns_voting,
            process_id,
        )
        .await?;

        println!(
            "\n4. Tally result submitted successfully. Result: {} Against, {} For, {} Abstain!\n\n\n",
            result[0], result[1], result[2]
        );

        Ok(())
    }

    async fn simulate_tally(
        voting_address: H160,
        tlcs_prk: PrivateKey,
        eth_connection: Provider<Http>,
        wallet_address: Address,
        chain_id: U256,
        client: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        nouns_voting: NounsVoting<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        process_id: U256,
    ) -> Result<[U256; 3], String> {
        // Get the process end block number
        let process_end_block = nouns_voting
            .get_end_block(process_id)
            .call()
            .await
            .expect("Error getting process end block");

        mine_blocks_until(eth_connection, wallet_address, &client, process_end_block).await?;

        println!("Process has ended! Proceeding to tally.",);

        tally(
            client.clone(),
            voting_address,
            wrap_into!(chain_id),
            wrap_into!(process_id),
            tlcs_prk.scalar_key(),
        )
        .await
        .map_err(|e| format!("Error tallying: {}", e))?;

        let result = nouns_voting
            .get_tally_result(process_id)
            .call()
            .await
            .expect("Error getting tally");
        Ok(result)
    }
}
