extern crate core;

use std::env;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use babyjubjub_ark::PrivateKey;
use clap::{command, Arg, Command};
use ethers::core::k256::U256;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::{abigen, Signer, U256 as EthersU256};
use ethers::providers::{Http, Middleware, Provider, ProviderExt};
use ethers::signers::LocalWallet;
use ethers::types::{Address, StorageProof};

use nouns_anonymous_voting::voter::Voter;
use nouns_anonymous_voting::{wrap, wrap_into, BBJJ_Ec, VoteChoice, Wrapper};
use parsers::*;

/// The CLI that to interact with the Nouns Anonymous Voting System
/// As global parameters, it should take:
/// 1. The EVM Address of the NounsVoting contract (could be also passed as an environmental variable)
/// 2. The RPC URL of the target EVM blockchain (could be also passed as an environmental variable)
/// 3. The Private Key of the account that will be used to send the transactions (should be passed as an environmental variable or as a password input)
///
/// The CLI will have 3 commands, which the user will choose from:
/// 1. `reg-key`
/// 2. `create-process`
/// 3. `vote`
/// 4. `tally`
///
/// The `reg-key` command registers a new BBJJ Public Key in the ZKRegistry contract.
/// It should ask the user for the following additional information:
/// 1. The BBJJ Private Key that will be registered in the ZKRegistry to the account that owns the NFT
/// Note that the Account that sends the transaction should be the owner of the NFT
///
/// The `create-process` command creates a new voting process in the NounsVoting contract.
/// It should ask the user for the following additional information:
/// 1. Process Duration (in minutes/hours/days)
/// 2. TLCS Public Key for the process end time used to encrypt the ballots
/// In future versions, we could allow to also pass the action to be executed after the process ends.
///
/// The `vote` command allows the user to vote in an existing voting process.
/// It should ask the user for the following additional information:
/// 1. The Voting Process ID
/// 2. The NFT ID of the Nouns to vote for
/// 3. The NFT Owner Address
/// 4. The BBJJ Private Key mapped in the ZKRegistry to the account that owns the NFT
/// 5. The Vote Choice (Yes/No/Abstain)
/// 6. TLCS Public Key for the process end time used to encrypt the ballots
///
/// The `tally` command allows the user to generate the tally for an existing voting process.
/// It then submits the result to the NounsVoting contract with the proof of the tally.
/// It should ask the user for the following additional information:
/// 1. The Voting Process ID it is generating the results for
/// 2. The TLCS Private Key corresponding to the TLCS Public Key used to encrypt the Ballots
///
#[tokio::main]
async fn main() {
    // parse the CLI input
    let (global_param, cli_command) = get_user_input().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    // connect to the EVM
    let eth_connection = Provider::<Http>::connect(global_param.rpc_url.as_str()).await;
    // create the signer for the txs
    let wallet = LocalWallet::from_bytes(global_param.tx_private_key.as_ref()).unwrap();
    let chain_id = eth_connection
        .get_chainid()
        .await
        .map_err(|_e| "Could not get chain id".to_string())
        .unwrap();

    let client = SignerMiddleware::new(
        eth_connection.clone(),
        wallet.with_chain_id(chain_id.as_u64()),
    );

    match cli_command {
        CliCommand::RegKey(bbjj_private_key) => {
            reg_key(client, global_param.contract_address, bbjj_private_key).await
        }
        CliCommand::CreateProcess(process_duration, tlcs_pbk) => {
            create_process(
                client,
                global_param.contract_address,
                process_duration,
                tlcs_pbk,
            )
            .await
        }
        CliCommand::Vote(
            process_id,
            nft_id,
            nft_owner,
            bbjj_private_key,
            vote_choice,
            tlcs_pbk,
        ) => {
            vote(
                client,
                global_param.contract_address,
                process_id,
                nft_id,
                wrap_into!(chain_id),
                nft_owner,
                bbjj_private_key,
                vote_choice,
                tlcs_pbk,
            )
            .await
        }
        _ => unimplemented!(),
    }
    .unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    println!("Done!");
}

