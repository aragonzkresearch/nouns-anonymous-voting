// Verification Key Hash: 0d897f9af3cf859c2d5b54f6e445f2857d3f91ef3fa905227344157ba04a1ea8
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library VoteUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0x0d897f9af3cf859c2d5b54f6e445f2857d3f91ef3fa905227344157ba04a1ea8;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000400000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x000000000000000000000000000000000000000000000000000000000000000e) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1ad92f46b1f8d9a7cda0ceb68be08215ec1a1f05359eebbba76dde56a219447e) // vk.work_root
            mstore(add(_vk, 0x60), 0x30644db14ff7d4a4f1cf9ed5406a7e5722d273a7aa184eaa5e1fb0846829b041) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x245930dfae07efb037b8863f246b10529233bd6544007197393e49a3f3924bb2) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x0414d2c855ab00bfb17ab8aa4dc858ea88a55e6615c6381ad44bb00663915e61) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x10961146746346a5b43beb8135dc629828030a456e8b98039615b81dae655a7a) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x29dd0e06c477acc68624fceb2923ac704442891ca9b5bcb9d05c026337a87b65) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x02121d4bb53df0b1def9306e302b052efc99113b44664ba90efae48233e8a4a5) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x13c740e7e3141081948d79195380316f22fce0b5c2a2751e5ad6bd8d80241307) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x085026d1a14f92fa149e22385bb9780236d93715a33501dad21634c7aea6997b) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x20266235b93c02251942c43e27df697437fb719c9b5b99bad084be52811a8e7c) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x2c993c1bcfb6beb8b92e098b80ab14452284acd96894f75bc2725b36e4293433) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x04b35c46185b379a0bddf7c0cb42724033015d2bacb62ae0586c7c80297c8687) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x08d481bc21155da63fc612262f5dc5c9a91657b60882f0455be152cb9575c49e) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x239e94f0cfbd72fbea22a9b47847f92aaa69866262ec5b2d92e5a44173b5f986) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x0b27d49c0c45b82c556bb0c5f1221cdd0f7f384d7a7835e75cc49c65441481fa) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x234d3f0e4fb91a4554db3fcf76a41f8968694720eb5200d6b5e6d6fbbfbba325) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x2a8be43f2a554ed2bf7bd878c7f66debdc6e7f5962c8ea4063c2f3103de2e70b) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x253a30fcc3b7d9f3379167e26284ad6b15159243d386fec19491b0ed2a7a517f) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x2285906730141888390d1809dd134a79e079f29c10be2c6984fefa51b32920f8) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x053a249dfcef61dcf9140ec7bbfcf1d84538beb151766142e6325ad6a578e73f) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x2b008a8204e7b2bd76be1d6369a782eb56cbc7e3d52868c6b52f0018b0c72668) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x2e936849a17a65e15eda80ccf87245c1d437be34156fcaf0ff0e368630cff8e0) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x0db52675aa531db22fc85af9f3cc48d15d7e7f587c0789f8a9a927cea13fbc1a) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x1e33234258e56d1a16b86801e6dcd59976a4c72f5ac21d85b2c0f8e33a65a889) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x02bc534b640fe2ba67aa14cb5d7cd4494f515a64845bcef08cc2fb7995afa48e) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x1d9942c289e10f5df4cb588eef12d9e6878f410837e6dea6251986641ea9e253) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x2df74520ecb88c2319f692e42dbfbc3323e4cd09bf88d75718d116e66dfe2568) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x03ec5506c7aa05901ff275f85ce05b0332264913f7652da13dd17a7b2979e783) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x17e598b1fa3dc5dde0a02035017be3bc3e9633c9b91500f58f7884b88b6eae70) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x2109675f3870186ce742926fffd2cc9b63d11cb4ab07578976b1e58006e04e14) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x1a3c2f950aa30450cd4ed8b4654e42ce5f9a4e51e5949a738b6672e13a213d63) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x182c0b193b8e0fcf34805cfec0846885df6fc0cf93f84e784ad0cd15b518bb09) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x035abd0fecc9372647a55fec3be67f623aa8e7e348d4b072536b3aefb9f5ae2f) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x100aab1c1c391785ebd4729d8d303dcf7418799b6c371ceb196fc2659435819d) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x101a69a00c7a8787eef07528f34995f63693bff44daf5ddac26bfaca840959b2) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x1373014d7f5cc5832a65acd1b7d296f75c627cc47e8328ca218a5cd0d261c29f) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x05dca48aa19130889aa6ba21dc462f3f96162bc5db003e8af9cf8d6c22d73906) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x27b2f27dfc439a25d004ebc5b10cb923857904f3596fa3c84b4de44fa66e7c2a) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x25696b5b7f348fdd478d830b18764ae85e23f00cb383d39fd2cfcf15120d3fa0) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x2a5a31e38ab68326c7305282dae8cf40894c727504cae1aec8bf5b56d0bd1d36) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x1673867bb68643b76e9e5389dad1475eab62e69978d10c1ad396dec1b9d0da80) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x2d208123ca77a2b59bc95f8ce6fc9d33eb78394ecdfbd23d3040d6b509f48205) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x1d2453f0a15311f663828e5a6fadea979826ef690c12fa02c469dc9835e42aca) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x1d7b0a686d997ea1db1c4c2ece06d0dc830452798f8329cbecb22f17739f3468) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x17faaa47661dea097760fe4152565cdf3cc6c648a1cf385409594d0f6f7abb23) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x2bc6ffb7bd34e14cd5bf7cb4bcb7d3ac6a9b74b468fd7a6a73a49a14f66921fb) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x25326f56f834e9e4a05780d5e9a6050e6707a21bf76baedb8a62ff295652a420) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x184f33025df3fe12302ac2b2f5cbdcea9188098309ad0b38188b501936be4a0f) // vk.ID4.y
            mstore(add(_vk, 0x640), 0x00) // vk.contains_recursive_proof
            mstore(add(_vk, 0x660), 0) // vk.recursive_proof_public_input_indices
            mstore(add(_vk, 0x680), 0x260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1) // vk.g2_x.X.c1 
            mstore(add(_vk, 0x6a0), 0x0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0) // vk.g2_x.X.c0 
            mstore(add(_vk, 0x6c0), 0x04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4) // vk.g2_x.Y.c1 
            mstore(add(_vk, 0x6e0), 0x22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55) // vk.g2_x.Y.c0 
            mstore(_omegaInverseLoc, 0x2eb584390c74a876ecc11e9c6d3c38c3d437be9d4beced2343dc52e27faa1396) // vk.work_root_inverse
        }
    }
}

contract VoteVerifier is BaseUltraVerifier {
    function getVerificationKeyHash() public pure override(BaseUltraVerifier) returns (bytes32) {
        return VoteUltraVerificationKey.verificationKeyHash();
    }

    function loadVerificationKey(uint256 vk, uint256 _omegaInverseLoc) internal pure virtual override(BaseUltraVerifier) {
        VoteUltraVerificationKey.loadVerificationKey(vk, _omegaInverseLoc);
    }
}
