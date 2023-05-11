#![allow(non_snake_case)]
#![allow(unreachable_code)] // TODO - remove this after scaffolding is complete
#![allow(unused_imports)] // TODO - remove this after scaffolding is complete
#![allow(unused_variables)] // TODO - remove this after scaffolding is complete

use std::ops::Mul;
use ark_bn254::{Fr, G1Projective};
use ark_bn254::g1::Config;
use ark_ec::Group;
use ark_ec::twisted_edwards::Projective;
use ark_ff::{BigInt, BigInteger};
use ark_std::rand::Rng;
use ark_std::UniformRand;
use babyjubjub_ark::Signature;
use poseidon_ark::Poseidon;
use toml::Value;
use crate::{concat_vec, ElectionIdentifier, ElectionParams, BN254_G1, VoteChoice, Voter, BN254_Fr, BBJJ_Fr, BBJJ_G1};
use crate::snark_repr::{TomlSerialisable, Wrapper};


#[derive(Debug)]
pub struct StorageProofPLACEHOLDER {
    // TODO - parametrise this with Ahmad's work
}

impl Into<Vec<BN254_Fr>> for StorageProofPLACEHOLDER {
    fn into(self) -> Vec<BN254_Fr> {
        vec![]
    }
}

impl TomlSerialisable for StorageProofPLACEHOLDER {
    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        Value::Table(map)
    }
}

#[derive(Debug)]
pub struct PublicInput {
    A_i: BN254_G1,
    B_i: BN254_Fr,
    N_i: BN254_Fr,
    H_id: BN254_Fr
}

#[derive(Debug)]
pub struct PrivateInput {
    v_i: VoteChoice,
    SIGMA_i: Signature,
    TAU_i: Signature,
    id: ElectionIdentifier,
    RCK_i: BBJJ_G1,
    p_1: StorageProofPLACEHOLDER,
    p_2: StorageProofPLACEHOLDER,
    p_3: StorageProofPLACEHOLDER,
}

#[derive(Debug)]
pub struct VoteProverPackage {
    public_input: PublicInput,
    private_input: PrivateInput,
}

// #[derive(Debug)]
// pub struct SerialisedPublicInput {
//     A_i: Vec<BN254_Fr>,
//     B_i: BN254_Fr,
//     N_i: BN254_Fr,
//     H_id: BN254_Fr
// }
//
// impl From<PublicInput> for SerialisedPublicInput {
//     fn from(public_input: PublicInput) -> Self {
//         SerialisedPublicInput {
//             A_i: Wrapper(public_input.A_i).into(),
//             B_i: public_input.B_i.into(),
//             N_i: public_input.N_i.into(),
//             H_id: public_input.H_id.into(),
//         }
//     }
// }

impl TomlSerialisable for PublicInput {

    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        map.insert("A_i".to_string(), <Wrapper<BN254_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.A_i)).toml());
        map.insert("B_i".to_string(), self.B_i.toml());
        map.insert("N_i".to_string(), self.N_i.toml());
        map.insert("H_id".to_string(), self.H_id.toml());
        Value::Table(map)
    }
}

// #[derive(Debug)]
// pub struct SerialisedPrivateInput {
//     v_i: BN254_Fr,
//     SIGMA_i: Vec<BN254_Fr>,
//     TAU_i: Vec<BN254_Fr>,
//     id: ElectionIdentifier,
//     RCK_i: Vec<BN254_Fr>,
//     p_1: Vec<BN254_Fr>,
//     p_2: Vec<BN254_Fr>,
//     p_3: Vec<BN254_Fr>,
// }
//
// impl From<PrivateInput> for SerialisedPrivateInput {
//     fn from(value: PrivateInput) -> Self {
//         SerialisedPrivateInput {
//             v_i: value.v_i.into(),
//             SIGMA_i: Wrapper(value.SIGMA_i).into(),
//             TAU_i: Wrapper(value.TAU_i).into(),
//             id: value.id,
//             RCK_i: Wrapper(value.RCK_i).into(),
//             p_1: Wrapper(value.p_1).into(),
//             p_2: Wrapper(value.p_2).into(),
//             p_3: Wrapper(value.p_3).into(),
//         }
//     }
// }

