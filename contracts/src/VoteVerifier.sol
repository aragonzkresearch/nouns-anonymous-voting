// Verification Key Hash: fa5630726dcef17f79649e4cf0df2a6435390bdfb3e8027700c94a3b01a5b1a8
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library VoteUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0xfa5630726dcef17f79649e4cf0df2a6435390bdfb3e8027700c94a3b01a5b1a8;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000400000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x000000000000000000000000000000000000000000000000000000000000000e) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1ad92f46b1f8d9a7cda0ceb68be08215ec1a1f05359eebbba76dde56a219447e) // vk.work_root
            mstore(add(_vk, 0x60), 0x30644db14ff7d4a4f1cf9ed5406a7e5722d273a7aa184eaa5e1fb0846829b041) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x01c6c31527299a2e93f06c2db9317e8623943eb7f090c14f7b7df77fc68e0926) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x02ad0e566f3f9a4e9f2a1a88e5860a161b06ea52b14ecced7dbcb163997c4ba8) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x0ff7eff76240e056e384904341e2f08d3c7d848a02a5f7adcefa184b47b154f4) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x02a2cb92aed76f9b13ee61e6c7c37e65aa31733fb572b01710174eebc1662879) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x0d826ee83a23337e629e2d4c884653c0d60adafe8af4d31afeb989b7ab9f0da9) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x08877231d47ab43b12944aac5eff2dfaae66e011309ee6a21a410139039a5820) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x1667dbf4e9ac53f10bf0e58cd4d291da6538d9baaee1d656ed8b04164561a416) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x03a30d231a1f9179934275be1cf40993b01d6b1d80bd9cb567fbe8f80e1e5610) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x0a8e2ed766c759a5c906a4503f34053e82ba4f5da796324bac039fa0e63a6e23) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x1dac5f494b4715b0090a10b4a1376f5450bd88113af28a673948ed1c31eafcd9) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x2c01a00fc101b7aee5d4801607032a18289ba40a8c263d848e5845419e4c7b6e) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x0698d23dc0409c3671ed675abc106045fbfd44840549c9858983f64398ed0d58) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x073cd51ad5f44f08100c46592d6105e071c6693ab4632a07ffe04b8bf7a10b72) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x17464133ca05f2207b346bcaba6a0364244867b59b36b58da3b2c87e446bd789) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x08608ad3fa754883b57ad2beb1e00d13ea25c6e5b90d6cdb2804aa244bb6579f) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x06eddc2e4a8627236915e8c974710954ac2d9ff3eae24bc3f28f61391842c29c) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x2285906730141888390d1809dd134a79e079f29c10be2c6984fefa51b32920f8) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x053a249dfcef61dcf9140ec7bbfcf1d84538beb151766142e6325ad6a578e73f) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x2b008a8204e7b2bd76be1d6369a782eb56cbc7e3d52868c6b52f0018b0c72668) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x2e936849a17a65e15eda80ccf87245c1d437be34156fcaf0ff0e368630cff8e0) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x0a16be4ae0244564dd09618fbb4e90d400c2693ce98b2ca9ea9fe020901a3b30) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x053bdd9fbc3a4df6788d56314df9fa23280c0f25c2bc59948c04c8c6ca23c335) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x2cc7abe2e40c6648cf0be65db16f3ce1000e1921a813b80c5e2120b2c45b8afe) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x0cd16efeef50166d328f5e9de4085b309123239caff95262f6e8153103f39a4d) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x070bc113c14e22cc07dbe9c67e023a96eae11f3776bb31724fe9aa2b62e9e515) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x1d4f5d853f9ccd4145c2d15a95b1a30c9e85263c443ce097105114127dc72770) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x28235f582fa6cfd3eab96bd329e2a7bad746a9c48447b311d046249904e782bd) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x1a68bf04ff2af2e7bd7eb2a1211d84eeab9626f1f12848f04233aa079568b485) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x1a3c2f950aa30450cd4ed8b4654e42ce5f9a4e51e5949a738b6672e13a213d63) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x182c0b193b8e0fcf34805cfec0846885df6fc0cf93f84e784ad0cd15b518bb09) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x035abd0fecc9372647a55fec3be67f623aa8e7e348d4b072536b3aefb9f5ae2f) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x100aab1c1c391785ebd4729d8d303dcf7418799b6c371ceb196fc2659435819d) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x101a69a00c7a8787eef07528f34995f63693bff44daf5ddac26bfaca840959b2) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x1373014d7f5cc5832a65acd1b7d296f75c627cc47e8328ca218a5cd0d261c29f) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x05dca48aa19130889aa6ba21dc462f3f96162bc5db003e8af9cf8d6c22d73906) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x27b2f27dfc439a25d004ebc5b10cb923857904f3596fa3c84b4de44fa66e7c2a) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x1206f2521f91fe393d24bed8d72bbf5a3403793bdbd4125e4bbda466c5a63545) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x2bc4557c42924ef217d8929ed568403ad7411956cca7cfca772d1a7984511706) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x1ea71964820c28e1b0632cd60e8558924fbb90d61dbb302363f50d7889ab3e4f) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x11580dcb9c6c5a4c27a7cadcc687957302a7e6fd06cdd19f726bcd1025da82b8) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x17d20d4c246cb24ab4fe256a2a3c840e8714e0de8c69eaec0561d9f86383ec8b) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x1896dd7d9e659bee00c1ec7efbc5e42d4edd827f4b3f8f273c4135c9259db91c) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x1bb818dbd5d2fb6446bde292245298ea5dcfddc61d912469b885f80225e76a33) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x1fa0f6b2244d15f0d3f653cb87b6df6ac280556300d8f491b1dc3c47b2ddbcfd) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x178a5782bd623384e6eb173ce247e5137def7a0d231963aa7c6ce43373c94fd9) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x14e313deb8141fe511ccfd86093887b7fa80bfcdf4407e938594cc7657fbf200) // vk.ID4.y
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
