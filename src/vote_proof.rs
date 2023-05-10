#![allow(non_snake_case)]
#![allow(unreachable_code)] // TODO - remove this after scaffolding is complete
#![allow(unused_imports)] // TODO - remove this after scaffolding is complete
#![allow(unused_variables)] // TODO - remove this after scaffolding is complete

use ark_ec::Group;
use crate::{ElectionIdentifier, ElectionParams, G1, ScalarField, SignaturePLACEHOLDER, StorageProofPLACEHOLDER, VoteChoice, Voter};

#[derive(Debug)]
pub struct PublicInput {
    A_i: G1,
    B_i: G1,
    N_i: ScalarField,
    H_id: ScalarField
}

#[derive(Debug)]
pub struct PrivateInput {
    v_i: ScalarField,
    SIGMA_i: SignaturePLACEHOLDER, // TODO - figure out what type this should be
    TAU_i: SignaturePLACEHOLDER,
    id: ElectionIdentifier,
    RCK_i: G1,
    p_1: StorageProofPLACEHOLDER,
    p_2: StorageProofPLACEHOLDER,
    p_3: StorageProofPLACEHOLDER,
}

#[derive(Debug)]
pub struct VoteProverPackage {
    public_input: PublicInput,
    private_input: PrivateInput,
}

impl Voter {
    pub fn package_vote_for_proving(&self, election_params: &ElectionParams, vote_choice: VoteChoice) -> VoteProverPackage {

        let SIGMA_i; // DS.Sign(registry_key, election_params.identifier);
        let TAU_i; // DS.Sign(registry_key, election_params.identifier);

        // NFT Vote nullifier
        let N_i; // Poseidon(sigma, election_params.identifier); // TODO - confirm that we need identifier here if SIGMA does not already include it

        let r_i : ScalarField = unimplemented!(); // random value
        let A_i = G1::generator() * r_i; // A = g^r_i in multiplicative notation
        let K_i = election_params.tlock.PK_t * r_i; // K = PK_t^r_i in multiplicative notation

        let B_i = unimplemented!(); // Poseidon(K_i, vote_choice, election_params.identifier);

        let p_1= unimplemented!(); // Storage Prove of NFT ownership by voter address
        let p_2= unimplemented!(); // Storage Prove that NFT has not been delegated
        let p_3= unimplemented!(); // Storage Prove that g^RK_i is in the registry under the voter's address

        let proverPackage = VoteProverPackage {
            public_input: PublicInput {
                A_i,
                B_i,
                N_i,
                H_id:  unimplemented!() // Poseidon(election_params.identifier)
            },
            private_input: PrivateInput {
                v_i: vote_choice.into(),
                SIGMA_i,
                TAU_i,
                id: election_params.identifier.clone(),
                RCK_i: G1::generator() * self.RK_i,
                p_1,
                p_2,
                p_3,
            }
        };

        return proverPackage;
    }


}

