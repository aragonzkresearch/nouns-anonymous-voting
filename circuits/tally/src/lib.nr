use dep::std::hash::poseidon::bn254;
use dep::std::hash::keccak256;

// TODO: election_id in a struct maybe
fn verify_tally<N>(
    b_k: Field,
    // Election identifiers
    process_id: Field,
    contract_addr: Field,
    chain_id: [Field; 2],
    vote_count: [Field; 3], // Vote count
    num_voters: Field, // Number of voters
    k_x: [Field; N],
    k_y: [Field; N],
    v: [Field; N] // Votes
) -> bool
{
    let mut b = [0; N];

    // Check validity of votes
    for x in v
    {
        assert(x*(x-1)*(x-2) == 0);
    }

    // Compute b
    for i in 0..N
    {
        b[i] = bn254::hash_7([k_x[i], k_y[i], v[i], chain_id[0], chain_id[1], process_id, contract_addr]);
    }

    // Compute b_k_prime
    let mut b_k_prime = b[0];
    
    for i in 1..N
    {
        let voter_p = ((i as u32) < (num_voters as u32)) as Field;
        b_k_prime = voter_p*bn254::hash_2([b[i], b_k_prime]) + (1-voter_p)*b_k_prime;

    }

    // Check that b_k_prime agrees with b_k
    assert(b_k == b_k_prime);

    // Verify that the votes have been counted correctly
    for j in 0..3
    {
        let mut v_count = 0;

        for i in 0..N
        {
            let voter_p = ((i as u32) < (num_voters as u32)) as Field;
            v_count += voter_p*(v[i] == j) as Field;
        }

        assert(v_count == vote_count[j]);
    }

    true
}
