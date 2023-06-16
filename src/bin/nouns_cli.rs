use clap::{App, Arg};

/// The CLI that to interact with the Nouns Anonymous Voting System
/// As global parameters, it should take:
/// 1. The EVM Address of the NounsVoting contract (could be also passed as an environmental variable)
/// 2. The RPC URL of the target EVM blockchain (could be also passed as an environmental variable)
/// 3. The Private Key of the account that will be used to send the transactions (should be passed as an environmental variable or as a password input)
///
/// The CLI will have 3 commands, which the user will choose from:
/// 1. `create-process`
/// 2. `vote`
/// 3. `tally`
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
/// 3. The Private Key of the account that owns the NFT
/// 4. The Vote Choice (Yes/No/Abstain)
/// 5. TLCS Public Key for the process end time used to encrypt the ballots
/// 
/// The `tally` command allows the user to generate the tally for an existing voting process.
/// It then submits the result to the NounsVoting contract with the proof of the tally.
/// It should ask the user for the following additional information:
/// 1. The Voting Process ID it is generating the results for
/// 2. The TLCS Private Key corresponding to the TLCS Public Key used to encrypt the Ballots
/// 
fn main() {
    
}
