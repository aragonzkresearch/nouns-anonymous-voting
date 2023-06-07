// Required modules (taken from Noir PR)
// TODO: Use stdlib versions once merged

use dep::std::ec::tecurve::affine::Point;
use dep::std::eddsa::eddsa_poseidon_verify as eddsa_verify;
use dep::std::hash::poseidon::bn254;
use dep::std::ec::consts::te::baby_jubjub;

global MAX_BITS: Field = 256; // Required for bit representation of BJJ associated field element
global DEPTH8_PROOF_SIZE: Field = 4256;

// Data types
// TODO: Use storage proof library
struct StorageProof<MAX_PROOF_SIZE>
{
    path: [u8; MAX_PROOF_SIZE],
    depth: Field
}

struct Signature {
    r_b8: Point,
    s: Field,
}

struct ElectionIdentifier
{
    chain_id: Field,
    process_id: Field,
    contract_addr: Field // 160 bits
}

struct VoteProverPackage<MAX_PROOF_SIZE> {
    // Public inputs
    a: Point,
    b: Field, // Hash of k,v,id
    nullifier: Field,
    id_hash: Field,
    election_id: ElectionIdentifier,
    r: [u1; MAX_BITS],
    
    // Private inputs
    k: Point,
    nft_id: Field,
    v: Field, // in {0,1,2}
    sigma: Signature,
    tau: Signature,
    rck: Point,
    p_1: StorageProof<MAX_PROOF_SIZE>,
    p_2: StorageProof<MAX_PROOF_SIZE>,
    p_3: StorageProof<MAX_PROOF_SIZE>,
}

fn main(
    a: pub [Field; 2],
    b: pub Field,
    nullifier: pub Field,
    id_hash: pub Field,
    election_id: pub [Field; 3],
    r: pub [u1; MAX_BITS],
    pk_t: pub [Field; 2],
    k: [Field; 2],
    nft_id: Field,
    v: Field,
    sigma: [Field; 3],
    tau: [Field; 3],
    rck: [Field; 2],
    p_1: StorageProof<DEPTH8_PROOF_SIZE>,
    p_2: StorageProof<DEPTH8_PROOF_SIZE>,
    p_3: StorageProof<DEPTH8_PROOF_SIZE>
) {
    let vote_package = VoteProverPackage { a: Point::new(a[0],a[1]),
                                     b,
                                     nullifier,
                                     id_hash,
                                     election_id: ElectionIdentifier {
                                         chain_id: election_id[0],
                                         process_id: election_id[1],
                                         contract_addr: election_id[2]
                                     },
                                     r,
                                     k: Point::new(k[0], k[1]),
                                     nft_id,
                                     v,
                                     sigma: Signature { r_b8: Point::new(sigma[0], sigma[1]),
                                                        s: sigma[2]},
                                     tau: Signature { r_b8: Point::new(tau[0], tau[1]),
                                                      s: tau[2]},
                                     rck: Point::new(rck[0], rck[1]),
                                     p_1,
                                     p_2,
                                     p_3
    };

    verify_vote_package(vote_package, Point::new(pk_t[0], pk_t[1]))
}

fn verify_vote_package<MAX_PROOF_SIZE>(vote_package: VoteProverPackage<MAX_PROOF_SIZE>, pk_t: Point)
{
    let bjj_curve = baby_jubjub().curve;

    // Check signatures
    assert(eddsa_verify(vote_package.rck.x, vote_package.rck.y, vote_package.sigma.s, vote_package.sigma.r_b8.x, vote_package.sigma.r_b8.y, vote_package.id_hash));
    assert(eddsa_verify(vote_package.rck.x, vote_package.rck.y, vote_package.tau.s, vote_package.tau.r_b8.x, vote_package.tau.r_b8.y, bn254::hash_1([vote_package.v])));

    // Check ID hash
    assert(vote_package.id_hash == bn254::hash_4([vote_package.nft_id, vote_package.election_id.chain_id, vote_package.election_id.process_id, vote_package.election_id.contract_addr]));

    // // Check nullifier
    assert(vote_package.nullifier == bn254::hash_3([vote_package.sigma.r_b8.x, vote_package.sigma.r_b8.y, vote_package.sigma.s]));

    // // Check vote encryption
    let b8 = baby_jubjub().base8;
    assert(bjj_curve.bit_mul(reverse(vote_package.r), b8).eq(vote_package.a));// & vote_package.k.eq(bjj_curve.bit_mul(vote_package.r, pk_t))); // TODO
    assert(vote_package.k.eq(bjj_curve.bit_mul(reverse(vote_package.r), pk_t)));
    assert(vote_package.b == bn254::hash_6([vote_package.k.x, vote_package.k.y, vote_package.v, vote_package.election_id.chain_id, vote_package.election_id.process_id, vote_package.election_id.contract_addr]));
    assert((vote_package.v == 0) | (vote_package.v == 1) | (vote_package.v == 2));

    // TODO: Storage proofs
    let _p1 = vote_package.p_1;
    let _p2 = vote_package.p_2;
    let _p3 = vote_package.p_3;
}

fn reverse<T,N>(x: [T; N]) -> [T; N]
{
    let mut out = x;

    for i in 0..N
    {
        out[i] = x[N - 1 - i];
    }

    out
}