// Verification Key Hash: 4b794ca380d1ca784a9e4f6b2f4f5d1db17d40a58a52952fcb91c04b8018e602
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library VoteUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0x4b794ca380d1ca784a9e4f6b2f4f5d1db17d40a58a52952fcb91c04b8018e602;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000400000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x000000000000000000000000000000000000000000000000000000000000000f) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1ad92f46b1f8d9a7cda0ceb68be08215ec1a1f05359eebbba76dde56a219447e) // vk.work_root
            mstore(add(_vk, 0x60), 0x30644db14ff7d4a4f1cf9ed5406a7e5722d273a7aa184eaa5e1fb0846829b041) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x0d7279798d80e2b192aed7fdd29ba7ad0dffae498f7de8893894a4497b364729) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x28d1187a8ed8f9c4e174458f3df24c2aac5cc99e5994ff778007a86d1c33ccb2) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x050d3100194b95c7a17da8549a06601d0829a9120a918fb57f2abf5ebea483f6) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x0c60fca63577479fe05b3797f0bfbac6ae73f7a88e4082399bb42bf5ab1329dd) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x175c9ddbee004a0e2d2c9ebbe2653c9d520e6e785266eafe67ca70cf5a5d2d23) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x17f7cedb2fda634b504ce5a9994567215e4afdcbdd38b0a3aa1f6c940baa4538) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x0f4eaba47c8b4aaea45d1c279a06595770c4c6a9dc6088e7ecda90ab8d03f404) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x22f26376f5edfc12e22e8ce1129c1bcd4f5cca063cc4d31893e45a76ebcb0313) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x2a6537a364229f3e39871f97fb330aa3f91e70e5b5048ddb22663ae634be33c2) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x1a6e10843072f1ff580437ee8bb1ec14fd8213ba1f15f619f7823d135ed9151a) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x177dd34869915a872abd624fe99471a75bfdeffa4db44850c6185f0f34554a84) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x2c9b91c6c3913010e1ac82e757302f7c77790411089bc54e50b6fc17f5173dc3) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x216d0e07063accea92b2ad92daad8f4102dd01fccda2c8820a2feb6c06792587) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x2d77537786abda56e9010a1100a93c049c629849bf087e102373c07cfb610523) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x29e321d55fd35804ef3e86e338ef9b78c46039890d0fa3f7b554c89da0abfc83) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x285fccf6fd2dca0ec0392f040ea8d33c396a2d3e5d45cbe8e723a1d835c3ac6c) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x2285906730141888390d1809dd134a79e079f29c10be2c6984fefa51b32920f8) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x053a249dfcef61dcf9140ec7bbfcf1d84538beb151766142e6325ad6a578e73f) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x0c0de646ae85a6df0161d127a9c793ab77a71be88106c793bad04ea5aa1dad4e) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x27dad8b2996bf644c8b6a7d1ae2f83ae617851694212fc7d24493e4dad3a37b3) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x08d030a4ba23b08d17f109226e4a2b3dc917316e3dbc69bd56846fa4eb8f0542) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x0698d6bb197ae63a33ce39d20a04a4e0734be50af9f6891c12d9487abdd26eb2) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x16e9a834129069e008f74ce3fe2480b75696d35271b7ca1706774af96268b681) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x1ac44eac8c4f1238cf075b612dca9782c1bb86655d916889a9714a191e42eed9) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x0ce73b81f37f8c9e550a3f6e530c939f68dfb969f35c79c1c475dbb26e7f482f) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x192bbf9697c2d1fbce8c24bcce112f614d284d3b65dace48d2c4fc9eeb3b942c) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x27aaa9f1bbddbf3e7d8ed424bf122d995af9ce150f213d7917317bd797264407) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x16cb45aa99cd247447f0c010d3a00f5b8c7e9013f3e18b2335b46e5a71e365e6) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x1a3c2f950aa30450cd4ed8b4654e42ce5f9a4e51e5949a738b6672e13a213d63) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x182c0b193b8e0fcf34805cfec0846885df6fc0cf93f84e784ad0cd15b518bb09) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x035abd0fecc9372647a55fec3be67f623aa8e7e348d4b072536b3aefb9f5ae2f) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x100aab1c1c391785ebd4729d8d303dcf7418799b6c371ceb196fc2659435819d) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x101a69a00c7a8787eef07528f34995f63693bff44daf5ddac26bfaca840959b2) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x1373014d7f5cc5832a65acd1b7d296f75c627cc47e8328ca218a5cd0d261c29f) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x05dca48aa19130889aa6ba21dc462f3f96162bc5db003e8af9cf8d6c22d73906) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x27b2f27dfc439a25d004ebc5b10cb923857904f3596fa3c84b4de44fa66e7c2a) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x22e3189d69b10412e03310ea6623da79b2c01b42eeec3ae25744925fa4316a0f) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x00efb85581be03ba3ba9e4e456ca974b96474884035041cb9ade07862f30c6f2) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x0cf87faab2640d55e33b65797c2079bcfead049ed287eaa8f1f9a91516f62a66) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x10c7b65c1b95c7fabaccbd2142c8dc3b4f57b9f9d9c449226d975cffd1eeddac) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x2957c962909b541f0f530a9162abd13e174b408d56f39cf42142881f0f38eb10) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x114f88a9cf5094a2968e732080e591aac621d6801b3490fb2e08e9bb7f61a5b2) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x0c5a2970e23fecbdcc3c218f90655c0d45e3b68ae47c2919e1056d100d82bde3) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x011dccf191867e6bd888663944773571bb480f79ed3116633646d1e46dcce442) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x1a4cec6afe5b2ef574abdd340aef52ec5a90f7cc0d93931f425fcc5eb965e848) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x26e35f45012fd0dd1dd2b1b238a347d3efa16ec6fa97f46fa8e622a2393233a4) // vk.ID4.y
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
