use dep::tally::verify_tally;

fn main(b_k: Field, process_id: Field, contract_addr: Field, chain_id: [Field; 2], vote_count: [Field; 3], num_voters: Field, k_x: [Field; 512], k_y: [Field; 512], v: [Field; 512])
{
    assert(verify_tally(b_k, process_id, contract_addr, chain_id, vote_count, num_voters, k_x, k_y, v));
}
