use ethers::abi::Address;
use ethers::prelude::{
    BigEndianHash, BlockId, Http, Middleware, Provider, StorageProof, H256, U64,
};
use ethers::utils::keccak256;

use nouns_protocol::noir;
use nouns_protocol::MAX_NODE_LEN;
use nouns_protocol::{BlockHeader, StateProof};

use ethers::types::{Block, Bytes};
use ethers::utils::rlp;

use crate::EthersU256;

pub(crate) const REGISTRY_SLOT_OFFSET: u64 = 0;
pub(crate) const NFT_OWNER_SLOT_OFFSET: u64 = 3;
pub(crate) const DELEGATE_SLOT_OFFSET: u64 = 0x0b;

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
    if let Err(err) = validate_proof(&nft_account_proof.storage_proof[0].proof) {
        return Err(format!("Invalid NFT Account proof: {}", err));
    }

    let nft_account_state_hash = nft_account_proof.storage_hash.into_uint();
    let nft_account_state_proof = nft_account_proof
        .storage_proof
        .get(0)
        .ok_or("Error getting NFT account state proof")?;

    Ok((nft_account_state_hash, nft_account_state_proof.clone()))
}

pub(crate) async fn get_delegation_proof(
    eth_connection: Provider<Http>,
    address: Address,
    start_block_number: U64,
    nouns_token_address: Address,
) -> Result<(EthersU256, StorageProof), String> {
    let delegation_proof = eth_connection
        .get_proof(
            nouns_token_address,
            vec![map_storage_slot(
                H256::from_uint(&DELEGATE_SLOT_OFFSET.into()),
                vec![H256::from(address)],
            )],
            Some(BlockId::from(start_block_number.as_u64())),
        )
        .await
        .map_err(|e| format!("Error getting delegation proof: {}", e))?;

    // Validate the proof
    if let Err(err) = validate_proof(&delegation_proof.storage_proof[0].proof) {
        return Err(format!("Invalid delegation proof: {}", err));
    }

    let nft_contract_storage_hash = delegation_proof.storage_hash.into_uint();
    let delegation_storage_proof = delegation_proof
        .storage_proof
        .get(0)
        .ok_or("Error getting delegation proof")?;

    Ok((nft_contract_storage_hash, delegation_storage_proof.clone()))
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
    if let Err(err) = validate_proof(&zk_registry_proof.storage_proof[0].proof) {
        return Err(format!("Invalid ZKRegistry proof: {}", err));
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

/// This function validates the proof returned by the Ethereum node in the following sense:
/// - It checks that the proof depth is not too large.
/// - It checks that the nodes' sizes do not exceed their upper bound.
/// - It returns an error if the proof is invalid.
fn validate_proof(proof: &Vec<Bytes>) -> Result<Option<()>, String> {
    // Check that the length of the proof is not too long
    if proof.len() > noir::MAX_DEPTH {
        return Err(format!("Proof is too long: {}", proof.len()));
    }

    // Make sure path is valid
    for node in proof.iter() {
        if node.len() > MAX_NODE_LEN {
            return Err(format!("Node is too big, thus invalid."));
        }
    }

    Ok(None)
}

pub(crate) async fn get_state_proof(
    eth_connection: &Provider<Http>,
    block_number: U64,
    address: Address,
) -> Result<StateProof, String> {
    // Call eth_getProof
    let proof_data = eth_connection
        .get_proof(address, vec![], Some(block_number.into()))
        .await
        .expect("Error getting state proof");

    // Form proof in the form of a path
    let proof = proof_data.account_proof;

    // Validate proof
    if let Err(err) = validate_proof(&proof) {
        return Err(format!(
            "Invalid state proof for address {}: {}",
            address, err
        ));
    }

    // Extract value in RLP form
    // TODO: Integrity check
    let value = rlp::Rlp::new(
        proof
            .last() // Terminal proof node
            .expect("Error: State proof empty"),
    ) // Proof should have been non-empty
    .as_list::<Vec<u8>>()
    .expect("Error: Invalid RLP encoding")
    .last() // Extract value
    .expect("Error: RLP list empty")
    .clone();

    Ok(StateProof {
        key: address,
        proof,
        value,
    })
}

pub(crate) fn header_from_block(block: &Block<H256>) -> Result<BlockHeader, String> {
    let fork_headsup = "Error: Should be on Shanghai fork.";

    let mut block_header = rlp::RlpStream::new_list(17);

    block_header.append(&block.parent_hash);
    block_header.append(&block.uncles_hash);
    block_header.append(&block.author.unwrap());
    block_header.append(&block.state_root);
    block_header.append(&block.transactions_root);
    block_header.append(&block.receipts_root);
    block_header.append(&block.logs_bloom.unwrap());
    block_header.append(&block.difficulty);
    block_header.append(&block.number.unwrap());
    block_header.append(&block.gas_limit);
    block_header.append(&block.gas_used);
    block_header.append(&block.timestamp);
    block_header.append(&block.extra_data.as_ref()); // ...
    block_header.append(&block.mix_hash.unwrap());
    block_header.append(&block.nonce.unwrap());
    block_header.append(&block.base_fee_per_gas.expect(fork_headsup));
    block_header.append(&block.withdrawals_root.expect(fork_headsup));

    Ok(BlockHeader(block_header.out().into()))
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
    use crate::ethereum::proofs::map_storage_slot;

    #[tokio::test]
    async fn test_nft_ownership_proof() -> Result<(), String> {
        let nouns_address =
            Address::from_str("0x9C8fF314C9Bc7F6e59A9d9225Fb22946427eDC03").unwrap();
        let ethereum_rpc = "https://mainnet.infura.io/v3/977a6ab6091b4a1a855f51a89cc64528";

        let eth_connection = Provider::<Http>::connect(ethereum_rpc).await;

        let nft_id = U256::from(0);
        let _expected_address =
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
