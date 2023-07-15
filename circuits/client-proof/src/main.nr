use dep::std::hash::keccak256;

// EdDSA dependencies from stdlib
use dep::std::ec::tecurve::affine::Point;
use dep::std::eddsa::eddsa_poseidon_verify as eddsa_verify;
use dep::std::hash::poseidon::bn254;
use dep::std::ec::consts::te::baby_jubjub;

// Trie proof primitives. TODO: Replace with stdlib equivalents once merged.
use dep::trie::TrieProof;

global MAX_BITS: Field = 256; // Required for bit representation of BJJ associated field element
global DEPTH8_PROOF_SIZE: Field = 4256;

// Data types
struct Signature
{
    r_b8: Point,
    s: Field,
}

struct VoteProverInput<MAX_PROOF_SIZE>
{
    // Public inputs
    a: Point,
    b: Field, // Hash of k,v,id
    n: Field,
    h_id: Field,
    process_id: Field,
    contract_addr: Field,
    chain_id: [Field; 2],
    registry_account_state: [Field; 2], // Storage hash of zkRegistry contract
    nft_account_state: [Field; 2], // Storage hash of NFT contract
    tlcs_pk: Point,
    
    // Private inputs
    v: Field, // in {0,1,2}
    blinding_factor: Field, // TODO: Formerly r: [u1; MAX_BITS]
    signed_id: Signature, //sigma
    voter_address: Field,
    signed_v: Signature, // tau
    nft_id: [Field; 2],
    k: Point,

    registered_pbk: Point,
    registry_key_sp: TrieProof<32, MAX_PROOF_SIZE, 32>, // Proof of registration
    nft_ownership_proof: TrieProof<32, MAX_PROOF_SIZE, 32> // Proof of NFT ownership
}

fn main(
    a: pub [Field; 2],
    b: pub Field, // Hash of k,v,id
    n: pub Field,
    h_id: pub Field,
    process_id: pub Field,
    contract_addr: pub Field,
    chain_id: pub [Field; 2],
    registry_account_state: pub [Field; 2], // Storage hash of zkRegistry contract
    nft_account_state: pub [Field; 2], // Storage hash of NFT contract
    tlcs_pk: pub [Field; 2],
    
    // Private inputs
    v: Field, // in {0,1,2}
    blinding_factor: Field, // TODO: Formerly r: [u1; MAX_BITS]
    signed_id: Signature, //sigma
    voter_address: Field,
    signed_v: Signature, // tau
    nft_id: [Field; 2],
    k: [Field; 2],
    registered_pbk: [Field; 2],
    registry_key_sp: TrieProof<32, DEPTH8_PROOF_SIZE, 32>, // Proof of registration
    nft_ownership_proof: TrieProof<32, DEPTH8_PROOF_SIZE, 32> // Proof of NFT ownership
)
{
    let vote = VoteProverInput {
        a: Point::new(a[0],a[1]),
        b,
        n,
        h_id,
        process_id,
        contract_addr,
        chain_id,
        registry_account_state,
        nft_account_state,
        tlcs_pk: Point::new(tlcs_pk[0], tlcs_pk[1]),
        v,
        blinding_factor,
        signed_id,
        voter_address,
        signed_v,
        nft_id,
        k: Point::new(k[0], k[1]),
        registered_pbk: Point::new(registered_pbk[0], registered_pbk[1]),
        registry_key_sp,
        nft_ownership_proof
    };

    verify_vote(vote);
}

fn verify_vote<MAX_PROOF_SIZE>(vote: VoteProverInput<MAX_PROOF_SIZE>)
{
    let bjj_curve = baby_jubjub().curve;

    // Check signatures
    assert(eddsa_verify(vote.registered_pbk.x, vote.registered_pbk.y, vote.signed_id.s, vote.signed_id.r_b8.x, vote.signed_id.r_b8.y, vote.h_id));
    assert(eddsa_verify(vote.registered_pbk.x, vote.registered_pbk.y, vote.signed_v.s, vote.signed_v.r_b8.x, vote.signed_v.r_b8.y, bn254::hash_1([vote.v])));

    // Check ID hash
    assert(vote.h_id == bn254::hash_6([vote.nft_id[0], vote.nft_id[1], vote.chain_id[0], vote.chain_id[1], vote.process_id, vote.contract_addr]));

    // Check nullifier
    assert(vote.n == bn254::hash_3([vote.signed_id.r_b8.x, vote.signed_id.r_b8.y, vote.signed_id.s]));

    // Check vote encryption
    let b8 = baby_jubjub().base8;
    assert(bjj_curve.mul(vote.blinding_factor, b8).eq(vote.a));
    assert(vote.k.eq(bjj_curve.mul(vote.blinding_factor, vote.tlcs_pk))); // TODO
    assert(vote.b == bn254::hash_7([vote.k.x, vote.k.y, vote.v, vote.chain_id[0], vote.chain_id[1], vote.process_id, vote.contract_addr]));
    assert((vote.v == 0) | (vote.v == 1) | (vote.v == 2)); // Check validity of vote

    // Check registration by verifying x coordinate of public key
    let address_in_bytes = vote.voter_address.to_be_bytes(20);
    let calculated_registry_key = {
        let mut buf = [0; 64]; // key ++ storage_slot
        let mut out = keccak256(buf, 64);

        for i in 0..20
        {
            buf[i + 12] = address_in_bytes[i];
        }

        for i in 0..32
        {
            buf[i + 32] = out[i];
        }

        keccak256(buf, 64)
    };
    
    // Keys should match
    assert(calculated_registry_key == vote.registry_key_sp.key);

    // And value should be what we expect
    let pbk_x_bytes = vote.registered_pbk.x.to_be_bytes(32);
    for i in 0..32
    {
        assert(pbk_x_bytes[i] == vote.registry_key_sp.value[i]);
    }

    // Storage proof verification
    assert(vote.registry_key_sp.verify_storage_root(u256_from_fields(vote.registry_account_state)));

    // Check NFT ownership
    let calculated_nft_key = {
        let mut buf = [0; 64];
        
        buf[63] = 3;
        let nft_id_bytes1 = vote.nft_id[0].to_be_bytes(16);
        let nft_id_bytes2 = vote.nft_id[1].to_be_bytes(16);
        
        for i in 0..16
        {
            buf[i] = nft_id_bytes1[i];
            buf[16 + i] = nft_id_bytes2[i];
        }

        keccak256(buf, 64)
    };
    
    // Keys should match
    assert(calculated_nft_key == vote.nft_ownership_proof.key);

    // And value should be what we expect
    for i in 0..12
    {
        assert(vote.nft_ownership_proof.value[i] == 0);
    }
    
    for i in 0..20
    {
        assert(vote.nft_ownership_proof.value[12 + i] == address_in_bytes[i]);
    }

    // Storage proof verification
    assert(vote.nft_ownership_proof.verify_storage_root(u256_from_fields(vote.nft_account_state)));
}

fn u256_from_fields(x: [Field; 2]) -> [u8; 32] // Convert a pair of 128-bit field elements to a 32-byte array
{
    let mut out = [0; 32];
    
    let a = x[0].to_be_bytes(16);
    let b = x[1].to_be_bytes(16);

    for i in 0..16
    {
        out[i] = a[i];
        out[16 + i] = b[i];
    }

    out
}
