// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/NounsTokenFactory.sol";
import "nouns/NounsToken.sol";
import "nouns/NounsDescriptor.sol";
import "nouns/NounsSeeder.sol";

contract NounsTokenDeployScript is Script {

    function setUp() public {}

    function run() public {

        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(deployerPrivateKey);

        vm.startBroadcast(deployerPrivateKey);

        NounsTokenFactory nounsTokenFactory = new NounsTokenFactory();
        nounsTokenFactory.deploy(deployerAddress);

        vm.stopBroadcast();
    }
}
