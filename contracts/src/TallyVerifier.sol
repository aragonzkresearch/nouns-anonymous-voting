// Verification Key Hash: 6c34c8899f1956fd727d92de712a7d10814c297866e7594714934eda59a8cf37
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library TallyUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0x6c34c8899f1956fd727d92de712a7d10814c297866e7594714934eda59a8cf37;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000200000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x0000000000000000000000000000000000000000000000000000000000000008) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1ded8980ae2bdd1a4222150e8598fc8c58f50577ca5a5ce3b2c87885fcd0b523) // vk.work_root
            mstore(add(_vk, 0x60), 0x30644cefbebe09202b4ef7f3ff53a4511d70ff06da772cc3785d6b74e0536081) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x191142993f7c9a99a698f61e1829d88cd32230c2d9a5961bc1c711eea343fa08) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x13eddab07392c0a9e545b866bd40b7fa25ca865761fe952d9d9f076ccb6af25d) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x1d6bc83c2fc691dd45d94f4f76f9a3124b23f493efb2207f4d7ef6046848ea64) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x26b57b93dfef4595320f49311827c705745b28fdaaac364e12c9dc527e52809b) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x1c705e7252101f8147ac6513d249fa44eba7643b5e1e2907f443b04c5bf52657) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x0b034c55bcaf8523ce1d39c86c15efb9a828358369ed9603734d30a2b85301f4) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x0ead9baaf6bec434eaf3222c542cbaa99978115ac2f50111194b2a474d548aa2) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x17065ac42e0d1e79116ba292cd1975b1f009107a55be53ca18334223b0560e82) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x2b9f3aa46e224396039cd51240982aa5fb361721b3ac86237920df8d285f21ee) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x18892d7ed818ddb931e56872e95cf02a55922a88f13f9adb180aa47cdd9aa6f5) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x29a35956f528ab6757e40454238866e02d2698c00a7985dfdff49ec99d3e094c) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x2211adb4c01a1c358fc5f6c8f75ba8ad6b9a8c14e5950ed7269dd817b66f5693) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x060c523264152ae892755413c6cfd19b399d5f67ebb1d6d2b1b72137ef2faabd) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x0a059c82b15ec642db37901a7181dc6fac1a3890faf625bb7c8abbc94171d021) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x0f7dfa992afb04fc5e727ca8370adbf110db77691049a642347b922bc1545120) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x1edfd355b17838ef6e1e29642b2828c716984c6997485d931dc96f85521d8866) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x1b01c21b5e68590e37c4529b0d455171dd808ec6dbdb0947e42233f902e552aa) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x2488e0934dab3854fbb692808a16d0caf847e4688e5e6b428d648880d8d5b6c7) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x02f136658067c302b6c1b676f99c0442bca606342d2d18b8c660e401d24da99d) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x2be2638875def1473d6402ea14c4b404920fbed596c5efde4d77c29634533902) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x09ed9d3ee090c26e398e2829505080a15ce0f94a453e0eb3c9d046d10fa82b68) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x0edc5b8d770a3b6c1647c59f8d6001f27f9639ee3e4debeab9e8fb442663b72d) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x06d26e0a1af21c58039d1521862f705f8665410d504c26e8b79f363b6f424d55) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x2092ddb3a900ca7d17e6de8aec6a4777f175ab7e26b41783a1ac6710c98eef1f) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x2b8a558f38cbba920ba539fff3437efe6fb4e1b2c19e8bd93bac932c70e88155) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x0bdacf20c479293ced0ea65ff1df4c8036203e0fd61467e3a1b3c652fdd26465) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x21bb03f19119b597c2480efc505b72ce03e78241f8092d8cb6294275af3f97a0) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x020528143594cd13eac0b8ecdfedd18fdcf9951333059bfc4e78886e692a47e4) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x05479a425e2584d3e92bf613e1e1981bb6958b0a71d5fc150c028edcca928768) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x0d5c6be83dc7204d94402e954bdc87db165bb783a1dd93f693840528ca28edfe) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x007ca369d5db55667822dac70c8eaa28b766efa91a4951baa973629c5496599c) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x203f49c2f5049d108b2d50348269d3ebe855073b85d64e163680c7a6dfcc99ca) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x1851780f9c3919a1f0d9f7ced3f1be19d33f99c2a67e57bb749a21f7605393b3) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x28d33495f84d57f7df3620f4c61216608c6bea403f1bd8760e84df35a5a93f1a) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x0cb783bac7d576ee9430961c03835ec2b7f1e7713a14912ceacedcf4b4e22624) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x288593094a6022d273994282ca268aa967d6b8bd68415da11d6af41669f2ffab) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x06ca78377fe73e9c6ed6da6d86223509e304d689f80a23c9387c8441eadc41f3) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x13b90adaaa0d6c5849e49ca68a13a1ebe47cbc36cc246b62124f3883eb5f69d1) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x00421152c4dc845b2dfdd436dbd8bdfd34d422d9724388023958b8f14780c4d0) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x2d6e263cf50987aa9f4607deb65978a1f2f0f90bc80ac1d67293b39078a3434b) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x1f439e11d9770ee6bbbf27fb37bb29fb7bec822b03a30cb0e870b4534f13ea65) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x1946a9575d8646574be6dd676e6a6dd584b7f8934399d6230b2a3df6cf4826d3) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x09fccf707e6d8c6a335c9c99c48b8a4730bdbc9ac1613772ca35236a31900a4d) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x27798d2f908c64c9354ec2eb3c00ecab38c44b5e3a70347d239b6a6f5ee1005a) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x037ac9260bd3491db676c404111bf938e1475bf7732c47c86960752c19dfd4ab) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x28ea3e5fe4be4bdfd711791101c50877a198038d9e6b22c630276e3aa0e619cd) // vk.ID4.y
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

contract Verifier is BaseUltraVerifier {
    function getVerificationKeyHash() public pure override(BaseUltraVerifier) returns (bytes32) {
        return TallyUltraVerificationKey.verificationKeyHash();
    }

    function loadVerificationKey(uint256 vk, uint256 _omegaInverseLoc) internal pure virtual override(BaseUltraVerifier) {
        TallyUltraVerificationKey.loadVerificationKey(vk, _omegaInverseLoc);
    }
}
