// Verification Key Hash: 9a417869dc6c7e396ce49dfb8d3374590f41ceddc036200b6303f70dd2b642f8
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library TallyUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0x9a417869dc6c7e396ce49dfb8d3374590f41ceddc036200b6303f70dd2b642f8;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000020000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x0000000000000000000000000000000000000000000000000000000000000008) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1bf82deba7d74902c3708cc6e70e61f30512eca95655210e276e5858ce8f58e5) // vk.work_root
            mstore(add(_vk, 0x60), 0x30643640b9f82f90e83b698e5ea6179c7c05542e859533b48b9953a2f5360801) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x0a228a4c0b32df2dffb59540a26a0d0fa547abc95d7069d4d7a950adf4e9e0d9) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x28c350e40b08dd6af12711d6b9250807b1d6d93ac38374719a1c5b391eaa25b0) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x29af35a97ab5c149fc29821447ae3f8b0fc250fcb8029c8a33cdeacba94f3398) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x21e9f4e41131521f3b6f0f04898ca57076a0549749171708a8788b79516209d3) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x0030a503a74b222c09e6201f30326331b88813453944f7ba745f6403a76b9b5c) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x1d4fd7e4da6f504c2ee48f64b24976fb008ba2f7cb4533aed7a53fb2516b6b28) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x0645813e6b58f22d9b1d80a6e0b6150dc929ac8325629f39a5551807f7e5e3e0) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x2e974c99acd0acccdbc535f0c8917d5f3bf9022b1e72767180aa75ba95034f21) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x0cbbd74ebdc57f0acce6035b80fbbf8d0d2c2b1921e1a32b914f66b7157f7078) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x16bf946ae9992d7ed55447444fdc2c0b7266d2d83d70449ef5e8966d1d347a10) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x0abc369e49f10888e64525256fab96541bec9cc10212d698d275c3dc08eb8431) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x0a3c083390f13acfdfce7198c9075a364848b4e6a1f301c546768c7d5d722702) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x2248245cfb38f7dc3c72d7959b1e31a096d96d8691a5a00828b14c74a38d5233) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x2ddc775f14b7e5c60e27e8d739f9792490c6f94f711f2044329f5f505b6d1676) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x0c74b066d380524f19da1d124fcdfd9227509a88452fbc974cd479648f867b81) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x075a776221222e5d83190fe2ae0f50e97065adeeee412723305ec22712870433) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x28b955aac4c043cbcfde06eee59b44363f96fe0a6fa93b0b36ef07fbe285d4c7) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x16e1bbb0a7728dcb7696fdf3361b39510c6a4f83cc2159f5fc65ceb2be0599f9) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x1414af76247139fa9e8fef8b393a3e03227ee3a6fedb1e55f5db82cb2352782a) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x2c7895a68d2fab5b2bce4d7703daebf9011e63d675bc6898c7f06087d6d83d99) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x0bea9ae6e61be097fe6585861997de385d297bcfd15e2683015af15911a9e60a) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x0cf5ac46ade1e87125719468c0cad09ae1b78103f4ddc08352d5343d4ecd099e) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x20b289445081020b7656cb3562c836f302664e93fb54bdde509e69f8f7651126) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x12556d4524015880374648dc0b4787d55c249e6132465df180394be956a0a1a6) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x1fbad6d3ed9ece6e210135c3b5bafec8fcf6a91a4a333367cc1dfd51bb2a2a36) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x184d7c701a6adc389c9da63820f690d765ba941b5a86eddc920504c9edbe63df) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x17e712a2cb34d04073441b5cff1e98b84bc4a9b9d20a455c49e851359150d706) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x10b9607524a41f6ca04210a2cb396533e1223fb02677fbe0c19b1decf99c58b7) // vk.SIGMA4.y
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
            mstore(add(_vk, 0x540), 0x04b0bdf8c116d3bf19b408a7c7e06bd32d1d862e145c7eb6d9f0f4f99b8e1476) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x274aa5721ec9d9bda75bc7817b3f0e0bc8c8bbc2fc186c99f324dd97f65a8226) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x11ddc84a69e0f4633d1cdaa9cfd1f2e9ba3962d7b96496dfdef10ce7d6c62d10) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x02c7fa7eaac23f6621955feaa8adfb86412d49b11416a6190a822b12e7a6bc31) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x05a699577acdd3df455806f7e223ea02023adc0a2689241413b02fbcf88ddb8a) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x2f096c6eda37cf002d4af046ca89da86edd5d671041f87805cbdc1e7b65f73ba) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x2cc06bf713c5aaa4d6aa4af3f7b315d10931098c24d728ef3616e807d35faa1b) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x18bf2c7d16188791b7245a9269daebfc53fa0ca1727874736438ed0c2e91720f) // vk.ID4.y
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
