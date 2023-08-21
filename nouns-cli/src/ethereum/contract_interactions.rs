use ark_ff::{biginteger::BigInteger256 as B256, BigInt, BigInteger, Field, PrimeField};
use console::Emoji;
use std::ops::Add;
use std::sync::{mpsc, Arc};
use std::thread;

use std::time::Duration;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::k256::U256;
use ethers::core::rand;
use ethers::prelude::{
    abigen, Address, BigEndianHash, Http, LocalWallet, Middleware, Provider, SignerMiddleware,
    TransactionRequest, Wallet,
};
use ethers::types::{H256, U64};

use indicatif::{ProgressBar, ProgressStyle};

use nouns_protocol::{
    wrap, wrap_into, PrivateKey, Tallier, TruncatedBallot, VoteChoice, Voter, Wrapper,
};

use nouns_protocol::noir::BlockHashVerifierInput;
use tokio::runtime::Runtime;

use crate::ethereum::proofs;
use crate::EthersU256;

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

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
            function createProcess(bytes32 ipfsHash, uint64 startDelay, uint64 blockDuration, uint64 tlcsRoundNumber, uint256[2] calldata tlcsPublicKey, uint64 census_block_number, bytes32 registry_storage_root,bytes32 nft_storage_root, bytes calldata hash_proof) public returns(uint256)  
            function submitVote(uint256 processId,uint256[2] a,uint256 b,uint256 n,bytes calldata proof)
            function submitTallyResult(uint256 processId,uint256[3] memory tallyResult,bytes calldata proof) public
            function getIpfsHash(uint256 processId) public view returns (bytes32)
            function getCensusBlock(uint256 processId) public view returns (uint64)
            function getStartBlock(uint256 processId) public view returns (uint64)
            function getEndBlock(uint256 processId) public view returns (uint64)
            function getTlcsRoundNumber(uint256 processId) public view returns (uint64)
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
            function delegate(address delegatee) public
            function delegates(address delegator) public view returns (address)
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

    let (x_tx_hash, y_tx_hash) =
        exec_with_progress("Submitting Baby Jubjub public key to registry", move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let x_register_request = contract.register(INTERFACE_ID, wrap_into!(bbjj_pbk[0]));

                let x_tx = x_register_request
                    .send()
                    .await
                    .map_err(|e| format!("Error sending X coordinate registration tx {e:?}"))?;

                let y_register_request =
                    contract.register(INTERFACE_ID + 1, wrap_into!(bbjj_pbk[1]));

                let y_tx = y_register_request
                    .send()
                    .await
                    .map_err(|_e| format!("Error sending Y coordinate registration tx"))?;
                Ok((x_tx.tx_hash(), y_tx.tx_hash()))
            })
        })?;

    println!(
        "{} Baby Jubjub public key registered successfully (transaction hash {})",
        SPARKLE, x_tx_hash
    );
    Ok(())
}

