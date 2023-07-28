// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "nouns/NounsDescriptor.sol";
import "nouns/NounsSeeder.sol";
import "../src/ZKRegistry.sol";
import "../src/NounsVoting.sol";

contract ZKRegistryTest is Test {
    ZKRegistry public zkRegistry;
    NounsVoting public nounsVoting;
    INoirVerifier public hashVerifier;
    INoirVerifier public voteVerifier;
    INoirVerifier public tallyVerifier;
    Poseidon2 public poseidon2;
    NounsToken public nounsToken;

    function setUp() public {
        zkRegistry = new ZKRegistry();
	hashVerifier = new YesManNoirVerifier();
	voteVerifier = new YesManNoirVerifier();
	tallyVerifier = new YesManNoirVerifier();
	PoseidonFactory poseidonFactory = new PoseidonFactory();
	poseidon2 = poseidonFactory.poseidon2();
	NounsDescriptor nounsDescriptor = new NounsDescriptor();
	NounsSeeder nounsSeeder = new NounsSeeder();
	DummyProxyRegistry dummyProxy = new DummyProxyRegistry();
	nounsToken = new NounsToken(address(0x0), address(0x0), nounsDescriptor, nounsSeeder, dummyProxy);
	nounsVoting = new NounsVoting(nounsToken, zkRegistry, hashVerifier, voteVerifier, tallyVerifier, poseidon2);
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

    /* function testSubmitVote() public */
    /* { */
    /* 	uint256 process_id = nounsVoting.createProcess(uint64(5), [uint256(0), uint256(0)]); */
    /* 	nounsVoting.submitVote(0, [process_id,uint256(0)], 0, 0, 0, hex"00"); */
    /* 	//	nounsVoting.submitVote(0, [process_id,uint256(0)], 0, 0, 0, hex"00"); */
    /* } */
}

contract DummyProxyRegistry is IProxyRegistry
{
    function proxies(address add) external pure returns (address)
    {
	return add;
    }
}
