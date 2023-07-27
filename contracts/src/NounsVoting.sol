// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

import "nouns/NounsToken.sol";
import "./ZKRegistry.sol";
import "./INoirVerifier.sol";
import "./Poseidon.sol";

contract NounsVoting {


    struct VotingProcess {
	/// IPFS CID of vote proposal
	bytes32 ipfsHash; // TODO
	/// The block number at which the census is taken
	uint64 censusBlock;
	/// The block number at which the voting process begins
	uint64 startBlock; // TODO
	/// The hash of the aforementioned block
        bytes32 blockHash;
        /// The block number at which the voting process will end
        uint64 endBlock;
	/// ZK Registry contract storage root at reference block
	bytes32 zkRegistryStorageRoot;
	/// Nouns token contract storage root at reference block
	bytes32 nounsTokenStorageRoot;
	/// TLCS round number
	uint64 tlcsRoundNumber;
        /// TLCS public key used to encrypt the votes for later decryption
        uint256[2] tlcsPublicKey;
        /// Value defining unique election state
        uint256 ballotsHash;
        /// The number of votes for the VotingProcess.
        /// @dev This value is populated after the voting process ends
        uint256 votesFor;
        /// The number of votes against the VotingProcess.
        /// @dev This value is populated after the voting process ends
        uint256 votesAgainst;
        /// The number of votes abstaining from voting for the VotingProcess.
        /// @dev This value is populated after the voting process ends
        uint256 votesAbstain;
        /// Indicates whether the voting process has ended
        /// @dev Default value is `false`
        bool tallied; // TODO
        /// The executable action to be executed after the voting process ends
        ExecutableAction action;
    }

    struct ExecutableAction {
        /// The target address on which the action will be executed
        /// @dev Default value is `address(0)`
        address target;
        /// The function signature of the action to be executed after the voting process ends
        /// @dev Default value is `bytes4(0)`
        bytes4 funcSignature;
        /// The function arguments of the action to be executed after the voting process ends
        /// @dev Default value is `bytes(0)`
        bytes args;
    }
	
    /// This is emitted when a voter submits a vote
    event BallotCast(uint256 processId, uint256 indexed a_x, uint256 indexed a_y, uint256 indexed b);

    bytes32[] public_args;

    /// The address of the NounsToken contract
    NounsToken public nounsToken;
    /// The address of the ZKRegistry contract
    ZKRegistry public zkRegistry;
    /// The Noir block hash verifier contract
    INoirVerifier private hashVerifier;
    /// The Noir Vote Verifier contract address
    INoirVerifier private voteVerifier;
    /// The Noir Tally Verifier contract address
    INoirVerifier private tallyVerifier;
    /// The Poseidon Hash contract address
    Poseidon2 private poseidon2;

    mapping(uint256 => VotingProcess) public votingProcesses;

    mapping(uint256 => bool) public nullifiers;
    
    // The id of the next voting process
    uint256 public nextProcessId = 0;

    constructor(
        NounsToken _nounsToken,
        ZKRegistry _zkRegistry,
	INoirVerifier _hashVerifier,
        INoirVerifier _voteVerifier,
        INoirVerifier _tallyVerifier,
        Poseidon2 _poseidon
    ) {

        nounsToken = _nounsToken;
        zkRegistry = _zkRegistry;

	hashVerifier = _hashVerifier;
        voteVerifier = _voteVerifier;
        tallyVerifier = _tallyVerifier;

        poseidon2 = _poseidon;
    }

    /// @notice This function is called to generate a new voting process
    /// @param blockDuration The number of blocks that the voting process will last
    /// @param tlcsPublicKey The public key of the TLCS service that encrypts the votes to the point in the future. We use the BabyJubJub curve for public/private key encryption, represented in Affine coordinates {x, y}. We trust that the voter will cross-check the public key with the one published by the TLCS service.
    /// @dev The storage roots should be for the same block
    /// @notice To make the voting process secure, instead of using the storage roots directly, we should use the block hash obtained inside the contract. This will be done in a future version.
    /// @return The id of the voting process
    function createProcess(
			   bytes32 ipfsHash,
			   uint64 startDelay,
			   uint64 blockDuration,
			   uint64 tlcsRoundNumber,
			   uint256[2] calldata tlcsPublicKey,
			   uint64 census_block_number, // TODO: Should be < 256 blocks in the past
			   bytes32 registry_storage_root,
			   bytes32 nft_storage_root,
			   bytes calldata hash_proof
    ) public returns (uint256) {

	bytes32 census_block_hash = blockhash(census_block_number); // This will be zero if we're out of range, but then the proof will fail to pass.

	// Form verifier argument
	_push_u256(census_block_hash);
	_push_address(address(zkRegistry));
	_push_u256(registry_storage_root);
	_push_address(address(nounsToken));
	_push_u256(nft_storage_root);
	
	require(hashVerifier.verify(hash_proof, public_args), "Error: Invalid proof");

	// free
	delete public_args;
	
        bytes memory emptyBytes = bytes("");

        return createProcessWithExecutableAction(
						 ipfsHash,
						 startDelay,
            blockDuration,
						 tlcsRoundNumber,
            tlcsPublicKey,
	    census_block_number,
	    census_block_hash,
	    registry_storage_root,
	    nft_storage_root,
            address(0),
            bytes4(0),
            emptyBytes
        );

    }

    /// @notice This function is called to generate a new voting process with an executable action
    /// @param blockDuration The number of blocks that the voting process will last
    /// @param tlcsPublicKey The public key of the TLCS service that encrypts the votes to the point in the future. We trust that the voter will cross-check the public key with the one published by the TLCS service.
    /// @param target The target address on which the action will be executed
    /// @param funcSignature The function signature of the action to be executed after the voting process ends
    /// @param args The function arguments of the action to be executed after the voting process ends
    /// @dev The storage roots should be for the same block
    /// @notice To make the voting process secure, instead of using the storage roots directly, we should use the block hash obtained inside the contract
    /// @return The id of the voting process
    function createProcessWithExecutableAction(
					       bytes32 ipfsHash,
					       uint64 startDelay,
        uint64 blockDuration,
					       uint64 tlcsRoundNumber,
        uint256[2] calldata tlcsPublicKey,
	uint64 census_block_number,
	bytes32 census_block_hash,
	bytes32 registry_storage_root,
	bytes32 nft_storage_root,
        address target,
        bytes4 funcSignature,
        bytes memory args
    ) public returns (uint256) {

	
	
        // Create the executable action
        ExecutableAction memory action = ExecutableAction({
            target: target,
            funcSignature: funcSignature,
            args: args
        });

        votingProcesses[nextProcessId] = VotingProcess({
	    ipfsHash: ipfsHash,
            censusBlock: census_block_number,
	    startBlock: uint64(block.number) + startDelay,
	    blockHash: census_block_hash,
            endBlock: uint64(block.number) + startDelay + blockDuration,
	    zkRegistryStorageRoot: registry_storage_root,
	    nounsTokenStorageRoot: nft_storage_root,
	    tlcsRoundNumber: tlcsRoundNumber,
            tlcsPublicKey: tlcsPublicKey,
            ballotsHash: 0,
            votesFor: 0,
            votesAgainst: 0,
            votesAbstain: 0,
            tallied: false,
            action: action
        });

        // Increase the voteId for the next voting process
	nextProcessId++;
	
        return nextProcessId;
    }

    /// @notice This function is called by voter to submit their vote
    /// @param processId The id of the voting process
    /// @param a The first part of the encrypted vote
    /// @param b The second part of the encrypted vote
    /// @param n The nullifier of the encrypted vote
    /// @param h_id The hash of the id of the vote, to prevent malleability
    /// @param proof The proof of the vote correctness
    /// @notice We should consider doing this using Account Abstraction to allow anyone to submit the vote on behalf of the voter
    function submitVote(
        uint256 processId,
        uint256[2] memory a,
        uint256 b,
        uint256 n,
        uint256 h_id, // TODO: Factor out
        bytes calldata proof
    ) public {

        // Check that the voting process exists
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");

	// Check whether voting process has begun
	require(block.number >= votingProcesses[processId].startBlock, "Voting process has not started");
	
        // Check that the voting process has not ended
        require(votingProcesses[processId].endBlock > block.number, "Voting process has ended");

	// Check whether user has already voted
	require(!nullifiers[n], "User has already voted");

        // Get the process data
        VotingProcess storage process = votingProcesses[processId];

        // Check the vote correctness
        require(
            _verifyVote(
                processId,
                process.tlcsPublicKey,
                a,
                b,
                n,
                h_id,
                proof
            ),
            "Vote is not correct"
        );

	// Add nullifier
	nullifiers[n] = true;

        uint256[2] memory hashingArgs = [process.ballotsHash, b];

        // Recalculate the election state value
        // Right now we do so as H(electionStateValue, b)
        process.ballotsHash = poseidon2.poseidon(hashingArgs);

        emit BallotCast(processId, a[0], a[1], b);
    }

    /// @notice This function is called to end the voting process
    /// @param processId The id of the voting process
    /// @param tallyResult The number of votes against, for and abstaining from voting for the voting process
    /// @param proof The proof of the tally correctness
    function submitTallyResult(
        uint256 processId,
        uint256[3] memory tallyResult,
        bytes calldata proof
    ) public {

        uint256 votesAgainst = tallyResult[0];
        uint256 votesFor = tallyResult[1];
        uint256 votesAbstain = tallyResult[2];

        // Check that the voting process exists
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");

        // Check that the voting process has ended
        require(votingProcesses[processId].endBlock <= block.number, "Voting process has not ended");

        // Check that the voting process has not already been tallied
        require(!votingProcesses[processId].tallied, "Votes have already been tallied"); // TODO

        // Get the process data
        VotingProcess storage process = votingProcesses[processId];

        // Check the tally correctness
        require(
		_verifyTally(
			     processId,
			     votesFor,
			     votesAgainst,
			     votesAbstain,
			     process.ballotsHash,
			     proof
		),
		"Tally is not correct"
        );

        // Update the voting process state
        process.votesFor = votesFor;
        process.votesAgainst = votesAgainst;
        process.votesAbstain = votesAbstain;
        process.tallied = true;

        // If the voting process was successful, execute the action
        if (votesFor > votesAgainst && process.action.target != address(0)) {

            ExecutableAction storage action = process.action;

            // Execute the action
            (bool success,) = action.target.call(
                abi.encodeWithSelector(action.funcSignature, action.args)
            );

            // Check that the action was executed successfully
            require(success, "Action execution failed");
        }

    }
    
    /// @notice This function returns the IPFS CID of the proposal
    /// @param processId The id of the voting process
    /// @return IPFS CID of proposal
    function getIpfsHash(uint256 processId) public view returns (bytes32) { // TODO
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");
        return votingProcesses[processId].ipfsHash;
    }
    /// @notice This function returns the block number when the census was taken
    /// @param processId The id of the voting process
    /// @return The block number when the census was taken
    function getCensusBlock(uint256 processId) public view returns (uint64) { // TODO
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");
        return votingProcesses[processId].censusBlock;
    }
    /// @notice This function returns the block number when the voting process started
    /// @param processId The id of the voting process
    /// @return The block number when the voting process started
    function getStartBlock(uint256 processId) public view returns (uint64) {
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");
        return votingProcesses[processId].startBlock;
    }
    /// @notice This function returns the block number when the voting process ends
    /// @param processId The id of the voting process
    /// @return The block number when the voting process ends
    function getEndBlock(uint256 processId) public view returns (uint64) {
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");
        return votingProcesses[processId].endBlock;
    }
    /// @notice This function returns the TLCS round number of the voting process
    /// @param processId The id of the voting process
    /// @return The TLCS round number of the voting process
    function getTlcsRoundNumber(uint256 processId) public view returns (uint64) {
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");
        return votingProcesses[processId].tlcsRoundNumber;
    }
    /// @notice This function returns the ballot hash of the voting process
    /// @param processId The id of the voting process
    /// @return The ballot hash of the voting process
    function getBallotsHash(uint256 processId) public view returns (uint256) {
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");
        return votingProcesses[processId].ballotsHash;
    }

    /// @notice This function returns the result of the voting process as a tuple of votes against, for and abstaining from voting
    /// @param processId The id of the voting process
    /// @return The result of the voting process as a tuple of votes against, for and abstaining from voting
    function getTallyResult(uint256 processId) public view returns (uint256[3] memory) {
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");
        require(votingProcesses[processId].tallied, "Votes have not been tallied yet");
        return [
            votingProcesses[processId].votesAgainst,
            votingProcesses[processId].votesFor,
            votingProcesses[processId].votesAbstain
            ];
    }

    /// @notice This function is used to abstract a call to the Noir Vote Verifier contract
    function _verifyVote(
        uint256 processId,
        uint256[2] memory tlcsPublicKey,
        uint256[2] memory a,
        uint256 b,
        uint256 n,
        uint256 h_id,
        bytes calldata proof
    ) internal returns (bool) {

	_push_uint256(a[0]);
	_push_uint256(a[1]);
	_push_uint256(b);
	_push_uint256(n);
	_push_uint256(h_id);
	_push_uint256(processId);
	_push_address(address(this));
        _push_u256(bytes32(block.chainid));
	_push_u256(votingProcesses[processId].zkRegistryStorageRoot);
	_push_u256(votingProcesses[processId].nounsTokenStorageRoot);
	_push_uint256(tlcsPublicKey[0]);
	_push_uint256(tlcsPublicKey[1]);

        bool result = voteVerifier.verify(
            proof,
            public_args
        );

        // Clear the public args
        delete public_args;

        return result;
    }

    /// @notice This function is used to abstract a call to the Noir Tally Verifier contract
    function _verifyTally(
			  uint256 processId,
			  uint256 votesFor,
        uint256 votesAgainst,
        uint256 votesAbstain,
        uint256 ballotsHash,
        bytes calldata proof
    ) internal returns (bool) {

	_push_uint256(ballotsHash);
	_push_uint256(processId);
	_push_address(address(this));
	_push_u256(bytes32(block.chainid));
	_push_uint256(votesAgainst);
	_push_uint256(votesFor);
	_push_uint256(votesAbstain);

        bool result = tallyVerifier.verify(
            proof,
            public_args
        );

        // Clear the public args
        delete public_args;

        return result;
    }


    function _bytesToBytes32(bytes memory b) private pure returns (bytes32) {
        bytes32 out;

        for (uint i = 0; i < 32; i++) {
            out |= bytes32(b[i] & 0xFF) >> (i * 8);
        }
        return out;
    }

    function _bytes32To32Bytes32(bytes32 b) private pure returns (bytes32[32] memory)
    {
	bytes32[32] memory out;

	for (uint i = 0; i < 32; i++) {
	    out[31 - i] = bytes32(uint256(b) % 256);
	    b = b >> 8;
    }

    return out;
    
    }

    /// Push 256-bit value (pair of field elements) to verifier arguments
    function _push_u256(bytes32 arg) private 
    {
	public_args.push(arg >> 128);
	public_args.push(arg & bytes32(uint256(((1 << 128) - 1))));
    }

    /// Push 160-bit address to verifier arguments
    function _push_address(address addr) private
    {
	public_args.push(bytes32(uint256(uint160(addr))));
    }

    function _push_uint256(uint256 arg) private
    {
	bytes32 x = bytes32(arg);
	public_args.push(x);
    }


}
