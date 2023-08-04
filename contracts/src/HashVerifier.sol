// Verification Key Hash: c61bdb6e49599ab7b263c3af5c7c5898ea317b3e3cc9fab6ea9ed597c7d42ab8
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library HashUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0xc61bdb6e49599ab7b263c3af5c7c5898ea317b3e3cc9fab6ea9ed597c7d42ab8;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000200000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x0000000000000000000000000000000000000000000000000000000000000008) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1ded8980ae2bdd1a4222150e8598fc8c58f50577ca5a5ce3b2c87885fcd0b523) // vk.work_root
            mstore(add(_vk, 0x60), 0x30644cefbebe09202b4ef7f3ff53a4511d70ff06da772cc3785d6b74e0536081) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x274a16cdeb5cfb37930649398741bc4db0cf53eb5204218b2556226aefd3ec62) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x07a55181620a811f1e4bbb7d7c5ab8669fd7360981abfd13fbb975f4ab2767a8) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x138b7046c7d46efa0dac4cf726a68d333d662b3314ef807ee98e233ea2df4c37) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x0562967d7c4d5f7cbba6af45fd5c16bb80e67c87eb290f31ed4c14eb6543ea90) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x1c48cd51ad2f2b715790170527e93a3a19b5c91fc9d51126756aa75593646e22) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x1a971cba26d97b37793848f8527e88b67ac44dac876cf8d77f3773d2ada8ccdc) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x14d5ed7fcc55a35e79f656e9bff4fb8f55165e78ba9dbe588dc1cae5239635f2) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x27c47590ecb2308822724489f1a7ea2facac27f66c96e29fe9c516c4590ab080) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x11e8b75d058b62f320343a8b377c746151f822b3fc4d4f5de33dca5cf95af001) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x28ecb3b6504d6c40c9daac8c324acb42a198eb0fba3df794899b2968425f8eea) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x305e32bdda2f838e91b891389d1dac9932a1d286646a33325d40d67d76a772eb) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x16c48feaa6d5cee96fd2236c1fb6c07f1e66bf24f3465663e90e8f6051ba7a81) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x087897bf66afbdae0752e06f330108a031b22cfbb2106a93e306c6fb79fdfeb1) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x0c99d3ea40beced3bf27e6581f09a8742f407b68d12128c06aa4634e088d8624) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x213f410269688929187f75217f6a9328059b0ae6cd137ae8efa0424d3d67b34e) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x15a375a81be1a96762ff77b287a1223b27feb60e9f1230cf88938bf1f65e1dc6) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x1b01c21b5e68590e37c4529b0d455171dd808ec6dbdb0947e42233f902e552aa) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x2488e0934dab3854fbb692808a16d0caf847e4688e5e6b428d648880d8d5b6c7) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x02f136658067c302b6c1b676f99c0442bca606342d2d18b8c660e401d24da99d) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x2be2638875def1473d6402ea14c4b404920fbed596c5efde4d77c29634533902) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x1fecf4dbeaa8636307d2caa3e5f21a1e4dc850603dbb773fa969081750e4d9c2) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x253f8326e7a878b4189eb8219f1644a8d9e219212c2cf379c9efd99ba0f34270) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x2b4a024d02d40e9ec9e32705e7e7a316878e2347009b14dc2b5fc72ee59c41df) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x0419379c84aaec9baccbfa81cb3510c45730621d9affb49274f2abb166060b1f) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x0ed24dbc2a403f825e77945a445b7537cf3793d1dfd83e1e76f15e598788f4aa) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x00796165c0790a7fc71bec6261bdd6a99469e52359a9d437eca133c2e2f72790) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x1cfd66e51b36a65b1e8d81508a6c95e051cf1198e16ac15910769b26c39e259e) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x0a30b9bd1c93ea2b9838c48a99373b62bb4ac6a81bfcc8861c222ca7263300e1) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x0192e4e66b6c8d139a6c84f217009e8efdda60a4720a9eda9fbe9f68e96f96f2) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x0cdc30e9fe340ec2c8afff58d9179d6d56468f5fb9f73573c0aba5bc48c2c015) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x15260b8c8ee7b1ec06639f6eb31c0aac912eeccd919d9db9e5179ffb460267f8) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x2a30dc67d60dcbd8471adf7a1b545022512fafa91dd2c1ed7719009ea6e35038) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x25fa27dab130619259a575ddd26997684a395962f83129d976eab97ded6226d4) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x0af324acaf4048a1c66f3e13061f48769b1b78cc682991c1cea83964ba243414) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x123f6044c38e39f7ced66b3bf533d1cea25d3b72109664537a8ce698d1f4b546) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x0dc0e4e9a5f7d87514ebc2afb1b6d039d96dcdbe86f2a1a6f683a9118f31b845) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x002da718e2633e731c47f39c4c27498c04d85feddff75ea460c0ce9ef255fc13) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x293e5b00cefe1f927d765488f545bb1538d8a8eedb1919ea5aac5a7022260b4c) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x161648e1d9b6f4417311e123ee2a6ae2fde0d4efb238e354143d1ded59519d0c) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x282bdfb5f6247fbe8dc9dbd87f6bf4086e139d699defd69bd6331bbc151d433c) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x22f3d4ac2592cdce5f471bbf7fea353223c9597288abc0f08b82cd7cc0593b3f) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x1812a831fc52d315467489cb3d7a0534a0d87331b4c1f16702ad68ca434625a9) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x26949036fd0998ad7a17874492cc84e95bac9ca117b17fd3e4aeedd27d76c697) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x0d87b1c63e25d12b35c555280fd515740032f18c07c5d255bdcd4229ea5f32b5) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x0c70898e1ecd2ff68e6a15b1e4fb52e7020fff536c000fe74bd28609154731ac) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x3015c55720391ebbe370f20ef01e26a63a5abb54efaa9e41bd1d0aae9e66d035) // vk.ID4.y
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