impl TomlSerialisable for PrivateInput {

    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        map.insert("v_i".to_string(), <VoteChoice as Into<BN254_Fr>>::into(self.v_i).toml());
        map.insert("SIGMA_i".to_string(), <Wrapper<Signature> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.SIGMA_i)).toml());
        map.insert("TAU_i".to_string(), <Wrapper<Signature> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.TAU_i)).toml());
        map.insert("id".to_string(), self.id.toml());
        map.insert("RCK_i".to_string(), <Wrapper<BBJJ_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(self.RCK_i)).toml());
        map.insert("p_1".to_string(), <StorageProofPLACEHOLDER as Into<Vec<BN254_Fr>>>::into(self.p_1).toml());
        map.insert("p_2".to_string(), <StorageProofPLACEHOLDER as Into<Vec<BN254_Fr>>>::into(self.p_2).toml());
        map.insert("p_3".to_string(), <StorageProofPLACEHOLDER as Into<Vec<BN254_Fr>>>::into(self.p_3).toml());
        Value::Table(map)
    }
}

// #[derive(Debug)]
// pub struct SerialisedVoteProverPackage {
//     public_input: SerialisedPublicInput,
//     private_input: SerialisedPrivateInput,
// }
//
// impl From<VoteProverPackage> for SerialisedVoteProverPackage {
//     fn from(value: VoteProverPackage) -> Self {
//         SerialisedVoteProverPackage {
//             public_input: value.public_input.into(),
//             private_input: value.private_input.into(),
//         }
//     }
// }

impl TomlSerialisable for VoteProverPackage {

    fn toml(self) -> Value {
        let mut map = toml::map::Map::new();
        map.insert("public_input".to_string(), self.public_input.toml());
        map.insert("private_input".to_string(), self.private_input.toml());
        Value::Table(map)
    }
}





impl Voter {
    pub fn package_vote_for_proving<R: Rng>(&self, rng: &mut R, election_params: &ElectionParams, v_i: &VoteChoice, nft_id: &BN254_Fr) -> Result<VoteProverPackage, String> {

        let poseidon = Poseidon::new();

        // Generate signatures for the vote and nullifier using the voter's registry key
        // Note that we first hash the messages and only then sign them
        let H_id = poseidon.hash(vec![*nft_id, election_params.identifier.chain_id, election_params.identifier.process_id, election_params.identifier.contract_addr])?;
        let SIGMA_i= self.RK_i.sign(H_id)?; // DS.Sign(registry_key, election_params.identifier);
        let vote_choice_message = poseidon.hash(vec![v_i.clone().into()])?;
        let TAU_i= self.RK_i.sign(vote_choice_message)?; // DS.Sign(registry_key, vote_choice);

        // Generate the nullifier
        let N_i = poseidon.hash(Wrapper(SIGMA_i.clone()).into())?; // Poseidon(sigma, election_params.identifier); // TODO - confirm that we need identifier here if SIGMA does not already include it

        let r_i : BN254_Fr = BN254_Fr::rand(rng); // random value
        let A_i = BN254_G1::generator() * r_i; // A = g^r_i in multiplicative notation
        let K_i = election_params.tlock.PK_t * r_i; // K = PK_t^r_i in multiplicative notation

        let B_i = poseidon.hash(concat_vec![<Wrapper<BN254_G1> as Into<Vec<BN254_Fr>>>::into(Wrapper(K_i)), vec![v_i.clone().into(), election_params.identifier.chain_id, election_params.identifier.process_id, election_params.identifier.contract_addr]])?; // Poseidon(K_i, vote_choice, election_params.identifier);

        let p_1 = StorageProofPLACEHOLDER {}; // TODO // Storage Prove of NFT ownership by voter address
        let p_2 = StorageProofPLACEHOLDER {}; // TODO // Storage Prove that NFT has not been delegated
        let p_3 = StorageProofPLACEHOLDER {}; // TODO // Storage Prove that g^RK_i is in the registry under the voter's address

        let proverPackage = VoteProverPackage {
            public_input: PublicInput {
                A_i,
                B_i,
                N_i,
                H_id
            },
            private_input: PrivateInput {
                v_i: v_i.clone(),
                SIGMA_i,
                TAU_i,
                id: election_params.identifier.clone(),
                RCK_i: self.RK_i.public(),
                p_1,
                p_2,
                p_3,
            }
        };

        return Ok(proverPackage);
    }


}

