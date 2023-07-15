// Verification Key Hash: 98b50224b4b49348686f71e62d8e92fbe0e1b4dcf6807462020e62dea1212160
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library VoteUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0x98b50224b4b49348686f71e62d8e92fbe0e1b4dcf6807462020e62dea1212160;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000200000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x000000000000000000000000000000000000000000000000000000000000000f) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x1ded8980ae2bdd1a4222150e8598fc8c58f50577ca5a5ce3b2c87885fcd0b523) // vk.work_root
            mstore(add(_vk, 0x60), 0x30644cefbebe09202b4ef7f3ff53a4511d70ff06da772cc3785d6b74e0536081) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x090845bde27c8fdd9e58b2a7d892901f545b850e19ae13dcc277569b9684df98) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x29cb7cae0ddcd41d308546bf0d725013339ef189d4da3e6dfb8b86defdcf6002) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x0f7cec4cab4655b6fc88a9ccbb740382d109e1e2655296779f42a3c098d1f5ff) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x2f73ec138dba0529959d881e35d64831d1b8e6a2754f967a42a32a60bc304d83) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x1cff2efd0d87e65cc4defed040c140c9a98558637ca823eccec64e65c9bf55f8) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x1286e47b08ae90096fd4958bc4a726673084673d123a6b23e55ef46eecc704de) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x1361bfd8097e2ff268ba5ed61e4951673defbe329653c5db5cccda2132f9f348) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x135476878616365992115ffb916e7562a6c8fc62170b5cc9ce8759eab00265b1) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x059ebbef2b299da30242196ab952c11d1b62e95bd3f4e2dbee0137838b01f0d8) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x1776df781478b1d30e87473fe9289d344b95fa016d6971a73b9f903f8f26359b) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x1fe7fb4d55e63cf1301bc2e15230f72c8c870dc0b2d3b3a8787abbfd921aad50) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x0e16675b1fdf66f63499c7ee22c0a30e7970e5c9bf2fbe969e6824ff0042b99e) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x2c6a9db6e5237e1697dbe46b04f003592e5c262f43ad1cc676e802738f77c798) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x13ba30da3479d067eb7eca7ab7beac274f1fcdbdea6186390a115f2a6832d7b3) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x122736b44b9200bb719efe2898cbbc535ad575f7d385408c23d105892ca64ea4) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x18fdd25f6c513fb82d8c0751934a2dc755ecf23982c048df704845d853b9ffc9) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x1b01c21b5e68590e37c4529b0d455171dd808ec6dbdb0947e42233f902e552aa) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x2488e0934dab3854fbb692808a16d0caf847e4688e5e6b428d648880d8d5b6c7) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x1db5f0d542e05560a271123da730ce06d0b08107d9b6d912616ece42916f46c4) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x068980bf361f0d0385ee65105a2f80c55f63108d0a9f2f955b907a04acd7fb6e) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x0a9ad04077bd2194e6870107585a49df96b64012fa3795c52c3d00ac8077b0df) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x071281bed3caf41a4145c09c4f6e71cfb4c8048656e7ccd1890edf3411e9edb9) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x0bcc72214d8e1cc07642e23e932f10b3a433e0307ca4128aefc7975f3488f22e) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x23f57ed20f26f4fbc3cc55726d34bbb76e63b101e6501d891d08a2d9a4cbed42) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x0032876f718b895f62da764d256b18047d3245f960696ed8e5f61712fba93d22) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x00c8508c6ef9556ae94ca38f9f46c4375eb5f598a3f149fd509d6b7f8fb05266) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x186ac335c1045ea27f1863f4cb0fcb5296596d85350692f336775cf399c7f2ac) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x1878cc0df7ee848d2e964fb0f151481a789b5050922f35a418cdbcb9387c2529) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x0192e4e66b6c8d139a6c84f217009e8efdda60a4720a9eda9fbe9f68e96f96f2) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x0cdc30e9fe340ec2c8afff58d9179d6d56468f5fb9f73573c0aba5bc48c2c015) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x15260b8c8ee7b1ec06639f6eb31c0aac912eeccd919d9db9e5179ffb460267f8) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x2a30dc67d60dcbd8471adf7a1b545022512fafa91dd2c1ed7719009ea6e35038) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x25fa27dab130619259a575ddd26997684a395962f83129d976eab97ded6226d4) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x0af324acaf4048a1c66f3e13061f48769b1b78cc682991c1cea83964ba243414) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x123f6044c38e39f7ced66b3bf533d1cea25d3b72109664537a8ce698d1f4b546) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x0dc0e4e9a5f7d87514ebc2afb1b6d039d96dcdbe86f2a1a6f683a9118f31b845) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x1e73fd509b30b075dca363f6fc5faf6755bfca44b29135e43a5393597443cd17) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x24e1fb9acd28aa639fc8f55f465cee950dac3ea3ba68b1827dc2c533d59f7d6d) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x24d379c9ee9a594965315dd5acfa3af811a9a904d93e037460409e3b88f9bcff) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x0fc07e3308f9129f272e12d5674c42ebeb4c9890b670ad18e5ed85aea97989ca) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x138148a1fe067347649a7493565fb11af14b56a2d3030e2e4dca686e01c19401) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x074d58c28dcc333a98443424418dd5fdf992aaad73f976c83ef07549c76eb8dd) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x287b8e3786100533a82b0f406da077e3737ee9d3c9f2ead6b64e15f4f14e4cfb) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x0e6f594b529a5d28da5cdc821ef86f0c54fdd101f5fba591fcf639d275c5f58d) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x013b420a43fd8847ee80360a84457c52f475b04e9b37792e709f653c5fc50b25) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x1279219b231cd029d44a36b9426aecca4cf8816f73170aa8194a2ebe06116eaf) // vk.ID4.y
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

contract VoteVerifier is BaseUltraVerifier {
    function getVerificationKeyHash() public pure override(BaseUltraVerifier) returns (bytes32) {
        return VoteUltraVerificationKey.verificationKeyHash();
    }

    function loadVerificationKey(uint256 vk, uint256 _omegaInverseLoc) internal pure virtual override(BaseUltraVerifier) {
        VoteUltraVerificationKey.loadVerificationKey(vk, _omegaInverseLoc);
    }
}
