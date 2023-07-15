// Verification Key Hash: 1e35c1751355024b2b1d2821a9890903d63403bd62a3cba9d4b841c715ae641b
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library HashUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0x1e35c1751355024b2b1d2821a9890903d63403bd62a3cba9d4b841c715ae641b;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000200000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x000000000000000000000000000000000000000000000000000000000000000a) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1ded8980ae2bdd1a4222150e8598fc8c58f50577ca5a5ce3b2c87885fcd0b523) // vk.work_root
            mstore(add(_vk, 0x60), 0x30644cefbebe09202b4ef7f3ff53a4511d70ff06da772cc3785d6b74e0536081) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x00296c1cb4eec86c649adc1c7a181453ffb498b9a046f5ed1dfc5b745fba188c) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x068e6c4b8097f737273c8399ea0821fbe62ee5a5f2ecd96f051c154313eb005f) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x1d93f8e1e76a9fda86c56c5791a74315e2247489c0cefdade2831d5832e2a8e6) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x1de207db59c19f20d9f2a2e8fa5e2c9ff7ed6d0dee386d893a35183f876ef76c) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x2233e5d53352eba066487815250d3f1b243821cc67131a4ba73956280624e074) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x2d24ba7d26e2a82235f1ce540ce4b072604e36e09d526545da1d699bd89847f6) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x2a810bd6ef1c7a8da020d06f685094658612acfa6459930306e62c99c542ed4f) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x15f0e4958dad28a3de75dc23b9fa6081a79e31b30a9616704b8313b5e0f9a624) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x126d4c5890c1ef4b79e8d481866f0cdaf75a94b9443f347bddc980bcb40f7008) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x15b285b63d63b53f54ab04c92b533b04330278312ceb45ed4a38832ebaddb3d4) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x1f60667d80e5b238115aa4079a4f352fa0186f64e7e7253932f6f1245708c2fd) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x205a4041a470635b1f4494acbfe998267b4b316d9f64cd5a037ac9d70b5f197d) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x2df1e2d7fbe3bd17795cf3dbe39a3e9e0174694463dceffe7344475806d44f5f) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x3020276afa4a32e4c3734630289e986d1d6d891bbc09588a39f8acff09168745) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x1231b7c40816b4c908a96ab2a48fb651334af3fd803e71f71c847ac28aa2cb98) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x0b3e36ec8fc4c8d360018eda62a4b0daeba128003cdd00659a2d89d6e4fd59cf) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x1b01c21b5e68590e37c4529b0d455171dd808ec6dbdb0947e42233f902e552aa) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x2488e0934dab3854fbb692808a16d0caf847e4688e5e6b428d648880d8d5b6c7) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x072da32deb4141bd814c753a16a0751d0017e538a556f85282793a38561015f3) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x167444520805d4a54236aeb1858f1dd2ecc845a6dfc40ef00f3a8a2eb699fdd9) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x005981b66926927ebde7b706a3f0da2289bd486a52350ee66b369f54f190cadc) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x136e275aa3ef22130252774a3fe2c992f0950bf5abfc0c4a5cc214c864c980fe) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x22759136e41864a9dd462d37b1c1b1a0037b92ded39fcae16cfa4f31d9344448) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x1b959f99d6d993ead92684e16cc39fc7fae8f1c1b6c079c73a8e9959d7cbe86c) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x1d5b0706210a45aac55a5e6c80262ff9ae5348943b9205ae8c1fa2c8b343c265) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x19629f8b710e7fd1ad13515921e9a7633ba56a5d6016c3d1d7d8653d0a5868ca) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x2b4b7212f7a779bc3e7bd051308e3eb33e8d36bb896813b0b94ac1984e620dc6) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x21a560630d62f27869fa3c4bc91997f4c6baacb6f6f4d08f46a4b492bfac4b6f) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x0192e4e66b6c8d139a6c84f217009e8efdda60a4720a9eda9fbe9f68e96f96f2) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x0cdc30e9fe340ec2c8afff58d9179d6d56468f5fb9f73573c0aba5bc48c2c015) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x15260b8c8ee7b1ec06639f6eb31c0aac912eeccd919d9db9e5179ffb460267f8) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x2a30dc67d60dcbd8471adf7a1b545022512fafa91dd2c1ed7719009ea6e35038) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x25fa27dab130619259a575ddd26997684a395962f83129d976eab97ded6226d4) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x0af324acaf4048a1c66f3e13061f48769b1b78cc682991c1cea83964ba243414) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x123f6044c38e39f7ced66b3bf533d1cea25d3b72109664537a8ce698d1f4b546) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x0dc0e4e9a5f7d87514ebc2afb1b6d039d96dcdbe86f2a1a6f683a9118f31b845) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x0786a3ff444ad5b73566858f6ca1e30a12b41a7af58c1a015d05341ef6ce0707) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x01dcb202c8b46d2c9d8f7bee904fb4eda8af98779a9ab048dff2983cdfea6c48) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x2bb8f602330663d00b796f199473d8b8b97740c406a20ba1a61b9b6a703508a2) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x24f926f3512124a5cdf6f3521c05db5719c4fa57cc7fecb89d76e6a795c8e037) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x0ead5eb2c66ed7d64b7121053f3fcdaa04b89270e4211b3d3fa589fcb18ab0e0) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x160630be1780de5ff47cb8623dd8bc120c69ac95a1f67ced5149079d50e0d5a5) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x013ef9a252b92827faf367509db7a5e3e4eef7db5107160fd3cc37bbda37bd0a) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x14d509b93ceae8fb7a982c36ec9e0a01a67da3da108ff9b1fe1e85f93bce3999) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x02293edf0567673bd0764fd519fa61d9436de422a74d0c65f5da17dc12e46083) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x12ccb59aa59e46ce2680ae24605f21cc63a2d8c9de47aa45315efbfe1d3f12ef) // vk.ID4.y
            mstore(add(_vk, 0x640), 0x00) // vk.contains_recursive_proof
            mstore(add(_vk, 0x660), 0) // vk.recursive_proof_public_input_indices
            mstore(add(_vk, 0x680), 0x260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1) // vk.g2_x.X.c1 
            mstore(add(_vk, 0x6a0), 0x0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0) // vk.g2_x.X.c0 
            mstore(add(_vk, 0x6c0), 0x04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4) // vk.g2_x.Y.c1 
            mstore(add(_vk, 0x6e0), 0x22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55) // vk.g2_x.Y.c0 
            mstore(_omegaInverseLoc, 0x19c6dfb841091b14ab14ecc1145f527850fd246e940797d3f5fac783a376d0f0) // vk.work_root_inverse
        }
    }
}

contract HashVerifier is BaseUltraVerifier {
    function getVerificationKeyHash() public pure override(BaseUltraVerifier) returns (bytes32) {
        return HashUltraVerificationKey.verificationKeyHash();
    }

    function loadVerificationKey(uint256 vk, uint256 _omegaInverseLoc) internal pure virtual override(BaseUltraVerifier) {
        HashUltraVerificationKey.loadVerificationKey(vk, _omegaInverseLoc);
    }
}
