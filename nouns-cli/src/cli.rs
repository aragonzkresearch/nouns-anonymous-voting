use std::str::FromStr;
use std::time::Duration;

use clap::{command, Arg, Command};
use ethers::abi::Address;
use ethers::core::k256::U256;

use nouns_protocol::{BBJJ_Ec, PrivateKey, VoteChoice};

use crate::parsers::{
    parse_bbjj_prk, parse_duration, parse_private_key, parse_tlcs_pbk, parse_u256,
};

/// The global parameters of the CLI
pub(crate) struct GlobalCliParams {
    pub(crate) contract_address: Address,
    pub(crate) rpc_url: String,
    pub(crate) tx_private_key: [u8; 32],
}

/// The CLI commands that the user can choose from
/// Parameters are passed as arguments and the command is executed
pub(crate) enum CliCommand {
    RegKey(PrivateKey),
    CreateProcess(Duration, BBJJ_Ec),
    Vote(U256, U256, Address, PrivateKey, VoteChoice, BBJJ_Ec),
    Tally(U256, PrivateKey),
    None, // No command was chosen
}

pub(crate) fn get_user_input() -> Result<(GlobalCliParams, CliCommand), String> {
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
