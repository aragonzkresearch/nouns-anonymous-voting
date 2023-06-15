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
        // Set 0x0 address as proxy address, since we don't need it for our purposes
        // This address is only used for the OpenSea integration
        IProxyRegistry proxyRegistry = IProxyRegistry(address(0x0));

        new NounsToken(
            deployerAddress, // owner
            deployerAddress, // minter
            nounsDescriptor,
            nounsSeeder,
            proxyRegistry
        );

        vm.stopBroadcast();
    }
}
