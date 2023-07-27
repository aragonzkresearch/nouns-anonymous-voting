use std::str::FromStr;
use std::time::Duration;

use clap::{command, Arg, Command};
use ethers::abi::Address;
use ethers::core::k256::U256;
use ethers::types::H256;

use nouns_protocol::{BBJJ_Ec, BBJJ_Fr, PrivateKey, VoteChoice};

use crate::parsers::{
    parse_bbjj_prk, parse_duration, parse_ipfs_hash, parse_private_key, parse_tlcs_pbk, parse_u256,
};

/// The global parameters of the CLI
pub struct GlobalCliParams {
    pub contract_address: Address,
    pub rpc_url: String,
    pub tx_private_key: [u8; 32],
}

/// The CLI commands that the user can choose from
/// Parameters are passed as arguments and the command is executed
pub enum CliCommand {
    RegKey(PrivateKey),
    CreateProcess(H256, Duration, Duration),
    Vote(Address, U256, U256, PrivateKey, VoteChoice),
    Tally(U256),
    None, // No command was chosen
}

pub fn get_user_input() -> Result<(GlobalCliParams, CliCommand), String> {
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

        let tx_private_key = parse_private_key(tx_private_key)?;

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

        let key_to_reg = PrivateKey::import(
            parse_private_key(key_to_reg)
                .map_err(|e| format!("Invalid key to register in ZKRegistry: {}", e))?
                .to_vec(),
        )
        .map_err(|e| format!("Invalid key to register in ZKRegistry: {}", e))?;

        return Ok((global_cli_param, CliCommand::RegKey(key_to_reg)));
    }

    // Parse the command `create-process`
    if let Some(matches) = matches.subcommand_matches("create-process") {
        let start_delay: &String = matches
            .get_one("start-delay")
            .ok_or("Missing start delay")?;
        let process_duration: &String = matches
            .get_one("process-duration")
            .ok_or("Missing process duration")?;
        let ipfs_hash: &String = matches
            .get_one("ipfs-hash")
            .ok_or("Missing IPFS hash")?;

        let start_delay = parse_duration(start_delay);
        let process_duration = parse_duration(process_duration);
        let ipfs_hash = parse_ipfs_hash(ipfs_hash)?;

        return Ok((
            global_cli_param,
            CliCommand::CreateProcess(ipfs_hash, start_delay, process_duration),
        ));
    }

    // Parse the command `vote`
    if let Some(matches) = matches.subcommand_matches("vote") {
        let voter_address: &String = matches
            .get_one("voter-address")
            .ok_or("Missing voter's address")?;
        let process_id: &String = matches
            .get_one("voting-process-id")
            .ok_or("Missing process id")?;
        let nft_id: &String = matches.get_one("nft-id").ok_or("Missing NFT id")?;
        let nft_owner_prk: &String = matches
            .get_one("reg-private-key")
            .ok_or("Missing nft owner private registry key")?;
        let vote_choice: &String = matches
            .get_one("vote-choice")
            .ok_or("Missing vote choice")?;

        let voter_address = Address::from_str(voter_address).map_err(|e| format!("Invalid voter address: {}", e))?;
        let process_id = U256::from_u64(
            u64::from_str(process_id.as_ref())
                .map_err(|e| format!("Invalid process id: {:?}", e))?,
        );
        // We allow the user to pass the nft id as a decimal or as a hex string (with or without the 0x prefix)
        let nft_id = parse_u256(nft_id)?;
        let nft_owner_prk = PrivateKey::import(
            parse_private_key(nft_owner_prk)
                .map_err(|e| format!("Invalid nft owner private key: {}", e))?
                .to_vec(),
        )
        .map_err(|e| format!("Invalid nft owner private key: {}", e))?;

        let vote_choice = VoteChoice::from(vote_choice.as_str());

        return Ok((
            global_cli_param,
            CliCommand::Vote(voter_address, process_id, nft_id, nft_owner_prk, vote_choice),
        ));
    }

    // Parse the command `tally`
    if let Some(matches) = matches.subcommand_matches("tally") {
        let process_id: &String = matches
            .get_one("voting-process-id")
            .ok_or("Missing process id")?;

        let process_id = parse_u256(process_id)?;

        return Ok((global_cli_param, CliCommand::Tally(process_id)));
    }

    // No command was chosen
    Ok((global_cli_param, CliCommand::None))
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
                        .help("Example: `043c3780cb30f913d1c34d80437f7c61c973461595986e899ee6a8171143db1d`")
                        .required(true)
                        .env("REG_PRIVATE_KEY")
                )
        )
        .subcommand(
            Command::new("create-process")
                .about("Creates a new voting process in the NounsVoting contract")
                .arg(
                    Arg::new("start-delay")
                        .short('s')
                        .long("start-delay")
                        .help("Optional delay period in m(minutes)/h(hours)/d(days)")
                        .help("Example: `1d` (1 day)")
                        .help("Example: `10h` (10 hours)")
                        .help("Example: `100m` (1 minutes)")
                        .default_value("0m")
                )
                .arg(
                    Arg::new("process-duration")
                        .short('d')
                        .long("process-duration")
                        .help("Process Duration in m(minutes)/h(hours)/d(days)")
                        .help("Example: `1d` (1 day)")
                        .help("Example: `10h` (10 hours)")
                        .help("Example: `100m` (1 minutes)")
                        .required(true)
                )
                .arg(
                    Arg::new("ipfs-hash")
                        .short('i')
                        .long("ipfs-hash")
                        .help("IPFS CIDv1 hash (raw binary codec using sha2-256 hash) associated with voting process")
                        .help("Example: bafkreidfgllkxpigujgbavuq5kxdd5yo2jid3abzuxhwj7l6socllnd3m4")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("vote")
                .about("Allows the user to vote in an existing voting process")
                .arg(
                    Arg::new("voter-address")
                        .short('a')
                        .long("voter-address")
                        .help("The address of the voter who must be in the zkRegistry. This should *not* coincide with the address of the wallet used to carry out this transaction!")
                        .help("Example: `0xa8b2e7f501928374169283f7b2a5d3f9e0a7b3d6`")
                        .required(true)
                )
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
        )
}
