use std::sync::Arc;

use nouns_cli::{obtain_token_ids_to_vote, setup_connection, setup_env_parameters, NounsVoting};

#[tokio::main]
async fn main() {
    //// Set the simulation parameters

    let (tx_private_key, rpc_url, voting_address) = setup_env_parameters();

    //// Configure the System Parameters

    let (_, wallet_address, client, _) = setup_connection(tx_private_key, rpc_url).await;

    let nouns_voting = NounsVoting::new(voting_address, Arc::new(client.clone()));

    //// Obtain the TokenIds owned by the wallet (possibly minting some)

    let token_ids = obtain_token_ids_to_vote(wallet_address, nouns_voting.clone(), client.clone())
        .await
        .map_err(|e| format!("Error obtaining token ids: {}", e))
        .expect("Error obtaining token ids");

    println!("Available TokenIds: {:?}", token_ids);
}
