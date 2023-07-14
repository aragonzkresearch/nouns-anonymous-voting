extern crate core;

use ethers::middleware::SignerMiddleware;
use ethers::prelude::Signer;
use ethers::providers::{Http, Middleware, Provider, ProviderExt};
use ethers::signers::LocalWallet;

use nouns_cli::cli::{get_user_input, CliCommand};
use nouns_cli::ethereum::contract_interactions::{create_process, reg_key, tally, vote};
use nouns_protocol::{wrap, wrap_into, Wrapper};

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
                eth_connection,
                global_param.contract_address,
                process_duration,
                tlcs_pbk,
            )
            .await
        }
        CliCommand::Vote(process_id, nft_id, bbjj_private_key, vote_choice, tlcs_pbk) => {
            vote(
                client,
                eth_connection,
                global_param.contract_address,
                process_id,
                nft_id,
                wrap_into!(chain_id),
                bbjj_private_key,
                vote_choice,
                tlcs_pbk,
            )
            .await
        }
        CliCommand::Tally(process_id, tlcs_prk) => {
            tally(
                client,
                global_param.contract_address,
                wrap_into!(chain_id),
                process_id,
                tlcs_prk,
            )
            .await
        }
        _ => {
            eprintln!("Error: Command not implemented yet");
            std::process::exit(1);
        }
    }
    .unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    println!("Done!");
}
