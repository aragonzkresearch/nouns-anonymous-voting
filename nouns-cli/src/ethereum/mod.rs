use std::str::FromStr;

use ethers::abi::Address;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::{
    Http, LocalWallet, Middleware, Provider, ProviderExt, Signer, SignerMiddleware, Wallet, H160,
    U256,
};

pub mod contract_interactions;
pub(crate) mod proofs;

pub async fn setup_connection(
    tx_private_key: String,
    rpc_url: String,
) -> (
    Provider<Http>,
    Address,
    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    U256,
) {
    let eth_connection = Provider::<Http>::connect(rpc_url.as_str()).await;
    // create the signer for the txs

    let wallet = LocalWallet::from_str(tx_private_key.as_str()).unwrap();
    let wallet_address = wallet.address();

    let chain_id = eth_connection
        .get_chainid()
        .await
        .map_err(|_e| "Could not get chain id".to_string())
        .unwrap();

    let client = SignerMiddleware::new(
        eth_connection.clone(),
        wallet.with_chain_id(chain_id.as_u64()),
    );
    (eth_connection, wallet_address, client, chain_id)
}

pub fn setup_env_parameters() -> (String, String, H160) {
    // Load the environment variables from the .env file
    dotenv::dotenv().ok();

    // Get from the environment the TX_PRIVATE_KEY value
    let tx_private_key = std::env::var("TX_PRIVATE_KEY").expect("TX_PRIVATE_KEY not set");

    // Get from the environment the RPC_URL value
    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL not set");

    // Get from the environment the VOTING_ADDRESS value
    let voting_address = Address::from_str(
        std::env::var("VOTING_ADDRESS")
            .expect("VOTING_ADDRESS not set")
            .as_str(),
    )
    .expect("Error parsing VOTING_ADDRESS");
    (tx_private_key, rpc_url, voting_address)
}
