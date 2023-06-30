use std::io;

use ethers::prelude::Middleware;

use nouns_cli::{mine_blocks_until, setup_connection, setup_env_parameters};

#[tokio::main]
async fn main() {
    //// Set the simulation parameters

    let (tx_private_key, rpc_url, _) = setup_env_parameters();

    let (eth_connection, wallet_address, client, _) =
        setup_connection(tx_private_key, rpc_url).await;

    let current_block = eth_connection
        .get_block_number()
        .await
        .expect("Error getting current block number")
        .as_u64();

    // Ask user how many blocks to mine
    let mut input = String::new();
    println!("How many blocks do you want to mine?");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let blocks_to_mine: u64 = input.trim().parse().expect("Please type a number!");

    mine_blocks_until(
        eth_connection,
        wallet_address,
        &client,
        current_block + blocks_to_mine,
    )
    .await
    .unwrap();
}
