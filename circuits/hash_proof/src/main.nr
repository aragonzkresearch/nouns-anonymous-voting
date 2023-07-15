use dep::trie::byte_value;
use dep::trie::rlp;
use dep::trie::TrieProof;
use dep::std::hash::keccak256;

global MAX_BLOCK_HEADER_SIZE: Field = 630; // Maximum size in bytes. Liberal estimate.
global MAX_PROOF_SIZE: Field = 4256; // Maximum proof size in bytes for a depth 8 state proof
global MAX_ACCOUNT_STATE_SIZE: Field = 134; // Including RLP headers

fn main(
    block_hash: pub [Field; 2],
    block_header: [u8; MAX_BLOCK_HEADER_SIZE],
    state_root: pub [Field; 2],
    registry_address: pub Field,
    registry_state_proof: TrieProof<20, MAX_PROOF_SIZE, MAX_ACCOUNT_STATE_SIZE>,
    registry_storage_root: pub [Field; 2],
    nft_contract_address: pub Field,
    nft_state_proof: TrieProof<20, MAX_PROOF_SIZE, MAX_ACCOUNT_STATE_SIZE>,
    nft_storage_root: pub [Field; 2]
)
{

    // Convert hashes to bytes
    let block_hash_bytes = u256_from_fields(block_hash);
    let state_root_bytes = u256_from_fields(state_root);
    let registry_storage_root_bytes = u256_from_fields(registry_storage_root);
    let nft_storage_root_bytes = u256_from_fields(nft_storage_root);
    
    // Check that state root, block header and block hash are consistent
    // TODO: Uncomment
    // assert(verify_block_hash(block_hash_bytes, block_header, state_root_bytes));

    // Check that zkRegistry contract account corresponds to the given address is reachable from state root and has the given storage root
    let registry_address_bytes = registry_address.to_be_bytes(20);
    for i in 0..20
    {
        assert(registry_state_proof.key[i] == registry_address_bytes[i]);
    }
    assert(registry_state_proof.verify_state_root(state_root_bytes));
    let registry_account_state = byte_value(registry_state_proof.value).0;
    assert(verify_account_storage(registry_account_state, registry_storage_root_bytes));


    // Check that NFT contract account is reachable from state root and has the given storage root
    let nft_contract_address_bytes = nft_contract_address.to_be_bytes(20);
    for i in 0..20
    {
        assert(nft_state_proof.key[i] == nft_contract_address_bytes[i]);
    }
    
    assert(nft_state_proof.verify_state_root(state_root_bytes));
    let nft_account_state = byte_value(nft_state_proof.value).0;
    assert(verify_account_storage(nft_account_state, nft_storage_root_bytes));
}

// Block hash verification assumes Shanghai fork
fn verify_block_hash<N>(
    block_hash: [u8; 32],
    block_header: [u8; N],
    state_root: [u8; 32])
    -> bool
{
    // Decode block header
    let rlp_list: rlp::RLP_List<17> = rlp::decode1(block_header);
    
    // Check for 17 slots
    assert(rlp_list.num_fields == 17);

    // Check lengths
    assert(rlp_list.length[0] == 32); // parent_hash
    assert(rlp_list.length[1] == 32); // ommers_hash
    assert(rlp_list.length[2] == 20); // beneficiary
    assert(rlp_list.length[3] == 32); // state_root
    assert(rlp_list.length[4] == 32); // transactions_root
    assert(rlp_list.length[5] == 32); // receipts_root
    assert(rlp_list.length[6] == 256); // logs_bloom
    assert((rlp_list.length[7] as u32) <= 32); // difficulty
    assert((rlp_list.length[8] as u32) <= 8); // number
    assert((rlp_list.length[9] as u32) <= 8); // gas_limit
    assert((rlp_list.length[10] as u32) <= 8); // gas_used
    assert((rlp_list.length[11] as u32) <= 8); // timestamp
    assert((rlp_list.length[12] as u32) <= 32); // extra_data
    assert(rlp_list.length[13] == 32); // mix_hash
    assert(rlp_list.length[14] == 8); // nonce
    assert((rlp_list.length[15] as u32) <= 8); // base_fee_per_gas
    assert(rlp_list.length[16] == 32); // withdrawals_root

    // Check for state root inclusion
    let sr_off = rlp_list.offset[3];
    for i in 0..32
    {
        assert(state_root[i] == block_header[sr_off + i]);
    }

    // Check hash
    // First compute the actual length of the block header
    let block_header_length = {
        let rlp_header = rlp::decode_len(block_header);
        rlp_header.length + rlp_header.offset
    } as u32;

    // Then feed it into Keccak256
    let block_header_hash = keccak256(block_header, block_header_length);
    assert(block_header_hash == block_hash);

    true
}

