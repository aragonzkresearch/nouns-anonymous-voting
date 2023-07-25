// Verification Key Hash: fe1e813e84e3feee098de6583dedd9ddce3ea84c6ab83d1b008533ba524918a6
// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 Aztec
pragma solidity >=0.8.4;

import "./BaseUltraVerifier.sol";

library HashUltraVerificationKey {
    function verificationKeyHash() internal pure returns(bytes32) {
        return 0xfe1e813e84e3feee098de6583dedd9ddce3ea84c6ab83d1b008533ba524918a6;
    }

    function loadVerificationKey(uint256 _vk, uint256 _omegaInverseLoc) internal pure {
        assembly {
            mstore(add(_vk, 0x00), 0x0000000000000000000000000000000000000000000000000000000000004000) // vk.circuit_size
            mstore(add(_vk, 0x20), 0x0000000000000000000000000000000000000000000000000000000000000008) // vk.num_inputs
            mstore(add(_vk, 0x40), 0x2d965651cdd9e4811f4e51b80ddca8a8b4a93ee17420aae6adaa01c2617c6e85) // vk.work_root
            mstore(add(_vk, 0x60), 0x30638ce1a7661b6337a964756aa75257c6bf4778d89789ab819ce60c19b04001) // vk.domain_inverse
            mstore(add(_vk, 0x80), 0x19a95214d45a56afa1a123d4ae64cab6125f75c2f5d02594da793885829462cb) // vk.Q1.x
            mstore(add(_vk, 0xa0), 0x0f72949eb0a40baf406ac91d1b7a22c78b2838324e96db8cc1941e1e5dc8476c) // vk.Q1.y
            mstore(add(_vk, 0xc0), 0x2c721a186af5ae039700e0e4db7c57ecdf1bc3765067a37edf834a8755bdecdc) // vk.Q2.x
            mstore(add(_vk, 0xe0), 0x2e81278631624b4bfb1f2e25a378969765b4aee4fcb2d034805f96ceaf98062d) // vk.Q2.y
            mstore(add(_vk, 0x100), 0x1829651e7e5d02a0b23235fab1c6603eabba1edb0fb4ffca892ecf5272e64901) // vk.Q3.x
            mstore(add(_vk, 0x120), 0x0b99f4295665403d1508fbe9ba2b5148d9c981082d9d92a8256e05f106fa6e3b) // vk.Q3.y
            mstore(add(_vk, 0x140), 0x1215918a55bc0f1763ec6fe04e7ba3b518c79d2e9fb1680bdccd128b81a6f820) // vk.Q4.x
            mstore(add(_vk, 0x160), 0x19f307b61867e92b7343867a4e0cd4eecb3bbf0db06e7a08497ea4309251339d) // vk.Q4.y
            mstore(add(_vk, 0x180), 0x18c2ad6a07192f135fd3a4cd94fb91e6097f90e9ea1c4f8f4743f6547e37c71a) // vk.Q_M.x
            mstore(add(_vk, 0x1a0), 0x1ed7587cf8b1039baa690dc832f954b53b1200d320346660d01fc2270f5aaaf7) // vk.Q_M.y
            mstore(add(_vk, 0x1c0), 0x16cde5146495d64b26a19a83c1eac51b1378a0f27470844090b1a6c919eb60ff) // vk.Q_C.x
            mstore(add(_vk, 0x1e0), 0x10e1f3d14a99c2b923646000d05ba3a3cd64fa5e9ee4bbe1e57a462793772b55) // vk.Q_C.y
            mstore(add(_vk, 0x200), 0x12db7bc9bce661accacb3d052151a875c59807bda276d825938c9e5f6cb5c99c) // vk.Q_ARITHMETIC.x
            mstore(add(_vk, 0x220), 0x255a0cc14944e97842964f986930f1feb160b523dc4879ed63dd87f913e01a84) // vk.Q_ARITHMETIC.y
            mstore(add(_vk, 0x240), 0x0aa88f1cc760e8f55f91e2115cbdd5891ec5d98f819cfc28bbfa8a4aa931f351) // vk.QSORT.x
            mstore(add(_vk, 0x260), 0x30200f0a0ca9217cceb70b1c589ec56601109e94a6f75ac68b93f4c754ff96e9) // vk.QSORT.y
            mstore(add(_vk, 0x280), 0x1b6554b6d2aa40d25b6ab07460e9ce2b6b85fb328405a1e1f56ca8176be0b329) // vk.Q_ELLIPTIC.x
            mstore(add(_vk, 0x2a0), 0x29955e8a0cd8c4afa3d13b5ce1be3e7712b58c51d589a9061426b3e4f0e548bc) // vk.Q_ELLIPTIC.y
            mstore(add(_vk, 0x2c0), 0x2c4c5b5ebd83d5248c92a7ef9552c3b45bcd805ed7c28e911beb9634f3ee5843) // vk.Q_AUX.x
            mstore(add(_vk, 0x2e0), 0x1c3e8854a1e23d7f9d2e76f71b85de046c50622c7c3346787f392f89d5c29526) // vk.Q_AUX.y
            mstore(add(_vk, 0x300), 0x06ecc3672ab37c523921cdb1202261c530253517abd061b37a6087420f9594cf) // vk.SIGMA1.x
            mstore(add(_vk, 0x320), 0x175eb8736ef02c47cded545f147672fd9d47294f6cc19f16df32add784c715ae) // vk.SIGMA1.y
            mstore(add(_vk, 0x340), 0x1a198fa6fbed046a12cd1ac905a897d7b03f45a3b46ccf0b5592e6347834ec60) // vk.SIGMA2.x
            mstore(add(_vk, 0x360), 0x298b2062859d282d76566dd0c1bd4599fa956d95c4f7d0df306be5282835105f) // vk.SIGMA2.y
            mstore(add(_vk, 0x380), 0x1abf9b095986006c7bcd2fc91bfb6197130aeb57f3915045e55fe2d9ede31766) // vk.SIGMA3.x
            mstore(add(_vk, 0x3a0), 0x02f5dd76193d38ba4bd33f6d2d4294487f2ff114997e4706cf78381042d43b61) // vk.SIGMA3.y
            mstore(add(_vk, 0x3c0), 0x0dc9e55eed4488c19ee6fb7aec9b85e7b565b95731449405814119565a695629) // vk.SIGMA4.x
            mstore(add(_vk, 0x3e0), 0x09619aa3e4e102fb8fee62912c3419e5039d6480bf27d22802ece0dd4423a6bb) // vk.SIGMA4.y
            mstore(add(_vk, 0x400), 0x2ed64ad6750aba45060308ded3024fe2b68ef2779adde7ec42484d04b4a026e5) // vk.TABLE1.x
            mstore(add(_vk, 0x420), 0x1536b9a11fe50685319e6216fdc57535a63a15a25c961c9f5412ce2b3d8bcdfc) // vk.TABLE1.y
            mstore(add(_vk, 0x440), 0x2d93bcc9e09c48fd3bd30b51e6ebfefc3d87d08b396ac625317b7eca5851bc95) // vk.TABLE2.x
            mstore(add(_vk, 0x460), 0x2a8c594dd2b2510a6484ba8a68720ee760db3b4dc636d7e7c21cbc719a09b32b) // vk.TABLE2.y
            mstore(add(_vk, 0x480), 0x18484b22d5b961fd34caf121451d6b9fe7aa21eb62f1f1f34f3a86b90feb9252) // vk.TABLE3.x
            mstore(add(_vk, 0x4a0), 0x08b3820eea73f1ea978300c76306896e5d0a95c6c68f7a5c2de8e9492eb30bf7) // vk.TABLE3.y
            mstore(add(_vk, 0x4c0), 0x1fcbf10eba042054bbc36002030f6ef53a5838a2bb11ea293357a419e93239a6) // vk.TABLE4.x
            mstore(add(_vk, 0x4e0), 0x24956baf11671877a1974a69059e506e06d154841aaa195a05ae3059a53aa86e) // vk.TABLE4.y
            mstore(add(_vk, 0x500), 0x239cfebd4b5c0ffa5f45a84f0d45a48bb39fd84e2b01b985261ccf20353bc744) // vk.TABLE_TYPE.x
            mstore(add(_vk, 0x520), 0x02768130086ce450622e1bef92f03b5af0639a9aa184177d281bbee559934729) // vk.TABLE_TYPE.y
            mstore(add(_vk, 0x540), 0x1e7ecaa803d3b61faca36355b786149c43dfe3932e016dc5725a360533f97ec1) // vk.ID1.x
            mstore(add(_vk, 0x560), 0x00f05d4cb1967acc9b8091b44b0fb3f37a325c14e3aea14fcbbe83faa81d1d88) // vk.ID1.y
            mstore(add(_vk, 0x580), 0x209f79e9a4d3a0026d2d628bb5e97479cb11421461e4b9bacde34ca31a2a64ad) // vk.ID2.x
            mstore(add(_vk, 0x5a0), 0x030b7e842e9b5997d3511fc6d90804009ca6c5a20e74c466fa81f2f22ed759d2) // vk.ID2.y
            mstore(add(_vk, 0x5c0), 0x19a95a0912fe308818147573f6e0c06490e54f69110ce3e7d58767d0180aad5e) // vk.ID3.x
            mstore(add(_vk, 0x5e0), 0x143908a72c71160b2432c0eda6e6072e3d22ec92ced8012c0f623bb615d73118) // vk.ID3.y
            mstore(add(_vk, 0x600), 0x2fb2086ee7ed8dbb3cf45cd30d927478b34e7e16ae279074388daa90d872fb08) // vk.ID4.x
            mstore(add(_vk, 0x620), 0x2e08cd0fb403098395a03ba65de170da2c96307a3bb3c6ca0615ea16b4dc4e13) // vk.ID4.y
            mstore(add(_vk, 0x640), 0x00) // vk.contains_recursive_proof
            mstore(add(_vk, 0x660), 0) // vk.recursive_proof_public_input_indices
            mstore(add(_vk, 0x680), 0x260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1) // vk.g2_x.X.c1 
            mstore(add(_vk, 0x6a0), 0x0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0) // vk.g2_x.X.c0 
            mstore(add(_vk, 0x6c0), 0x04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4) // vk.g2_x.Y.c1 
            mstore(add(_vk, 0x6e0), 0x22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55) // vk.g2_x.Y.c0 
            mstore(_omegaInverseLoc, 0x281c036f06e7e9e911680d42558e6e8cf40976b0677771c0f8eee934641c8410) // vk.work_root_inverse
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
