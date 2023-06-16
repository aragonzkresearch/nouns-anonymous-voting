// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "nouns/NounsToken.sol";
import "../src/ZKRegistry.sol";
import "../src/NounsTokenFactory.sol";
import "../src/INoirVerifier.sol";
import "../src/Poseidon.sol";
import "../src/NounsVoting.sol";

contract NounsVotingDeployScript is Script {

    function setUp() public {}

    function run() public {

        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(deployerPrivateKey);

        NounsTokenFactory nounsTokenFactory = new NounsTokenFactory();
        NounsToken token = nounsTokenFactory.deploy(deployerAddress);

        ZKRegistry zkRegistry = new ZKRegistry();
        // TODO - Deploy Noir Vote Verifier
        INoirVerifier noirVoteVerifier = INoirVerifier(address(0));
        // TODO - Deploy Noir Tally Verifier
        INoirVerifier noirTallyVerifier = INoirVerifier(address(0));
        PoseidonFactory poseidonFactory = new PoseidonFactory();

        new NounsVoting(
            token,
            zkRegistry,
            noirVoteVerifier,
            noirTallyVerifier,
            poseidonFactory.poseidon2()
        );

        vm.stopBroadcast();
    }
}