// Verify that the given storage root corresponds to the given account
fn verify_account_storage<N>(
    account_state: [u8; N],
    storage_root: [u8; 32]
) -> bool
{
    // Decode account state
    let rlp_list: rlp::RLP_List<4> = rlp::decode1_small_lis(account_state);
    
    // Check for 4 slots
    assert(rlp_list.num_fields == 4);

    // Check lengths
    assert((rlp_list.length[0] as u32) <= 32);
    assert((rlp_list.length[1] as u32) <= 32);
    assert(rlp_list.length[2] == 32);
    assert(rlp_list.length[3] == 32);

    // Check for storage root inclusion
    let sr_off = rlp_list.offset[2];
    for i in 0..32
    {
        assert(storage_root[i] == account_state[sr_off + i]);
    }

    true
}

fn u256_from_fields(x: [Field; 2]) -> [u8; 32] // Convert a pair of 128-bit field elements to a 32-byte array
{
    let mut out = [0; 32];
    
    let x_l = x[0].to_be_bytes(16);
    let x_r = x[1].to_be_bytes(16);

    for i in 0..16
    {
        out[i] = x_l[i];
        out[16 + i] = x_r[i];
    }

    out
}

#[test]
fn test_block_hash_verifier()
{
    let block_header = [249, 2, 29, 160, 37, 13, 13, 88, 122, 39, 89, 216, 57, 189, 191, 68, 193, 112, 34, 113, 58, 133, 74, 115, 177, 61, 67, 247, 132, 53, 149, 16, 96, 117, 222, 94, 160, 29, 204, 77, 232, 222, 199, 93, 122, 171, 133, 181, 103, 182, 204, 212, 26, 211, 18, 69, 27, 148, 138, 116, 19, 240, 161, 66, 253, 64, 212, 147, 71, 148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 22, 217, 46, 53, 217, 138, 10, 227, 30, 100, 200, 81, 190, 85, 145, 108, 254, 87, 198, 108, 86, 43, 89, 107, 243, 146, 97, 82, 196, 113, 144, 5, 160, 169, 19, 33, 153, 33, 83, 97, 150, 130, 9, 201, 249, 128, 150, 36, 178, 129, 104, 209, 198, 44, 32, 243, 163, 103, 250, 0, 23, 65, 119, 40, 63, 160, 0, 190, 130, 62, 23, 126, 4, 105, 51, 91, 103, 24, 117, 110, 122, 189, 235, 19, 9, 4, 208, 41, 110, 250, 209, 51, 0, 167, 122, 241, 134, 78, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 1, 131, 102, 145, 183, 131, 18, 122, 175, 132, 100, 174, 225, 215, 128, 160, 20, 119, 166, 56, 74, 55, 218, 186, 216, 52, 240, 74, 126, 155, 35, 172, 245, 168, 82, 134, 247, 202, 115, 87, 82, 72, 233, 129, 92, 136, 2, 147, 136, 0, 0, 0, 0, 0, 0, 0, 0, 132, 52, 39, 112, 192, 160, 86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72, 224, 27, 153, 108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let block_hash = [61, 154, 163, 164, 36, 23, 186, 77, 196, 192, 21, 95, 224, 26, 97, 22, 49, 231, 114, 231, 167, 13, 124, 67, 214, 181, 37, 76, 223, 200, 25, 156];
    let state_root = [22, 217, 46, 53, 217, 138, 10, 227, 30, 100, 200, 81, 190, 85, 145, 108, 254, 87, 198, 108, 86, 43, 89, 107, 243, 146, 97, 82, 196, 113, 144, 5];

    assert(verify_block_hash(block_hash, block_header, state_root));
}
