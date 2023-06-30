use nouns_protocol::{wrap, wrap_into, BN254_Fr, PrivateKey, Wrapper};

fn main() {
    let tlcs_prk = PrivateKey::import(vec![1u8; 32]).unwrap();
    let tlcs_pubk = tlcs_prk.public();

    let tlcs_pubk_s: [ethers::core::k256::U256; 2] = wrap_into!(tlcs_pubk);
    let tlcs_prk_s: BN254_Fr = wrap_into!(tlcs_prk.scalar_key());
    let tlcs_prk_s: ethers::core::k256::U256 = wrap_into!(tlcs_prk_s);

    println!("tlcs_prk: {}", tlcs_prk_s);
    println!("tlcs_pubk: '{},{}'", tlcs_pubk_s[0], tlcs_pubk_s[1]);
}
