// Verification Key Hash: c0b8edb889f5f468348842a55f10c45fcbca3e8febcc0f97bdf01f08f8bcaf64
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library HashUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0xc0b8edb889f5f468348842a55f10c45fcbca3e8febcc0f97bdf01f08f8bcaf64;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000200000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x000000000000000000000000000000000000000000000000000000000000000a) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1ded8980ae2bdd1a4222150e8598fc8c58f50577ca5a5ce3b2c87885fcd0b523) // vk.work_root
            mstore(add(_vk, 0x60), 0x30644cefbebe09202b4ef7f3ff53a4511d70ff06da772cc3785d6b74e0536081) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x25684ac194e53a6c250035bfd3cba5b937c0ddd127c3097e62300c74740ccc94) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x068b1535ae5806e7be913cfc21164c3cd6d27ff5ef679238b4d348cbd42e4e28) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x13f8e308f04cae05da90560653447daa1c5beeae5ce680c4d8bfeb5400c5ec8b) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x24e786f78bba28a2926406bf78ac69d8a9265b429b2e5275e790e5efb91951df) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x2541e68d00fc042e823d80c55c1a06cb9122e726d8db02ac909c5b9d7896fbd5) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x164c0fe5aed2ffd9246d551b5922b34ae955e72fde5b8e4154942b2b132f3a0c) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x00b44a2d9f230989b5dc2a2426706719f03a257974a8ee612528c020d17df74b) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x1dde3c93c30fa506c35d46cd06f7fc60c3875b93df6c644bf2e6c92ae52e6bac) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x0005b432ebe3de796e745e21559b65c879f497fa8c1756296ced6dbe6d0737f9) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x069d165a505ef93581837659e29c06e5413c94eee4baca631ed88243a8eafb4f) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x0388bba41153403eabdfacc9245655561d9d867cbbca9ae8fb2953d1b0c8d43d) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x21110a2ff4e67c6a242d9f3c69f4ec21590edde23f09656773f29dc7790a5959) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x18c81249d1ac23172313637264a7ac55e7d8dda3283b605eba8c27ff1916f27c) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x170f1032fc00f47b13b464606247c650a2307ca7bf49939507cbe47d27dce461) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x0e354e5d8326cf28ae1cb5d44d8544db87631e24b60c12f1762cda5f26364f80) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x1fe31083a4f09384b2b92304c0c4813956b120b0d8867f38dac0d7f538b1a6a4) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x1b01c21b5e68590e37c4529b0d455171dd808ec6dbdb0947e42233f902e552aa) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x2488e0934dab3854fbb692808a16d0caf847e4688e5e6b428d648880d8d5b6c7) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x00012324958a115c9e143209b8d924650276ab4c3467cff1744ecbba76b5f8f8) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x0d6be61d7f0bdb561fc719d8105c2517f9e55481476a740a32b8bf88151e0674) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x1c989a562e5ae14fe09ff11aa6d5d8f0a9c2a919e8192a1d2e845746f02a96ae) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x1725d1d3790b03292a764e262e977c822a94a219fbf21230cbf5205fa984e9b2) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x16f96a7c02da42ea50ed98c6fc01af1e769eca695d23bbadbe1c059d3464db0d) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x1ab71ee5014c18f227cc7567773faff94a6437b5a6d80b30de6639944ecfa727) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x027b34866c89f863339e2b999b3822af3924b730bbda436ff83c962525b57c94) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x133b2c4f3841f4b60a914fd6e84fb910d12877a2b9e341b96233f3562434c563) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x126af37d146e111db6d30819296ffb1f81c742f9ba1418ad618d5479d723c3ce) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x0395c0ae0b6ea982effb5fbeb03aad2c76c559b19b2ac20e1a3c9a57485468c1) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x0192e4e66b6c8d139a6c84f217009e8efdda60a4720a9eda9fbe9f68e96f96f2) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x0cdc30e9fe340ec2c8afff58d9179d6d56468f5fb9f73573c0aba5bc48c2c015) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x15260b8c8ee7b1ec06639f6eb31c0aac912eeccd919d9db9e5179ffb460267f8) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x2a30dc67d60dcbd8471adf7a1b545022512fafa91dd2c1ed7719009ea6e35038) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x25fa27dab130619259a575ddd26997684a395962f83129d976eab97ded6226d4) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x0af324acaf4048a1c66f3e13061f48769b1b78cc682991c1cea83964ba243414) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x123f6044c38e39f7ced66b3bf533d1cea25d3b72109664537a8ce698d1f4b546) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x0dc0e4e9a5f7d87514ebc2afb1b6d039d96dcdbe86f2a1a6f683a9118f31b845) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x2235aea8cdb6be6819922d6e308857db74e499e49ec0f29ea5a7feac43f8f3dc) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x1568000858f3677c25b879d72e98cb368e01475d07f14048799f2bd8fd627ee4) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x2b1771845029124845b0eef051ebd3033ea2be12f517cbce770bd9d917e29798) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x17b84def3e4d3f3b795d2800990cc3b67c775deaf9dd28e7cad12e12674f4ef1) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x1ef56c1f8d2cbd0785303ed008b9873848715ae9c0ba962657b0a0b45b4fa4ca) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x0e9de5dedf9ff6298d6ca64f1f8e71fdb596cbf7b62a08d2bb19867aed12f814) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x0cde77125de34c699344f9467b6d97c61575d2837e111a242cb2948f74858ce0) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x26efe440a786ea7ea79fe8499754d082dee0a2f471a1ad0220443810d23fa071) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x1273e160f90427ec679ae03837a457803feb5c034d2ba771b4d63de31125c6c4) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x1756b9965f58b1f7d04bf24f0f8575e6633a869207b5d5bab6d677fc414f925d) // vk.ID4.y
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
