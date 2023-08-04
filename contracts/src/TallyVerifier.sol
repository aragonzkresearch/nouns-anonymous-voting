// Verification Key Hash: eb86b4b7203fd73cb14249d5123dbfcbf14ea599d0f8108925a372203da3a38a
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library TallyUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0xeb86b4b7203fd73cb14249d5123dbfcbf14ea599d0f8108925a372203da3a38a;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000020000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x0000000000000000000000000000000000000000000000000000000000000008) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1bf82deba7d74902c3708cc6e70e61f30512eca95655210e276e5858ce8f58e5) // vk.work_root
            mstore(add(_vk, 0x60), 0x30643640b9f82f90e83b698e5ea6179c7c05542e859533b48b9953a2f5360801) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x1cf1be065b55e616674b59189e2eeffe9d25ad6601d36110570e81abf934cfa8) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x10013d7bbdaef9dc0ecdb649bcf18a2385ce67e590bf1b8f6b579be55ca94fdf) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x0c187392eed1b3629ccd8051069c4141d670f2adcc40e463791c953c076af6c7) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x0fab9e8000884ec049165595d743a990e0b510a7c1271ff5898bced08c08146d) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x0a18c48fabc41c0c7d6262242047d4bbca557a7c3f635959df5e6f3fe70246e9) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x13d0e132ef93efb87569123c070261eb0c2fadc3735b1ab0566918eafcaabb56) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x03a5a35bb83f772ad1c443959c0646253ed7eb8943d3f2e0dfc15b97d6db7739) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x1dbfad4d9e2133692f7440ee7e990892c0416fa8e6524629cad2d7f886d91f76) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x2a459d22f9d9450c5bc5858b71ae1802c95bf265a2b6e97131d19e0d9d606d2b) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x0f55ac375e4882e7c140bc05a85fb4dfca13eb1b0b7d5d83b898e2f677c39a25) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x083fa82cb6a665e2bbb3bf130344e01e096c53b6ce36433099a01bd1520e51b1) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x1a22aabcfdf86ca05c7424bc730dc3acb878bb05fcf364ee64f3fdf7e2c02154) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x3026415681c7f17f800913a98da798de715f53e6caa58f02206dd3e3c51d8960) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x047f1729b4bc18309127eebbed8b5dc8e8a6e7a3d23dbc49f58d18e1b035a02c) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x219f0fc7fe54c6786239303c722671ee6381e9d5c4416992565223f0a3f6fdcf) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x12103f7edd56251fa3e385fed01110dfea6c5b8859a9eed2fcfb4b1ee5f32f1d) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x28b955aac4c043cbcfde06eee59b44363f96fe0a6fa93b0b36ef07fbe285d4c7) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x16e1bbb0a7728dcb7696fdf3361b39510c6a4f83cc2159f5fc65ceb2be0599f9) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x1414af76247139fa9e8fef8b393a3e03227ee3a6fedb1e55f5db82cb2352782a) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x2c7895a68d2fab5b2bce4d7703daebf9011e63d675bc6898c7f06087d6d83d99) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x008364146d59d00408e8031f4744bd081dff578c1dcecbf82b1865fa4aeadc73) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x1e417706114b191f475459f1a88ec40eaed5f81c3153caeb20cfd56c48753297) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x1e6e8bbc6ee885204818e9f81336ee6d7ae8e664b38cc9ca7616ff54933d9c3c) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x1a798cc683ff77a3845ef89b1de39f02661a7675d493bde38da479518be55731) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x1e1e05e2617566e796b14be3dda22ae7d5641999499391c63ee5e0beb7660c0f) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x2e1e5a38ee9d457757ce8479b61b746dc03126d291b5718ca587bafaa212d05a) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x2a01d852d1e149fac1b500cce118e34231c87101df660422adc200d811226ea4) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x10930f516ee1cd2bb22bff9215e6f042c7fd772df9eb6a8cf80d1e3de40ecce5) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x28faa42b5c13a5e9927d13e54a2ed806854cd23c6662b320439aa3168beffe03) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x1e5c18afa66b4c0d19473e0536e64f678c1b094d1b2eff1d7f499dc289efc084) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x10a001251e9f3a9f283ff8f6bd14cba9c706f3c5040ec8ef10ff44988441251c) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x12138fab93fce066ddb2f9be4eff97b0fe128a2a2c079f749b8452698cace8bf) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x133738f359ce2e0f909a0b76a78c602e66e39c41d99f65bfea25f47998283ccd) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x2dd03593caea05125f520c0d02a3bbbdc5519822ba0e0b00984c5a9281143a81) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x09ec9b0aca4e9671903e0577f2a4efd36f7a58af0a5102f5a42e1b8061f62421) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x15affeadf66c8428f4f44d2ebe66e9dc0f04215bef81efbbee166d3f4544feab) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x1fd912d00da77afb70848e4442324157606f77f54ebc05d1a1a5fc2030836b13) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x1b1c85cc22723d352d37c86cfd66d45e809a813b99a452fe452c7ae975de2286) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x2efbd97bf4d8824a3a075a18d6e549e7a8ffdb1ead7da00d438d05bc57077d95) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x0823aa7e26174aedbc9ed1091d2d0443dca7304ab8c1c539abda10d1497f7df8) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x270a4603ab3289ff23d5da4cfbaf3f7ffbc7f6cc7863bfbefcdab699d649e096) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x2a946cf23ea4282ac753d956caea8eae05fb5418065b91578697d18359b29602) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x17b985e6b1468aa77e61730d70201b4a661e39b34a15dfb50c9d81bdb775baf9) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x1e0f44eabfc7a4e8407f11362850cddc7b81c210064cb786d3dfcf3724f4eb63) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x2e1e6c33b7cceea1c1b367dab722bb98fb26c35da5d985c9b52d1fa059a9c5e5) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x17fac67cfe74edc1c9a00f9e4f71f35bb61faa6a75ddc221faaabe1a68b00b6a) // vk.ID4.y
            mstore(add(_vk, 0x640), 0x00) // vk.contains_recursive_proof
            mstore(add(_vk, 0x660), 0) // vk.recursive_proof_public_input_indices
            mstore(add(_vk, 0x680), 0x260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1) // vk.g2_x.X.c1 
            mstore(add(_vk, 0x6a0), 0x0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0) // vk.g2_x.X.c0 
            mstore(add(_vk, 0x6c0), 0x04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4) // vk.g2_x.Y.c1 
            mstore(add(_vk, 0x6e0), 0x22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55) // vk.g2_x.Y.c0 
            mstore(_omegaInverseLoc, 0x244cf010c43ca87237d8b00bf9dd50c4c01c7f086bd4e8c920e75251d96f0d22) // vk.work_root_inverse
        }
    }
}

contract TallyVerifier is BaseUltraVerifier {
    function getVerificationKeyHash() public pure override(BaseUltraVerifier) returns (bytes32) {
        return TallyUltraVerificationKey.verificationKeyHash();
    }

    function loadVerificationKey(uint256 vk, uint256 _omegaInverseLoc) internal pure virtual override(BaseUltraVerifier) {
        TallyUltraVerificationKey.loadVerificationKey(vk, _omegaInverseLoc);
    }
}
