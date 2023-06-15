// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "nouns/NounsToken.sol";
import "nouns/NounsDescriptor.sol";
import "nouns/NounsSeeder.sol";

contract ZKRegistryScript is Script {
    function setUp() public {}

    function run() public {

        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(deployerPrivateKey);

        vm.startBroadcast(deployerPrivateKey);

        NounsDescriptor nounsDescriptor = new NounsDescriptor();
        NounsSeeder nounsSeeder = new NounsSeeder();

        NounsToken nounsToken = new NounsToken(
            deployerAddress, // owner
            deployerAddress, // minter
            address(nounsDescriptor),
            address(nounsSeeder),
            address(0x0) // Proxy address is only used for OpenSea integration, not needed for our purposes
        );

        vm.stopBroadcast();
    }
}