/// Function that creates a new voting process in the NounsVoting contract.
pub async fn create_process(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    eth_connection: Provider<Http>,
    contract_address: Address,
    ipfs_hash: H256,
    start_delay: Duration,
    process_duration: Duration,
) -> Result<(), String> {
    const ETH_BLOCK_TIME: u64 = 12;

    let client = Arc::new(client);
    let nouns_voting = NounsVoting::new(contract_address, client);

    // Get number of blocks for start delay (rounded up)
    let start_delay = U64::from(start_delay.as_secs() / ETH_BLOCK_TIME + 1);

    // Get number of blocks for the process duration (rounded up)
    let process_duration = U64::from(process_duration.as_secs() / ETH_BLOCK_TIME + 1);

    // Before creating process, need to obtain current state and storage roots for the relevant contracts
    // and submit a proof that these are consistent with the current block hash.

    let (
        census_block_number,
        block_hash,
        block_header,
        nouns_token_address,
        zk_registry_address,
        zk_registry_state_proof,
        nouns_token_contract_state_proof,
        zk_registry_storage_root,
        nouns_token_contract_storage_root,
    ) = exec_with_progress("Fetching data from blockchain", {
        let nouns_voting = nouns_voting.clone();
        move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                // First fetch current block number, block hash, block header and state root
                let census_block_number = eth_connection
                    .get_block_number()
                    .await
                    .map_err(|_| format!("Error getting current block number"))?;
                let block = eth_connection
                    .get_block(census_block_number)
                    .await
                    .map_err(|_| format!("Error obtaining block data"))?
                    .unwrap();
                let block_hash = block.hash.unwrap();
                let block_header = proofs::header_from_block(&block)?;

                // Then fetch state proofs and storage hashes of the relevant contracts
                // First fetch addresses from voting contract
                let nouns_token_address = nouns_voting.nouns_token().call().await.map_err(|e| {
                    format!(
                        "Error getting the NounsToken address from the NounsVoting contract: {e:?}"
                    )
                })?;

                let zk_registry_address = nouns_voting.zk_registry().call().await.map_err(|e| {
                    format!(
                        "Error getting the ZKRegistry address from the NounsVoting contract: {e:?}"
                    )
                })?;

                // Then fetch state proofs
                let zk_registry_state_proof = proofs::get_state_proof(
                    &eth_connection,
                    census_block_number,
                    zk_registry_address,
                )
                .await?;
                let nouns_token_contract_state_proof = proofs::get_state_proof(
                    &eth_connection,
                    census_block_number,
                    nouns_token_address,
                )
                .await?;

                // ...and storage roots
                let zk_registry_storage_root = eth_connection
                    .get_proof(
                        zk_registry_address,
                        vec![],
                        Some(census_block_number.into()),
                    )
                    .await
                    .map_err(|_| "Error fetching storage root")?
                    .storage_hash;
                let nouns_token_contract_storage_root = eth_connection
                    .get_proof(
                        nouns_token_address,
                        vec![],
                        Some(census_block_number.into()),
                    )
                    .await
                    .map_err(|_| "Error fetching storage root")?
                    .storage_hash;
                Ok((
                    census_block_number,
                    block_hash,
                    block_header,
                    nouns_token_address,
                    zk_registry_address,
                    zk_registry_state_proof,
                    nouns_token_contract_state_proof,
                    zk_registry_storage_root,
                    nouns_token_contract_storage_root,
                ))
            })
        }
    })?;

    let proof = exec_with_progress(
        "Generating block hash proof (this might take a while)",
        move || {
            nouns_protocol::noir::prove_block_hash(BlockHashVerifierInput {
                block_hash,
                block_header,
                registry_address: zk_registry_address,
                registry_state_proof: zk_registry_state_proof,
                registry_storage_root: zk_registry_storage_root,
                nft_contract_address: nouns_token_address,
                nft_state_proof: nouns_token_contract_state_proof,
                nft_storage_root: nouns_token_contract_storage_root,
            })
        },
    )?;

    let tlcs_round_number = exec_with_progress("Initiating TLCS key round", {
        move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                tlcs::request_tlcs_key(
                    start_delay.as_u64() * ETH_BLOCK_TIME,
                    process_duration.as_u64() * ETH_BLOCK_TIME,
                )
                .await
            })
        }
    })?;

    let tlcs_pbk_string = exec_with_progress("Waiting for TLCS public key", {
        move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                loop {
                    let keypair = tlcs::get_bjj_keypair_strings(tlcs_round_number).await?;
                    if keypair.0 != "" {
                        return Ok(keypair.0);
                    }
                    thread::sleep(Duration::from_millis(5000));
                }
            })
        }
    })?;
    let tlcs_pbk = crate::parsers::parse_tlcs_pbk(format!(
        "{},{}",
        &tlcs_pbk_string[2..66],
        &tlcs_pbk_string[66..]
    ))?;

    // Pass proof together with state root, storage roots and block number along to process creation request,
    // since the remaining public inputs lie (or may be obtained) within the contract itself

    let (tx_hash, process_id) = exec_with_progress("Submitting data to smart contract", {
        move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let create_process_request = nouns_voting.create_process(
                    ipfs_hash.into(),
                    start_delay.as_u64(),
                    process_duration.as_u64(),
                    tlcs_round_number,
                    wrap_into!(wrap_into!(tlcs_pbk)),
                    census_block_number.as_u64(),
                    zk_registry_storage_root.into(),
                    nouns_token_contract_storage_root.into(),
                    proof.into(),
                );

                let tx = create_process_request
                    .send()
                    .await
                    .map_err(|e| format!("Error sending createProcess tx: {}", e))?;
                let tx_hash = tx.tx_hash();

                let process_id = nouns_voting.next_process_id().call().await.map_err(|e| {
                    format!(
                        "Error getting the next process id from the NounsVoting contract: {e:?}"
                    )
                })? - 1;
                Ok((tx_hash, process_id.to_owned()))
            })
        }
    })?;

    println!(
        "{} Process created successfully with ID {} and TLCS round number {} (transaction hash {})",
        SPARKLE, process_id, tlcs_round_number, tx_hash
    );

    Ok(())
}

