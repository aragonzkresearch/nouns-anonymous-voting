use dep::tally::verify_tally;

fn main(b_k: pub Field, process_id: pub Field, contract_addr: pub Field, chain_id: pub [Field; 2], vote_count: pub [Field; 3], num_voters: Field, k_x: [Field; 16], k_y: [Field; 16], v: [Field; 16])
{
    assert(verify_tally(b_k, process_id, contract_addr, chain_id, vote_count, num_voters, k_x, k_y, v));
}
