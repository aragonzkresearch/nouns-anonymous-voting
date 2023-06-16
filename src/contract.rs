// TODO: Delegation
use ethers::prelude::*;
use crate::voter::Voter;
use std::collections::HashMap;

// Enumeration of relevant smart contracts including addresses
#[derive(Eq, Hash, PartialEq)]
enum Contract
{
    ZKRegistry,
    Voting
}

// Relevant contract parameters
struct ContractParam
{
    address: HashMap<Contract, Address>,
    block_hash: H256 // Block hash at cut-off
}

impl ContractParam
{
    fn new(registry_address: String, voting_address: String, block_hash: String) -> ContractParam
    {
        
    }
}

impl Voter
{
    pub async fn fetch_storage_proof<T: JsonRpcClient>(&self, block_hash: H256, contract: Contract, contract_params: ContractParam, provider: Provider<T>) -> Option<StorageProof>
    {
        let address = contract_params.address.get(&contract)?;
        let storage_location = match contract
        {
            Contract::ZKRegistry => {
                // Logic for determining storage location goes here
                let location: H256 = "0x00".parse().unwrap();
                location
            },
            Contract::Voting => {
                // Logic for determining storage location goes here
                let location: H256 = "0x00".parse().unwrap();
                location
            }
            
        };
        let proof = provider.get_proof(*address, vec![storage_location], Some(BlockId::from(block_hash))).await.unwrap();
        proof.storage_proof.get(0).cloned()
    }

    // zkRegistry registration
    async fn register<T: JsonRpcClient>(&self, registry_address: Address, provider: Provider<T>) -> Result<(), String>
    {
        todo!()
    }

}