/// Function that registers a new BBJJ Public Key in the ZKRegistry contract.
async fn reg_key(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    nouns_voting_address: Address,
    bbjj_private_key: PrivateKey,
) -> Result<(), String> {
    /// The BBJJ Public Key interface ID in ZKRegistry
    const INTERFACE_ID: u8 = 0x00;

    let client = Arc::new(client);

    abigen!(
        NounsVoting,
        r#"[
            function zkRegistry() view returns (address)
        ]"#,
    );

    let nouns_voting = NounsVoting::new(nouns_voting_address, client.clone());
    let zk_registry_address = nouns_voting.zk_registry().call().await.map_err(|e| {
        format!("Error getting the ZKRegistry address from the NounsVoting contract: {e:?}")
    })?;

    abigen!(
        ZKRegistry,
        r#"[
            function register(uint8 interface_id, uint256 value)
        ]"#,
    );

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
async fn create_process(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    contract_address: Address,
    process_duration: Duration,
    tlcs_pbk: BBJJ_Ec,
) -> Result<(), String> {
    const ETH_BLOCK_TIME: u64 = 12;

    abigen!(
        NounsVoting,
        r#"[
            function createProcess(uint256 nounsTokenStorageRoot,uint256 zkRegistryStorageRoot,uint256 blockDuration,uint256[2] calldata tlcsPublicKey) public returns(uint256)  
          ]"#,
    );

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
async fn vote(
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
    abigen!(
        NounsVoting,
        r#"[
            function submitVote(uint256 processId,uint256[2] a,uint256 b,uint256 n,uint256 h_id,bytes calldata proof)
            ]"#,
    );

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

/// The global parameters of the CLI
struct GlobalCliParams {
    contract_address: Address,
    rpc_url: String,
    tx_private_key: [u8; 32],
}

/// The CLI commands that the user can choose from
/// Parameters are passed as arguments and the command is executed
enum CliCommand {
    RegKey(PrivateKey),
    CreateProcess(Duration, BBJJ_Ec),
    Vote(U256, U256, Address, PrivateKey, VoteChoice, BBJJ_Ec),
    Tally(U256, PrivateKey),
    None, // No command was chosen
}

fn get_user_input() -> Result<(GlobalCliParams, CliCommand), String> {
    let command = command_constructor();
    let matches = command.get_matches();

    // Parse the global parameters
    let global_cli_param = {
        let contract_address: &String = matches
            .get_one("contract-address")
            .ok_or("Missing contract address")?;
        let rpc_url: &String = matches.get_one("rpc-url").ok_or("Missing RPC URL")?;
        let tx_private_key: &String = matches
            .get_one("tx-private-key")
            .ok_or("Missing transaction private key")?;

        let contract_address = Address::from_str(contract_address)
            .map_err(|e| format!("Invalid contract address: {}", e))?;
        let tx_private_key =
            hex::decode(tx_private_key).map_err(|e| format!("Invalid private key: {}", e))?;
        let tx_private_key = <[u8; 32]>::try_from(tx_private_key.deref())
            .map_err(|e| format!("Invalid private key: {}", e))?;

        GlobalCliParams {
            contract_address,
            rpc_url: String::from(rpc_url),
            tx_private_key,
        }
    };

    if let Some(matches) = matches.subcommand_matches("reg-key") {
        let key_to_reg: &String = matches
            .get_one("reg-private-key")
            .ok_or("Missing key to register in ZKRegistry")?;

        let key_to_reg =
            parse_bbjj_prk(key_to_reg).map_err(|_e| "Invalid key to register in ZKRegistry")?;

        return Ok((global_cli_param, CliCommand::RegKey(key_to_reg)));
    }

    // Parse the command `create-process`
    if let Some(matches) = matches.subcommand_matches("create-process") {
        let process_duration: &String = matches
            .get_one("process-duration")
            .ok_or("Missing process duration")?;
        let tlcs_pbk: &String = matches
            .get_one("tcls-pk")
            .ok_or("Missing tcls public key")?;

        let process_duration = parse_duration(process_duration);
        let tlcs_pbk = parse_tlcs_pbk(tlcs_pbk)?;

        return Ok((
            global_cli_param,
            CliCommand::CreateProcess(process_duration, tlcs_pbk),
        ));
    }

    // Parse the command `vote`
    if let Some(matches) = matches.subcommand_matches("vote") {
        let process_id = matches.get_one("process-id").ok_or("Missing process id")?;
        let nft_id: &String = matches.get_one("nft-id").ok_or("Missing nft id")?;
        let nft_owner_address = matches
            .get_one("nft-owner-address")
            .ok_or("Missing nft owner address")?;
        let nft_owner_prk: &String = matches
            .get_one("nft-owner-pk")
            .ok_or("Missing nft owner public key")?;
        let vote_choice: &String = matches
            .get_one("vote-choice")
            .ok_or("Missing vote choice")?;
        let tlcs_pbk: &String = matches
            .get_one("tcls-pk")
            .ok_or("Missing tcls public key")?;

        let process_id =
            U256::try_from(process_id).map_err(|e| format!("Invalid process id: {:?}", e))?;
        // We allow the user to pass the nft id as a decimal or as a hex string (with or without the 0x prefix)
        let nft_id = parse_u256(nft_id)?;
        let nft_owner_address = Address::from_str(*nft_owner_address)
            .map_err(|e| format!("Invalid nft owner address: {}", e))?;
        let nft_owner_prk =
            parse_bbjj_prk(nft_owner_prk).map_err(|_e| "Invalid nft owner private key")?;
        let vote_choice = VoteChoice::from(vote_choice.as_str());
        let tlcs_pbk = parse_tlcs_pbk(tlcs_pbk)?;

        return Ok((
            global_cli_param,
            CliCommand::Vote(
                process_id,
                nft_id,
                nft_owner_address,
                nft_owner_prk,
                vote_choice,
                tlcs_pbk,
            ),
        ));
    }

    // Parse the command `tally`
    if let Some(matches) = matches.subcommand_matches("tally") {
        let process_id: &String = matches.get_one("process-id").ok_or("Missing process id")?;
        let tcls_prk: &String = matches
            .get_one("tcls-pk")
            .ok_or("Missing tcls public key")?;

        let process_id = parse_u256(process_id)?;
        let tcls_prk = parse_bbjj_prk(tcls_prk).map_err(|_e| "Invalid tcls private key")?;

        return Ok((global_cli_param, CliCommand::Tally(process_id, tcls_prk)));
    }

    // No command was chosen
    Ok((global_cli_param, CliCommand::None))
}

