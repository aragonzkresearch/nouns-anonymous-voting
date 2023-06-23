// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/ZKRegistry.sol";

contract ZKRegistryTest is Test {
    ZKRegistry public zkRegistry;

    function setUp() public {
        zkRegistry = new ZKRegistry();
    }

    function testRegister() public {
        uint8 interface_id = zkRegistry.POSEIDON_INTERFACE_ID();
        zkRegistry.register(interface_id, 0x1234);
        assertEq(zkRegistry.get(interface_id, address(this)), 0x1234);
    }

    function testDeregister() public {
        uint8 interface_id = zkRegistry.POSEIDON_INTERFACE_ID();
        zkRegistry.register(interface_id, 0x1234);
        zkRegistry.deregister(interface_id);
        assertEq(zkRegistry.get(interface_id, address(this)), 0);
    }
}
