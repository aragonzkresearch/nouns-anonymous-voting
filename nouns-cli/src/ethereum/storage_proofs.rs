use ethers::abi::Address;
use ethers::prelude::{BigEndianHash, BlockId, Http, Middleware, Provider, StorageProof, H256};
use ethers::utils::keccak256;

use crate::EthersU256;

// Useful constants for storage proofs
pub(crate) const MAX_NODE_LEN: usize = 532;
// The maximum byte length of a node
pub(crate) const MAX_DEPTH: usize = 8; // For technical reasons, we need a fixed maximum trie proof size.

pub(crate) const REGISTRY_SLOT_OFFSET: u64 = 0;
pub(crate) const NFT_OWNER_SLOT_OFFSET: u64 = 1; // TODO - find out the correct value

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
    nft_owner: Address,
    start_block_number: EthersU256,
    nouns_token_address: Address,
) -> Result<(EthersU256, StorageProof), String> {
    let nft_account_proof = eth_connection
        .get_proof(
            nouns_token_address,
            vec![map_storage_slot(
                H256::from_uint(&NFT_OWNER_SLOT_OFFSET.into()),
                vec![H256::from(nft_owner), H256::from_uint(&nft_id)],
            )],
            Some(BlockId::from(start_block_number.as_u64())),
        )
        .await
        .map_err(|e| format!("Error getting NFT account proof: {}", e))?;

    // Check that the length of the proof is not too long
    if nft_account_proof.storage_proof[0].proof.len() > MAX_NODE_LEN {
        return Err(format!(
            "NFT account proof is too long: {}",
            nft_account_proof.storage_proof[0].proof.len()
        ));
    }

    let nft_account_state_hash = nft_account_proof.storage_hash.into_uint();
    let nft_account_state_proof = nft_account_proof
        .storage_proof
        .get(0)
        .ok_or("Error getting NFT account state proof")?;
    Ok((nft_account_state_hash, nft_account_state_proof.clone()))
}

pub(crate) async fn get_zk_registry_proof(
    eth_connection: &Provider<Http>,
    nft_owner: Address,
    start_block_number: EthersU256,
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

    // Check that the length of the proof is not too long
    if zk_registry_proof.storage_proof[0].proof.len() > MAX_DEPTH {
        return Err(format!(
            "ZKRegistry proof is too long: {}",
            zk_registry_proof.storage_proof[0].proof.len()
        ));
    }

    let registry_account_state_hash = zk_registry_proof.storage_hash.into_uint();
    let registry_account_state_proof = zk_registry_proof
        .storage_proof
        .get(0) // TODO: we currently only provide storage proof of the X coordinate, however we need to provide both for protocol soundness
        .ok_or("Error getting ZKRegistry state proof")?;
    Ok((
        registry_account_state_hash,
        registry_account_state_proof.clone(),
    ))
}