mod parsers {
    use std::time::Duration;

    use ark_ff::PrimeField;
    use babyjubjub_ark::PrivateKey;
    use ethers::core::k256::U256;

    use nouns_anonymous_voting::{BBJJ_Ec, BN254_Fr};

    /// Parses a hex string into BBJJ PrivateKey
    /// Example: `1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef` of 32 bytes
    pub(crate) fn parse_bbjj_prk(key_to_reg: &String) -> Result<PrivateKey, String> {
        PrivateKey::import(
            hex::decode(key_to_reg).map_err(|e| format!("Failed to parse hex string: {}", e))?,
        )
    }

    /// Parses a duration string into a Duration
    /// Example: `1d` (1 day)
    /// Example: `1h` (1 hour)
    /// Example: `1m` (1 minute)
    pub(crate) fn parse_duration<T: Into<String>>(s: T) -> Duration {
        let s = s.into();
        let mut chars = s.chars();
        let mut number = String::new();
        let mut unit = String::new();
        while let Some(c) = chars.next() {
            if c.is_numeric() {
                number.push(c);
            } else {
                unit.push(c);
            }
        }
        let number = number.parse::<u64>().unwrap();
        let duration = match unit.as_str() {
            "d" => Duration::from_secs(number * 24 * 60 * 60),
            "h" => Duration::from_secs(number * 60 * 60),
            "m" => Duration::from_secs(number * 60),
            _ => panic!("Invalid duration unit"),
        };
        duration
    }

    /// Parses a be TLCS Public Key string into a BBJJ_Ec
    pub(crate) fn parse_tlcs_pbk<T: Into<String>>(s: T) -> Result<BBJJ_Ec, String> {
        let s = s.into();
        let mut chars = s.chars();
        let mut x = String::new();
        let mut y = String::new();
        while let Some(c) = chars.next() {
            if c == ',' {
                break;
            }
            x.push(c);
        }
        while let Some(c) = chars.next() {
            y.push(c);
        }

        let x = BN254_Fr::from_be_bytes_mod_order(
            &hex::decode(x).map_err(|_| "Invalid TLCS Public Key X coordinate")?,
        );
        let y = BN254_Fr::from_be_bytes_mod_order(
            &hex::decode(y).map_err(|_| "Invalid TLCS Public Key Y coordinate")?,
        );

        Ok(BBJJ_Ec { x, y })
    }

    /// Parses a U256
    /// Supports both decimal (for small numbers) and hex (for large numbers) representations
    /// Decimal example: `123456789`, the size of the number is limited to 8 bytes
    /// Hex example: `0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef` (be 32 bytes)
    pub(crate) fn parse_u256<T: Into<String>>(s: T) -> Result<U256, String> {
        let s = s.into();

        // Check if the string starts with 0x
        return if s[0..2] == "0x".as_ref() {
            // If it does, we parse it as a hex string
            let s = &s[2..];
            Ok(U256::from_be_hex(s))
        } else {
            // If it doesn't, we parse it as a decimal string
            let number = s.parse::<u64>().map_err(|_| "Invalid decimal string")?;
            Ok(U256::from(number))
        };
    }
}

