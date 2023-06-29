// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";

import "nouns/NounsToken.sol";
import "nouns/NounsDescriptor.sol";
import "nouns/NounsSeeder.sol";

import "../src/ZKRegistry.sol";
import "../src/INoirVerifier.sol";
import "../src/Poseidon.sol";
import "../src/NounsVoting.sol";


contract NounsVotingDeployScript is Script {

    function setUp() public {}

    function run() public {

        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(deployerPrivateKey);

        vm.startBroadcast(deployerPrivateKey);

        NounsDescriptor nounsDescriptor = new NounsDescriptor();

        NounsSeeder nounsSeeder = new NounsSeeder();
        // Set 0x0 address as proxy address, since we don't need it for our purposes
        // This address is only used for the OpenSea integration
        IProxyRegistry proxyRegistry = IProxyRegistry(address(0x0));

        NounsToken token = new NounsToken(
            deployerAddress, // owner
            deployerAddress, // minter
            nounsDescriptor,
            nounsSeeder,
            proxyRegistry
        );

        ZKRegistry zkRegistry = new ZKRegistry();
        // TODO - Deploy Correct Noir Vote Verifier
        INoirVerifier noirVoteVerifier = new YesManNoirVerifier();
        // TODO - Deploy Correct Noir Tally Verifier
        INoirVerifier noirTallyVerifier = new YesManNoirVerifier();
        PoseidonFactory poseidonFactory = new PoseidonFactory();

        NounsVoting nounsVoting = new NounsVoting(
            token,
            zkRegistry,
            noirVoteVerifier,
            noirTallyVerifier,
            poseidonFactory.poseidon2()
        );

        vm.stopBroadcast();

        console.log("NounsVoting deployed at address: %s", address(nounsVoting));
        console.log("NounsToken deployed at address: %s", address(token));
        console.log("ZKRegistry deployed at address: %s", address(zkRegistry));
    }
}