/// Function that votes in an existing voting process in the NounsVoting contract.
pub async fn vote(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    eth_connection: Provider<Http>,
    voter_address: Option<Address>, // The address that is enrolled to vote
    nouns_voting_address: Address,
    process_id: U256,
    nft_id: U256,
    chain_id: U256,
    bbjj_private_key: PrivateKey,
    vote_choice: VoteChoice,
) -> Result<(), String> {
    let client = Arc::new(client);
    let nouns_voting = NounsVoting::new(nouns_voting_address, client.clone());

    // TODO: Factor out
    let ipfs_digest = nouns_voting
        .clone()
        .get_ipfs_hash(wrap_into!(process_id))
        .call()
        .await
        .map_err(|e| format!("Error fetching proposal's IPFS CID: {:?}", e))?;
    let ipfs_cid_string = {
        let mut multihash_bytes: Vec<u8> = vec![0x12, 0x20];
        let mut ipfs_digest = ipfs_digest.to_vec();
        multihash_bytes.append(&mut ipfs_digest);
        let cid_multihash = multihash::Multihash::from_bytes(&multihash_bytes)
            .map_err(|e| format!("Error parsing CID multihash bytes: {}", e))?;
        let cid = cid::Cid::new_v1(0x55, cid_multihash);
        cid.to_string_of_base(multibase::Base::Base32Lower)
            .map_err(|e| format!("Could not form CID string: {}", e))?
    };

    let tlcs_pbk = {
        let tlcs_round_number = nouns_voting
            .get_tlcs_round_number(wrap_into!(process_id))
            .call()
            .await
            .map_err(|e| {
                format!(
                    "Error fetching TLCS round number from NounsVoting contract: {:?}",
                    e
                )
            })?;
        let tlcs_pbk_string = exec_with_progress("Fetching TLCS public key", {
            move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    let keypair = tlcs::get_bjj_keypair_strings(tlcs_round_number).await?;
                    if keypair.0 != "" {
                        Ok(keypair.0)
                    } else {
                        Err("TLCS public key unavailable".to_string())
                    }
                })
            }
        })?;
        crate::parsers::parse_tlcs_pbk(format!(
            "{},{}",
            &tlcs_pbk_string[2..66],
            &tlcs_pbk_string[66..]
        ))?
    };
    println!(
        "Voting \"{}\" to proposal ipfs://{}",
        vote_choice, ipfs_cid_string
    );

    let (
        voter_address,
        registry_account_state_hash,
        registry_account_state_proof_x,
        nft_account_state_hash,
        nft_account_state_proof,
        delegation_proof,
    ) = exec_with_progress("Fetching data from blockchain", {
        let bbjj_private_key = PrivateKey {
            key: bbjj_private_key.key.clone(),
        };
        let nouns_voting = nouns_voting.clone();
        move || {
            let rt = Runtime::new().unwrap();
            rt.block_on( async {
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

                                         // If no voter address was specified, assume it is the NFT owner.
                                         let voter_address = voter_address.unwrap_or(nft_owner);

                                         let census_block_number = nouns_voting
                                             .get_census_block(wrap_into!(process_id))
                                             .call()
                                             .await
                                             .map_err(|_| format!("Error getting census block number"))?;

                                         let zk_registry_address = nouns_voting.zk_registry().call().await.map_err(|e| {
                                             format!("Error getting the ZKRegistry address from the NounsVoting contract: {e:?}")
                                         })?;

                                         let (registry_account_state_hash, registry_account_state_proof_x) = proofs::get_zk_registry_proof(
                                             &eth_connection,
                                             voter_address,
                                             U64::from(census_block_number),
                                             zk_registry_address,
                                         )
                                             .await?;

                                         // Check that the storage proof is correct
                                         // TODO: Also check roots against contract
                                         let expected_value: [U256; 2] = wrap_into!(bbjj_private_key.public());
                                         if registry_account_state_proof_x.value != wrap_into!(expected_value[0]) {
                                             return Err(format!(
                                                 "The public key you specified is invalid or does not exist. Are you sure you enrolled to vote?"
                                             ));
                                         }

                                         // Fetch NFT ownership proof
                                         let (nft_account_state_hash, nft_account_state_proof) = proofs::get_nft_ownership_proof(
                                             eth_connection.clone(),
                                             wrap_into!(nft_id),
                                             U64::from(census_block_number),
                                             nouns_token_address,
                                         )
                                             .await?;

                                         // ...as well as delegation proof
                                         let (_, delegation_proof) = proofs::get_delegation_proof(
                                             eth_connection,
                                             nft_owner,
                                             U64::from(census_block_number),
                                             nouns_token_address,
                                         )
                                             .await?;

                                         // Check that the NFT ownership proof implies that voter_address is the owner
                                         // *or* nft_owner has delegated to voter_address
                                         if (nft_account_state_proof.value != EthersU256::from_big_endian(&nft_owner.as_bytes())) & (delegation_proof.value != EthersU256::from_big_endian(&voter_address.as_bytes())) {
                                             return Err(format!(
                                                 "Error: The voter is neither the owner of the NFT nor its delegate."
                                             ));
                                         }
                                         Ok((voter_address,
                                             registry_account_state_hash, registry_account_state_proof_x,
                                             nft_account_state_hash, nft_account_state_proof, delegation_proof))
                                     })
        }
    })?;

    let (ballot, proof) = exec_with_progress(
        "Generating vote proof (this might take a while)",
        move || {
            let rng = &mut rand::thread_rng();

            let voter = Voter::new(voter_address, bbjj_private_key);

            voter
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
                        delegation_proof.clone(),
                    ),
                    rng,
                )
                .map_err(|e| format!("Error generating vote proof: {}", e))
        },
    )?;

    let tx_hash = exec_with_progress("Submitting data to smart contract", move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let a = wrap_into!(ballot.a);
            let b = wrap_into!(ballot.b);
            let n = wrap_into!(ballot.n);

            let submit_vote_request = nouns_voting.submit_vote(
                wrap_into!(process_id),
                wrap_into!(a),
                wrap_into!(b),
                wrap_into!(n),
                proof.into(),
            );

            let tx = submit_vote_request
                .send()
                .await
                .map_err(|e| format!("Error sending vote tx: {}", e))?;

            Ok(tx.tx_hash())
        })
    })?;

    println!(
        "{} Vote submitted successfully (transaction hash {})",
        SPARKLE, tx_hash
    );

    Ok(())
}

