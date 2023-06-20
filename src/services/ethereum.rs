use std::collections::HashMap;

use ethers::prelude::*;

// TODO: Delegation
use crate::voter::Voter;

mod nouns_nft;
mod nouns_voting;
mod zk_registry;

// Enumeration of relevant smart contracts including addresses
#[derive(Eq, Hash, PartialEq)]
pub enum Contract {
    ZKRegistry,
    Voting,
}

// Relevant contract parameters
pub struct ContractParam {
    address: HashMap<Contract, Address>,
    block_hash: H256, // Block hash at cut-off
}

impl ContractParam {
    fn new(registry_address: String, voting_address: String, block_hash: String) -> ContractParam {
        unimplemented!()
    }
}

impl Voter {
    pub async fn fetch_storage_proof<T: JsonRpcClient>(
        &self,
        block_hash: H256,
        contract: Contract,
        contract_params: ContractParam,
        provider: Provider<T>,
    ) -> Option<StorageProof> {
        let address = contract_params.address.get(&contract)?;
        let storage_location = match contract {
            Contract::ZKRegistry => {
                // Logic for determining storage location goes here
                let location: H256 = "0x00".parse().unwrap();
                location
            }
            Contract::Voting => {
                // Logic for determining storage location goes here
                let location: H256 = "0x00".parse().unwrap();
                location
            }
        };
        let proof = provider
            .get_proof(
                *address,
                vec![storage_location],
                Some(BlockId::from(block_hash)),
            )
            .await
            .unwrap();
        proof.storage_proof.get(0).cloned()
    }

    // zkRegistry registration
    async fn register<T: JsonRpcClient>(
        &self,
        registry_address: Address,
        provider: Provider<T>,
    ) -> Result<(), String> {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
/// Ethereum state proof for a single storage slot
pub(crate) struct StateProof {}

struct BallotWithProof {}
