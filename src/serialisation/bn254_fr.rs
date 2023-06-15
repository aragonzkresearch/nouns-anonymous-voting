use crate::election::{ElectionIdentifier, VoteChoice};
use crate::{BBJJ_Fr, BN254_Fr, BBJJ_G1};
use ark_ff::{BigInt, BigInteger};
use babyjubjub_ark::Signature;

use crate::preprover::StorageProof;
use crate::serialisation::Wrapper;

impl Into<BN254_Fr> for Wrapper<BBJJ_Fr> {
    fn into(self) -> BN254_Fr {
        // As BN254_Fr == BBJJ_Fq > BBJJ_Fr the value fits in the new field
        BN254_Fr::new(self.0 .0)
    }
}

impl Into<Vec<BN254_Fr>> for Wrapper<Signature> {
    fn into(self) -> Vec<BN254_Fr> {
        vec![self.0.r_b8.x, self.0.r_b8.y, Wrapper(self.0.s).into()]
    }
}

impl Into<Vec<BN254_Fr>> for Wrapper<BigInt<4>> {
    fn into(self) -> Vec<BN254_Fr> {
        // Split the BigInt into two BN254_Frs and return them as an array
        let bytes = self.0.to_bits_be();
        // Split the bytes into two halves
        let x = &bytes[..128];
        let y = &bytes[128..];
        // Convert the halves into BN254_Frs
        let x = BN254_Fr::new(BigInt::from_bits_be(x));
        let y = BN254_Fr::new(BigInt::from_bits_be(y));
        vec![x, y]
    }
}

impl Into<Vec<BN254_Fr>> for Wrapper<BBJJ_G1> {
    fn into(self) -> Vec<BN254_Fr> {
        vec![self.0.x, self.0.y]
    }
}

impl Into<Vec<BN254_Fr>> for ElectionIdentifier {
    fn into(self) -> Vec<BN254_Fr> {
        vec![self.chain_id, self.process_id, self.contract_addr]
    }
}

// Implement conversion trait for Vote to F
impl Into<BN254_Fr> for VoteChoice {
    fn into(self) -> BN254_Fr {
        match self {
            VoteChoice::Yes => BN254_Fr::from(0u8),
            VoteChoice::No => BN254_Fr::from(1u8),
            VoteChoice::Abstain => BN254_Fr::from(2u8),
        }
    }
}