/// Constructs the CLI
fn command_constructor() -> Command {
    command!()
        .arg(
            Arg::new("contract-address")
                .short('c')
                .long("contract-address")
                .help("The EVM Address of the NounsVoting contract")
                .help("Example: `0x1234567890123456789012345678901234567890`")
                .required(true)
                .env("VOTING_ADDRESS"),
        )
        .arg(
            Arg::new("rpc-url")
                .short('r')
                .long("rpc-url")
                .help("The RPC URL of the target EVM blockchain")
                .help("Example: `https://rpc2.sepolia.org`")
                .required(true)
                .env("RPC_URL"),
        )
        .arg(
            Arg::new("tx-private-key")
                .short('k')
                .long("private-key")
                .help("The Private Key of the account that will be used to send the transactions")
                .help("Example: `1234567890123456789012345678901234567890123456789012345678901234`")
                .required(true)
                .env("TX_PRIVATE_KEY"),
        )
        .subcommand(
            Command::new("reg-key")
                .about("Registers a new BBJJ Public Key in the ZKRegistry contract")
                .arg(
                    Arg::new("reg-private-key")
                        .short('k')
                        .long("private-key")
                        .help("The Private Key to register in the zkRegistry under the account, that owns the NFT")
                        .help("Example: `0x043c3780cb30f913d1c34d80437f7c61c973461595986e899ee6a8171143db1d`")
                        .required(true)
                        .env("REG_PRIVATE_KEY")
                )
        )
        .subcommand(
            Command::new("create-process")
                .about("Creates a new voting process in the NounsVoting contract")
                .arg(
                    Arg::new("process-duration")
                        .short('d')
                        .long("process-duration")
                        .help("Process Duration in m(minutes)/h(hours)/d(days)")
                        .help("Example: `1d` (1 day)")
                        .required(true)
                )
                .arg(
                    Arg::new("tlcs-public-key")
                        .short('t')
                        .long("tlcs-public-key")
                        .help("TLCS Public Key for the process end time used to encrypt the ballots")
                        .help("Example: `0x0882c07dfb863de7cb769152e581f987b01f723d3cf9a00b3801fd3c206b9537, 0x1f3179c62406bf009ae22a0b15d8d5cf156b9d6945c23aabedea2def1d929364`")
                        .required(true)
                ),
        )
        .subcommand(
            Command::new("vote")
                .about("Allows the user to vote in an existing voting process")
                .arg(
                    Arg::new("voting-process-id")
                        .short('p')
                        .long("voting-process-id")
                        .help("The Voting Process ID")
                        .help("Example: `1`")
                        .required(true)
                )
                .arg(
                    Arg::new("nft-id")
                        .short('n')
                        .long("nft-id")
                        .help("The NFT ID of the Nouns to vote for")
                        .help("Example: `1` or `0x0000000000000000000000000000000000000000000000000000000000000001`")
                        .required(true)
                )
                .arg(
                    Arg::new("nft-owner-address")
                        .short('a')
                        .long("nft-owner-address")
                        .help("The Address of the Voter who owns the NFT")
                        .help("Example: `0x1234567890123456789012345678901234567890`")
                        .required(true)
                        .env("VOTING_ADDRESS"),
                )
                .arg(
                    Arg::new("reg-private-key")
                        .short('k')
                        .long("private-key")
                        .help("The Private Key registered in the zkRegistry under the account, that owns the NFT")
                        .help("Example: `0x043c3780cb30f913d1c34d80437f7c61c973461595986e899ee6a8171143db1d`")
                        .required(true)
                        .env("REG_PRIVATE_KEY")
                )
                .arg(
                    Arg::new("vote-choice")
                        .short('v')
                        .long("vote-choice")
                        .help("The Vote Choice as: (Y)es/(N)o/(A)bstain)")
                        .required(true)
                )
                .arg(
                    Arg::new("tlcs-public-key")
                        .short('t')
                        .long("tlcs-public-key")
                        .help("TLCS Public Key for the process end time used to encrypt the ballots")
                        .help("Example: `0x0882c07dfb863de7cb769152e581f987b01f723d3cf9a00b3801fd3c206b9537, 0x1f3179c62406bf009ae22a0b15d8d5cf156b9d6945c23aabedea2def1d929364`")
                        .required(true)
                ),
        )
        .subcommand(
            Command::new("tally")
                .about("Generates the tally for an existing voting process")
                .arg(
                    Arg::new("voting-process-id")
                        .short('p')
                        .long("voting-process-id")
                        .help("The Voting Process ID it is generating the results for")
                        .help("Example: `1`")
                        .required(true)
                )
                .arg(
                    Arg::new("tlcs-private-key")
                        .short('t')
                        .long("tlcs-private-key")
                        .help("The TLCS Private Key corresponding to the TLCS Public Key used to encrypt the Ballots")
                        .required(true)
                ),
        )
}
