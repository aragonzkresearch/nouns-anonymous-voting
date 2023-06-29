use ethers::abi::Address;
use ethers::prelude::{
    BigEndianHash, BlockId, Http, Middleware, Provider, StorageProof, H256, U64,
};
use ethers::utils::keccak256;

use nouns_protocol::noir::MAX_DEPTH;
use nouns_protocol::MAX_NODE_LEN;

use crate::EthersU256;

pub(crate) const REGISTRY_SLOT_OFFSET: u64 = 0;
pub(crate) const NFT_OWNER_SLOT_OFFSET: u64 = 3;

pub(crate) const BBJJ_INTERFACE_X_ID: u8 = 0;
pub(crate) const BBJJ_INTERFACE_Y_ID: u8 = 1;

/// This function calculates the storage location of the Ethereum multidimensional mapping
/// map_keys is a vector of mapping indices, starting from the highest
/// For instance, `a[0][1][2]` would be represented by `map_keys = vec![0, 1, 2]`
fn map_storage_slot(slot_number: H256, map_keys: Vec<H256>) -> H256 {
    let mut location = slot_number;
    for key in map_keys {
        location = keccak256(&[key.as_bytes(), location.as_bytes()].concat()).into()
    }

    return location;
}

pub(crate) async fn get_nft_ownership_proof(
    eth_connection: Provider<Http>,
    nft_id: EthersU256,
    start_block_number: U64,
    nouns_token_address: Address,
) -> Result<(EthersU256, StorageProof), String> {
    let nft_account_proof = eth_connection
        .get_proof(
            nouns_token_address,
            vec![map_storage_slot(
                H256::from_uint(&NFT_OWNER_SLOT_OFFSET.into()),
                vec![H256::from_uint(&nft_id)],
            )],
            Some(BlockId::from(start_block_number.as_u64())),
        )
        .await
        .map_err(|e| format!("Error getting NFT account proof: {}", e))?;

    // Validate the proof
    if let Err(err) = validate_storage_proof(&nft_account_proof.storage_proof[0]) {
        return Err(format!("Invalid NFT Account proof: {}", err));
    }

    let nft_account_state_hash = nft_account_proof.storage_hash.into_uint();
    let nft_account_state_proof = nft_account_proof
        .storage_proof
        .get(0)
        .ok_or("Error getting NFT account state proof")?;

    println!("NFT storage value: {:?}", nft_account_state_proof.value);

    Ok((nft_account_state_hash, nft_account_state_proof.clone()))
}

pub(crate) async fn get_zk_registry_proof(
    eth_connection: &Provider<Http>,
    nft_owner: Address,
    start_block_number: U64,
    zk_registry_address: Address,
) -> Result<(EthersU256, StorageProof), String> {
    let zk_registry_proof = eth_connection
        .get_proof(
            zk_registry_address,
            vec![
                map_storage_slot(
                    H256::from_uint(&REGISTRY_SLOT_OFFSET.into()),
                    vec![
                        H256::from_uint(&BBJJ_INTERFACE_X_ID.into()),
                        H256::from(nft_owner),
                    ],
                ),
                map_storage_slot(
                    H256::from_uint(&REGISTRY_SLOT_OFFSET.into()),
                    vec![
                        H256::from_uint(&BBJJ_INTERFACE_Y_ID.into()),
                        H256::from(nft_owner),
                    ],
                ),
            ],
            Some(BlockId::from(start_block_number.as_u64())),
        )
        .await
        .map_err(|_| format!("Error getting ZKRegistry proof"))?;

    // Validate the proof
    if let Err(err) = validate_storage_proof(&zk_registry_proof.storage_proof[0]) {
        return Err(format!("Invalid ZKRegistry proof: {}", err));
    }

    let registry_account_state_hash = zk_registry_proof.storage_hash.into_uint();
    let registry_account_state_proof = zk_registry_proof
        .storage_proof
        .get(0) // TODO: we currently only provide storage proof of the X coordinate, however we need to provide both for protocol soundness
        .ok_or("Error getting ZKRegistry state proof")?;

    println!(
        "zk registry storage value: {:?}",
        registry_account_state_proof.value
    );

    Ok((
        registry_account_state_hash,
        registry_account_state_proof.clone(),
    ))
}

/// This function validates the storage proof returned by the Ethereum node
/// It checks that the proof is not too long and that the nodes are not too long for the circuit
/// It returns an error if the proof is invalid
fn validate_storage_proof(proof: &StorageProof) -> Result<Option<()>, String> {
    // Check that the length of the proof is not too long
    if proof.proof.len() > MAX_DEPTH {
        return Err(format!("Proof is too long: {}", proof.proof.len()));
    }

    // Make sure path is valid
    for node in proof.proof.iter() {
        if node.len() > MAX_NODE_LEN {
            return Err(format!("Invalid node!"));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::sync::Arc;

    use ethers::abi::AbiEncode;
    use ethers::prelude::{
        Address, BigEndianHash, BlockId, Http, Middleware, Provider, ProviderExt, H256, U256,
    };

    use crate::ethereum::contract_interactions::NounsToken;
    use crate::ethereum::storage_proofs::map_storage_slot;

    #[tokio::test]
    async fn test_nft_ownership_proof() -> Result<(), String> {
        let nouns_address =
            Address::from_str("0x9C8fF314C9Bc7F6e59A9d9225Fb22946427eDC03").unwrap();
        let ethereum_rpc = "https://mainnet.infura.io/v3/977a6ab6091b4a1a855f51a89cc64528";

        let eth_connection = Provider::<Http>::connect(ethereum_rpc).await;

        let nft_id = U256::from(0);
        let expected_address =
            Address::from_str("0x2573C60a6D127755aA2DC85e342F7da2378a0Cc5").unwrap();

        let mut offset = 0u64;

        let block_number = eth_connection.get_block_number().await.unwrap();

        let client = Arc::new(eth_connection.clone());

        let nouns_token = NounsToken::new(nouns_address, client.clone());
        let owner = nouns_token.owner_of(nft_id).await.unwrap();
        let owner = U256::from_big_endian(&owner.as_bytes());

        while offset < 1000000 {
            let nft_account_proof = eth_connection
                .get_proof(
                    nouns_address,
                    vec![map_storage_slot(
                        H256::from_uint(&offset.into()),
                        vec![H256::from_uint(&nft_id)],
                    )],
                    Some(BlockId::from(block_number)),
                )
                .await
                .map_err(|e| format!("Error getting NFT account proof: {}", e))?;

            if nft_account_proof.storage_proof[0].value == owner {
                println!("Correct Offset: {}", offset);
                println!(
                    "Got value {}",
                    nft_account_proof.storage_proof[0].value.encode_hex()
                );
                break;
            } else {
                println!("Offset {} failed, moving to next value", offset);
                println!("Got value {}", nft_account_proof.storage_proof[0].value);
            }

            offset += 1;
        }

        return Ok(());
    }
}
