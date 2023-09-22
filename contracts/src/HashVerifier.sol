// Verification Key Hash: 5a24b545f240916d897f2b1620b119486f4f774f6057f55739444bf28c3d8c8a
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library HashUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0x5a24b545f240916d897f2b1620b119486f4f774f6057f55739444bf28c3d8c8a;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000400000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x0000000000000000000000000000000000000000000000000000000000000009) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1ad92f46b1f8d9a7cda0ceb68be08215ec1a1f05359eebbba76dde56a219447e) // vk.work_root
            mstore(add(_vk, 0x60), 0x30644db14ff7d4a4f1cf9ed5406a7e5722d273a7aa184eaa5e1fb0846829b041) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x2013fe82d3f57791e7639843191f858d8baae786fb4a150116224022391e4baf) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x10db38e181ef5ff11dd8af00df3966da63cd2655fa452b3fcf05df473bc183fa) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x2364a70cd5ab2460989bde5f546d4feccd9cee3d5a0014251c6dfd8844f7d4f8) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x158cea7b8a7d3a92f7192ce12e565e3ea440ae10b28f439757a61f79f2d99611) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x0ace3212a567ffdf5e3e1b04a8bfe646530aa480114f28b09116ef8bbb9eb776) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x0e4c6a98a4bb8d2bf5b459e2f32394e916b68e5476a042516ec2fcc8020f2b35) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x19e55d9c1495e1da34a92d676a8ce53da66c546dc585f472063bc895b5ae71b9) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x2a2746eebb1142098437100d0c92d030d55aaae4a6b1db2249e9502384b1b375) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x26fe4d6a0517c78d322fa817575b7db742912456f33e4a4475f4e9cff080a663) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x1d9ef76b29e301a5c1dfcada23f3743105334d321122cee668ff5734e124a83e) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x00aa32ff380656a7f5e1e637cbabb2eca62fb6e7aeb880d831925cb583f20e58) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x29119ba0fb07dcd8d3525311d87a21bf8f26dca4c54110fe10d41c41be54e765) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x13f1a9e778754f268f02fb06145c6b9a3ff718623ca79beac6cc806985f37fce) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x22699d4abf74d25a5fe5dc8e45fccfd16fe79e6e33c9982a7733df04a5b7b252) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x168580fb798b231994b827ce339eab2a5e6797ea37f11e4dca8d1b1d73fd9488) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x1d87f9d4316da4d221e02aa11210bb2cb3a5241802676d02703d2e24c3235ea8) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x2285906730141888390d1809dd134a79e079f29c10be2c6984fefa51b32920f8) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x053a249dfcef61dcf9140ec7bbfcf1d84538beb151766142e6325ad6a578e73f) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x2130c67afb5eaeba132f484fe2914d8f60d9012e3f3b352b72a7bfb900dd5ad3) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x04c26541f2120002e059cbf7384814280e8bdb218460ce4991b1d505eb32319d) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x0b45359bffda0e27a2dc4d4447404021b5f5a55916ae557ea96f81b043e566c5) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x247bb2dbf003c1976ff0a52c08b19f15e3a1d01e9527ee1071402e095660602e) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x27fb9181a907c50084ed331b118ef0ba48bb809b912edddaa38a4b84393039f4) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x19ec60233d61079d1bf7344a8e8358bba41a353e95a7568100a76b1047000a03) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x11be71b8d981f388144ecab1debaca4e1b2d2bd3813b975d13a53ef4a13ec7fc) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x09bb4538bba09ff61f6779d9776bc89f4040b2d27370ecf7cee2fe8fffb5c34a) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x002e107b965159a87d182556cace72e4977597f928e6d57e58a5d67c9a86bcd8) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x19ebe7a203263a299939500255790bd1ad7f9706dcbea2aaf4497bc6c682fd2c) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x1a3c2f950aa30450cd4ed8b4654e42ce5f9a4e51e5949a738b6672e13a213d63) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x182c0b193b8e0fcf34805cfec0846885df6fc0cf93f84e784ad0cd15b518bb09) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x035abd0fecc9372647a55fec3be67f623aa8e7e348d4b072536b3aefb9f5ae2f) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x100aab1c1c391785ebd4729d8d303dcf7418799b6c371ceb196fc2659435819d) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x101a69a00c7a8787eef07528f34995f63693bff44daf5ddac26bfaca840959b2) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x1373014d7f5cc5832a65acd1b7d296f75c627cc47e8328ca218a5cd0d261c29f) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x05dca48aa19130889aa6ba21dc462f3f96162bc5db003e8af9cf8d6c22d73906) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x27b2f27dfc439a25d004ebc5b10cb923857904f3596fa3c84b4de44fa66e7c2a) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x23b6cc861d24d071d1ca492831f9c6689bd5e4718d0bd151c0f08936feb73f9d) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x167f65b708a57e4232a16c395af81763abf688ece6d1b863a4aa4ab9b1ca00d8) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x209b86fdd45229fba5e7cb6d131804a3f5d66671173c5b7695577e3a5e5dce69) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x12a3e11fd5590fd770e967a204f393b6c9924b2bd82fc80646015f85f1549f48) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x0a698593ee34d541383b328c93adcfb0e5d75cb0647d4eadd79bba866f9cf36b) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x0a3dbbce392791f9d6c24dec4b2c8a501ec2791af516a469254a1836fc258177) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x03d81accbc2691d66a6b856d1ed6562525e4afdda492a5572495d747aedda415) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x0230063f402413ca01fb4bd555d913bd7ccce15d7be42d9c924f824d442b1ec1) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x02d5cf6d245e18b1302d490c40d74ce7a60d3cac8027319b9ede36757a343e27) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x2f860f1fabf74c87e5b3a110b59fee54f31b57be9bf0cd2caf47be9ead0fd103) // vk.ID4.y
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

contract HashVerifier is BaseUltraVerifier {
    function getVerificationKeyHash() public pure override(BaseUltraVerifier) returns (bytes32) {
        return HashUltraVerificationKey.verificationKeyHash();
    }

    function loadVerificationKey(uint256 vk, uint256 _omegaInverseLoc) internal pure virtual override(BaseUltraVerifier) {
        HashUltraVerificationKey.loadVerificationKey(vk, _omegaInverseLoc);
    }
}
