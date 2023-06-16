// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

interface INoirVerifier {
    /// @dev Verifies a Noir proof
    function verify(bytes calldata _proof, bytes32[] calldata _publicInputs) external view returns (bool);
}
