// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract ZKRegistry {
    mapping(uint8 => mapping(address => uint256)) public registry;

    /// We currently support only maximum 16 interfaces
    uint8 public BBJJPK_INTERFACE_ID = 0x00; // first byte of keccak("BBJJPK")
    uint8 public BLS12PK_INTERFACE_ID = 0xc2; // first byte of keccak("BLS12PK")
    uint8 public POSEIDON_INTERFACE_ID = 0xbf; // first byte of keccak("POSEIDON")

    /// Register a value for a given interface id
    function register(uint8 interface_id, uint256 value) public {
        registry[interface_id][msg.sender] = value;
    }

    /// De-register a value for a given interface id
    function deregister(uint8 interface_id) public {
        delete registry[interface_id][msg.sender];
    }

    /// Get a value for a given interface id and address
    function get(uint8 interface_id, address addr) public view returns (uint256) {
        return registry[interface_id][addr];
    }
}
