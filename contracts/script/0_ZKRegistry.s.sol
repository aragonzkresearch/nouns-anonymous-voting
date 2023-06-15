// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/ZKRegistry.sol";

contract ZKRegistryScript is Script {
    function setUp() public {}

    function run() public {

        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        ZKRegistry zkRegistry = new ZKRegistry();

        vm.stopBroadcast();
    }
}
