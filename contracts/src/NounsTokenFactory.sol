pragma solidity ^0.8.13;

import "nouns/NounsToken.sol";
import "nouns/NounsDescriptor.sol";
import "nouns/NounsSeeder.sol";

contract NounsTokenFactory {


    function deploy(address owner) public returns (NounsToken) {

        if (owner == address(0)) {
            owner = msg.sender;
        }

        NounsDescriptor nounsDescriptor = new NounsDescriptor();
        NounsSeeder nounsSeeder = new NounsSeeder();
        // Set 0x0 address as proxy address, since we don't need it for our purposes
        // This address is only used for the OpenSea integration
        IProxyRegistry proxyRegistry = IProxyRegistry(address(0x0));

        address deployerAddress = msg.sender;

        NounsToken token = new NounsToken(
            deployerAddress, // owner
            deployerAddress, // minter
            nounsDescriptor,
            nounsSeeder,
            proxyRegistry
        );

        return token;
    }
}