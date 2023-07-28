// Verification Key Hash: 51d9df1f19fc2f00e37c4217d6fc634e6a6d9c2e9d37d9827a49e142caedecc0
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library HashUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0x51d9df1f19fc2f00e37c4217d6fc634e6a6d9c2e9d37d9827a49e142caedecc0;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000200000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x0000000000000000000000000000000000000000000000000000000000000008) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1ded8980ae2bdd1a4222150e8598fc8c58f50577ca5a5ce3b2c87885fcd0b523) // vk.work_root
            mstore(add(_vk, 0x60), 0x30644cefbebe09202b4ef7f3ff53a4511d70ff06da772cc3785d6b74e0536081) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x16d85d6f5992a32bcdaa6ed71b1d9b97fffc351e7cca967a28ea53493ee38ccd) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x257165263e7bca5939e5960b6029fb92a1878aae8b865731860b4453ce2443d2) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x06e9475b5b8f40493bccc2ffc395bcaabdd496b1f50036ec8e96d18fed43d126) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x0a04e180d8f9dcea0e38f76b9090b6d1e18218c03acb2b1359bad2f8641c5299) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x07d828ceec659f9b8021b15322793a141c3dacb6318f9afdd1c4f388e6323cd0) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x05ddf609dbaa27f824ebd34c2a2e83cb82d52200dee5f12769bf9caa47ff8490) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x07724b16f37dc72f6595c1afd065b61cf219b2c82d262978d37e0d923cfab3c0) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x088899f7cae473da7455406bd44b6b50c3884246112ad54b080e241e5fb8e17f) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x11b009d4d65e886bead979fe56a4438073a957dd2e008637014b2941baaf5d82) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x301950745779a51de076915ba3c181492cf2ea1d8552fc76c38b686750451de5) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x090b2e5abc0a8715b9b3064c1cd89af4e284a7cedac1b71748e0419d70c8319f) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x0746e58288ec90e683197fa31bc50e4b6f371f48c0207ba726c015105f1321eb) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x033c44858482fefea91fcfd4db2e5f840ec75757e4802a51c43572bf928cf49d) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x1a58a26a17ff0d9ee31b5367a821b91b105c94d081e1edf503bdc159a5106a11) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x21cf545d25230e855e51ca3bd5c81da28f0e9da6e0f17aa24c4fd9e3bc61ca57) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x20579700ebb35cf2beecd80a77a3c99976378581eab1175161791bf3e9974acb) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x1b01c21b5e68590e37c4529b0d455171dd808ec6dbdb0947e42233f902e552aa) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x2488e0934dab3854fbb692808a16d0caf847e4688e5e6b428d648880d8d5b6c7) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x0d1214d25beaa575e65f24e803f3d322716a5853039e7689b460fdc5f185553a) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x0f689436c4bd28a55a7f23720b92bff038c9c99cb352a23dc7b441ef5f9f32c9) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x29548cc2fe4bd22514993f6652e4c353164572664522981d85382b0bfcd7cd14) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x280318bc9ed31b224fdd4c9f7779a05eeb06ba580973213a0d3835f4217a2994) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x0d1014f30b04ce79f626d01908369b773baccff442b42d73535004aa518166ce) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x1b0a963cbb04424c21934727e1e269327b2180e4425feadeb232bb13a4aee149) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x027179c9d50a8642b524aa843815950e46ebfb92551a1313ed346d79829bb6f9) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x069427a9b156b3342a703b987225e020cd7d9af89a46a122e961535aab286cf0) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x2016f297c266abf98262f7e9eb9d7b03dea07f56983e008284cf454c5971c794) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x0f915ca61cc3521000b17c02ba4de692e2b75de7eef1ade844192925110a2364) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x0192e4e66b6c8d139a6c84f217009e8efdda60a4720a9eda9fbe9f68e96f96f2) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x0cdc30e9fe340ec2c8afff58d9179d6d56468f5fb9f73573c0aba5bc48c2c015) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x15260b8c8ee7b1ec06639f6eb31c0aac912eeccd919d9db9e5179ffb460267f8) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x2a30dc67d60dcbd8471adf7a1b545022512fafa91dd2c1ed7719009ea6e35038) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x25fa27dab130619259a575ddd26997684a395962f83129d976eab97ded6226d4) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x0af324acaf4048a1c66f3e13061f48769b1b78cc682991c1cea83964ba243414) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x123f6044c38e39f7ced66b3bf533d1cea25d3b72109664537a8ce698d1f4b546) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x0dc0e4e9a5f7d87514ebc2afb1b6d039d96dcdbe86f2a1a6f683a9118f31b845) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x23ccc0894a2809222410d2c09207bdb7a39ee1493b34d6e131159185c5abaec7) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x08af0a122d032f055a3e8aba10a2dae27b8a1e04184667cef0829fc630e56da9) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x1d8ed3455e0ca394dab5c5dc64ccca1b0d3dc5ca881e74ec2294bd344124e4fc) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x04ea83b69f9cbc0b4d4efabd07355e45688a974fe5acb59ce6fc98b2d3a2ce37) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x1f2e4c3681bf581906df17fe8ec639ca4756f4f9d4dcdd1dec32d56b57748a5d) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x23dbf06d8bf2d293642a6935b31d02bed4675c5361cbeecd8258b93e84539101) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x2deffe29f09f5bcd5d5a21a5cd743412ddd448d8f7210eba677cde4e4bda89c9) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x29de9db229b0f2b6ee72ab967075b20df8468e67bed2ec02a448a6272ec4514b) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x05b8706c4ecaee78aa3b167950a756ecaebe85426a791c65551cf2463170e141) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x16f6f7cb0dc3111ea461b97702ecb339417a3af73a6fd173f5c8908ea0922892) // vk.ID4.y
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