/// Function to tally the votes in an existing voting process in the NounsVoting contract.
pub async fn tally(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    nouns_voting_address: Address,
    chain_id: U256,
    process_id: U256,
) -> Result<(), String> {
    let client = Arc::new(client);
    let nouns_voting = NounsVoting::new(nouns_voting_address, client.clone());

    // Fetch TLCS private key
    let tlcs_prk = {
        let tlcs_prk_string = exec_with_progress("Fetching TLCS private key", {
            let nouns_voting = nouns_voting.clone();
            move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    let round_number = nouns_voting
                        .get_tlcs_round_number(wrap_into!(process_id))
                        .call()
                        .await
                        .map_err(|e| {
                            format!(
                                "Error fetching TLCS round number from NounsVoting contract: {:?}",
                                e
                            )
                        })?;
                    // Fetch LOE data, triggering computation of the private key.
                    tlcs::fetch_loe_data(round_number).await?;
                    // Wait for private key computation
                    let mut priv_key: Result<String, String> = Err("The TLCS private key cannot be obtained.".to_string());
                    for _i in 0..10
                    {
                        let keypair_strings = tlcs::get_bjj_keypair_strings(round_number).await?;
                        if keypair_strings.1 != "" {
                            priv_key = Ok(keypair_strings.1);
                            break;
                        }
                        
                        thread::sleep(Duration::from_millis(tlcs::LOE_DELAY));
                    }
                    
                    priv_key
                })
            }
        })?;
        crate::parsers::parse_bbjj_prk(&tlcs_prk_string)
    }?;

    // Get all the ballots casted in the voting process
    let (ballots, ballot_hash) = exec_with_progress("Fetching ballots from blockchain", {
        let nouns_voting = nouns_voting.clone();
        move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
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

                    let log_process_id: U256 = wrap_into!(H256::from(<[u8;32]>::try_from(log.data.as_ref()).map_err(|e| format!("Could not convert log process ID to byte array: {}", e))?).into_uint());
                    if log_process_id == process_id
                    {
                        let a_x: U256 = wrap_into!(log.topics[1].into_uint());
                        let a_y: U256 = wrap_into!(log.topics[2].into_uint());
                        let b: U256 = wrap_into!(log.topics[3].into_uint());

                        let truncated_ballot = TruncatedBallot {
                            a: wrap_into!([a_x, a_y]),
                            b: wrap_into!(b),
                        };

                        ballots.push(truncated_ballot);
                    }
                }
                
                // Get the ballot hash
                let ballot_hash = nouns_voting
                    .get_ballots_hash(wrap_into!(process_id))
                    .call()
                    .await
                    .map_err(|_| format!("Error getting ballot hash"))
                    .unwrap();

                let ballot_hash: U256 = wrap_into!(ballot_hash);

                Ok((ballots, ballot_hash))
            })
        }
    })?;

    let (tally, proof) = exec_with_progress(
        "Generating tally proof (this might take a while)",
        move || {
            Tallier::tally(
                ballots,
                tlcs_prk,
                wrap_into!(ballot_hash),
                chain_id,
                process_id,
                nouns_voting_address,
            )
        },
    )?;

    let tx_hash = exec_with_progress("Submitting results to smart contract", move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let submit_tally_result_request = nouns_voting.submit_tally_result(
                wrap_into!(process_id),
                tally.vote_count.map(|val| EthersU256::from(val)),
                proof.into(),
            );

            let tx = submit_tally_result_request
                .send()
                .await
                .map_err(|e| format!("Error sending tally tx: {}", e))?;

            Ok(tx.tx_hash())
        })
    })?;

    println!(
        "Tally submitted successfully (transaction hash {}) with the following results:\nFor: {}, Against: {}, Neutral: {}",
        tx_hash,
        tally.vote_count[1], tally.vote_count[0], tally.vote_count[2]
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
    //    if balance == EthersU256::zero() && minter == wallet_address {
    if minter == wallet_address {
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

/// Function to delegate tokens to another address
pub async fn delegate_tokens(
    wallet_address: Address,
    delegate_address: Address,
    nouns_voting: NounsVoting<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    client: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
) -> Result<Address, String> {
    let client = Arc::new(client);
    // Request from voting contract the nouns token address
    let nouns_token_address = nouns_voting.nouns_token().call().await.unwrap();
    let nouns_token = NounsToken::new(nouns_token_address, client);

    // Get the balance of the account and check if user has any tokens
    let balance = nouns_token.balance_of(wallet_address).call().await.unwrap();

    if balance == EthersU256::zero() {
        Err("User has no tokens.".to_string())?;
    }

    nouns_token.delegate(delegate_address).send().await.unwrap();

    // Confirm delegation
    let delegates_req = nouns_token.delegates(wallet_address);
    let delegates = delegates_req.call().await.unwrap();

    Ok(delegates)
}

//#[cfg(test)]
// mod test {
//     use std::ops::Sub;
//     use std::sync::Arc;
//     use std::time::Duration;

//     use ethers::abi::Address;
//     use ethers::core::k256::ecdsa::SigningKey;
//     use ethers::middleware::SignerMiddleware;
//     use ethers::prelude::{Wallet, H160};
//     use ethers::providers::Http;
//     use ethers::providers::Provider;
//     use ethers::types::U256;

//     use nouns_protocol::{wrap, wrap_into, PrivateKey, VoteChoice, Wrapper};

//     use crate::ethereum::contract_interactions::{
//         create_process, mine_blocks_until, obtain_token_ids_to_vote, reg_key, tally, vote,
//         NounsVoting,
//     };
//     use crate::{setup_connection, setup_env_parameters};

//     #[tokio::test]
//     async fn integration_test_of_contract_interactions() -> Result<(), String> {
//         //// Set the simulation parameters

//         let (tx_private_key, rpc_url, voting_address) = setup_env_parameters();

//         // Get the TLCS key pair
//         let tlcs_prk = PrivateKey::import(vec![0; 32]).expect("Error importing TLCS private key");
//         let tlcs_pubk = tlcs_prk.public();

//         // Generate the Voter BBJJ Private Key
//         // We do it in a function as PrivateKey does not implement Clone
//         fn voter_bbjj_prk() -> PrivateKey {
//             PrivateKey::import(vec![1; 32]).expect("Error importing Voter BBJJ private key")
//         }

//         // Set the process duration
//         let duration = Duration::from_secs(120); // 1 block confirmation time

//         let vote_choice = VoteChoice::Abstain;

//         //// Configure the System Parameters

//         let (eth_connection, wallet_address, client, chain_id) =
//             setup_connection(tx_private_key, rpc_url).await;

//         let nouns_voting = NounsVoting::new(voting_address, Arc::new(client.clone()));

//         //// Obtain the TokenIds owned by the wallet (possibly minting some)

//         let token_ids =
//             obtain_token_ids_to_vote(wallet_address, nouns_voting.clone(), client.clone())
//                 .await
//                 .map_err(|e| format!("Error obtaining token ids: {}", e))?;

//         println!("Token ids: {:?}", token_ids);

//         //// 1. Register in zk Registry

//         println!("\n\n1. Registering key...\n");

//         // Register the key
//         reg_key(client.clone(), voting_address, voter_bbjj_prk())
//             .await
//             .map_err(|e| format!("Error registering key: {}", e))?;

//         println!("\nKey registered successfully!\n\n\n");

//         //// 2. Create process

//         println!("2. Creating process...\n");

//         // Start the voting process
//         create_process(client.clone(), voting_address, duration, tlcs_pubk.clone())
//             .await
//             .map_err(|e| format!("Error creating process: {}", e))?;

//         // Get the process id
//         let next_process_id = nouns_voting
//             .next_process_id()
//             .call()
//             .await
//             .expect("Error getting process id");
//         let process_id = next_process_id.sub(U256::one());

//         //// 3. Submit ballot

//         println!(
//             "\nProcess created successfully with id {}!\n\n\n",
//             process_id
//         );
//         println!("3. Submitting ballot...\n");

//         vote(
//             client.clone(),
//             eth_connection.clone(),
//             voting_address,
//             wrap_into!(process_id),
//             wrap_into!(token_ids[0]),
//             wrap_into!(chain_id),
//             voter_bbjj_prk(),
//             vote_choice,
//             tlcs_pubk,
//         )
//         .await
//         .map_err(|e| format!("Error voting: {}", e))?;

//         println!(
//             "\nBallot submitted successfully for tokenid {}!\n\n\n",
//             token_ids[0]
//         );

//         //// 4. Tally the results

//         println!("4. Tallying...\n");

//         let result = simulate_tally(
//             voting_address,
//             tlcs_prk,
//             eth_connection,
//             wallet_address,
//             chain_id,
//             client,
//             nouns_voting,
//             process_id,
//         )
//         .await?;

//         println!(
//             "\n4. Tally result submitted successfully. Result: {} Against, {} For, {} Abstain!\n\n\n",
//             result[0], result[1], result[2]
//         );

//         Ok(())
//     }

//     async fn simulate_tally(
//         voting_address: H160,
//         tlcs_prk: PrivateKey,
//         eth_connection: Provider<Http>,
//         wallet_address: Address,
//         chain_id: U256,
//         client: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
//         nouns_voting: NounsVoting<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
//         process_id: U256,
//     ) -> Result<[U256; 3], String> {
//         // Get the process end block number
//         let process_end_block = nouns_voting
//             .get_end_block(process_id)
//             .call()
//             .await
//             .expect("Error getting process end block");

//         mine_blocks_until(eth_connection, wallet_address, &client, process_end_block).await?;

//         println!("Process has ended! Proceeding to tally.",);

//         tally(
//             client.clone(),
//             voting_address,
//             wrap_into!(chain_id),
//             wrap_into!(process_id),
//             tlcs_prk.scalar_key(),
//         )
//         .await
//         .map_err(|e| format!("Error tallying: {}", e))?;

//         let result = nouns_voting
//             .get_tally_result(process_id)
//             .call()
//             .await
//             .expect("Error getting tally");
//         Ok(result)
//     }
// }

fn exec_with_progress<
    F: FnOnce() -> Result<T, String> + std::marker::Send + 'static,
    T: std::marker::Send + 'static,
>(
    msg: &'static str,
    f: F,
) -> Result<T, String> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(f()).unwrap();
    });

    let pb = ProgressBar::new(1024);
    pb.set_style(ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}").unwrap());
    pb.set_message(msg);

    loop {
        if let Ok(x) = rx.try_recv() {
            return x;
        }
        thread::sleep(Duration::from_millis(50));
        pb.inc(1);
    }
}

pub(crate) mod tlcs {
    use serde::{Deserialize, Serialize};
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    const NEW_ROUND_API: &str = "https://demo.timelock.zone/newround/";
    const LOE_DATA_API: &str = "https://demo.timelock.zone/loedata/";
    const KEYPAIR_API: &str = "https://api.timelock.zone/azkr/tlcs/v1beta1/keypairs/round/";
    pub(crate) const LOE_DELAY: u64 = 10000;

    #[derive(Serialize, Deserialize, Debug)]
    struct Keypairs {
        keypairs: Vec<Roundkeypair>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Roundkeypair {
        round: u64,
        scheme: u64,
        public_key: String,
        private_key: String,
    }

    fn seconds_since_loe_epoch() -> u64 {
        let loe_epoch: SystemTime = UNIX_EPOCH
            .checked_add(Duration::from_secs(1677685200))
            .unwrap();

        SystemTime::now()
            .duration_since(loe_epoch)
            .expect("Have we built a time machine?")
            .as_secs()
    }

    /// This function requests a TLCS keypair.
    pub(crate) async fn request_tlcs_key(
        start_delay: u64,
        process_duration: u64,
    ) -> Result<u64, String> {
        let t = seconds_since_loe_epoch();

        // A new round starts every 3 seconds.
        let round_number = (t + start_delay + process_duration) / 3;

        // Send request for public/private key pair for particular round number
        // TODO: Parse at least some of the body.
        reqwest::get(NEW_ROUND_API.to_string() + &round_number.to_string())
            .await
            .map_err(|e| format!("{:?}", e))?
            .text()
            .await
            .map_err(|e| format!("{:?}", e))?;

        Ok(round_number)
    }

    /// This function fetches LOE data, which is required for computing the TLCS private key (server side).
    pub(crate) async fn fetch_loe_data(
        round_number: u64
    ) -> Result<(), String> {
        reqwest::get(LOE_DATA_API.to_string() + &round_number.to_string())
            .await
            .map_err(|e| format!("{:?}", e))?
            .text()
            .await
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    /// This function fetches the TLCS keypair as strings.
    pub(crate) async fn get_bjj_keypair_strings(
        round_number: u64,
    ) -> Result<(String, String), String> {
        // Request key pairs for round
        let keypairs = reqwest::get(KEYPAIR_API.to_string() + &round_number.to_string())
            .await
            .map_err(|e| format!("{:?}", e))?
            .json::<Keypairs>()
            .await
            .map_err(|e| format!("Invalid response to public key request: {}", e))?
            .keypairs;

        // Filter out those for our scheme
        let bjj_keypair = keypairs
            .into_iter()
            .filter(|x| x.scheme == 1)
            .next()
            .ok_or("No suitable keypair found.")?;

        Ok((bjj_keypair.public_key, bjj_keypair.private_key))
    }
}
