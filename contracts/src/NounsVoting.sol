// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

import "nouns/NounsToken.sol";
import "./ZKRegistry.sol";
import "./INoirVerifier.sol";
import "./Poseidon.sol";

contract NounsVoting {


    struct VotingProcess {
        /// The block hash of the start block of the voting process, used to check the storage proofs
        bytes32 startBlockHash;
        /// The block number from which the storage roots were obtained
        uint64 startBlock;
        /// The block number at which the voting process will end
        uint64 endBlock;
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
        bool finished;
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

    /// This when a voter submits a vote
    event BallotCast(uint256 processId, uint256 a_x, uint256 a_y, uint256 b);

    bytes32[] public_args;

    /// The address of the NounsToken contract
    NounsToken public nounsToken;
    /// The address of the ZKRegistry contract
    ZKRegistry public zkRegistry;
    /// The Noir Vote Verifier contract address
    INoirVerifier private voteVerifier;
    /// The Noir Tally Verifier contract address
    INoirVerifier private tallyVerifier;
    /// The Poseidon Hash contract address
    Poseidon2 private poseidon2;

    mapping(uint256 => VotingProcess) public votingProcesses;
    // The id of the next voting process
    uint256 public nextProcessId = 0;

    constructor(
        NounsToken _nounsToken,
        ZKRegistry _zkRegistry,
        INoirVerifier _voteVerifier,
        INoirVerifier _tallyVerifier,
        Poseidon2 _poseidon
    ) {

        nounsToken = _nounsToken;
        zkRegistry = _zkRegistry;

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
        uint64 blockDuration,
        uint256[2] calldata tlcsPublicKey
    ) public returns (uint256) {


        bytes memory emptyBytes = bytes("");

        return createProcessWithExecutableAction(
            blockDuration,
            tlcsPublicKey,
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
        uint64 blockDuration,
        uint256[2] calldata tlcsPublicKey,
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
            startBlockHash: blockhash(block.number),
            startBlock: uint64(block.number),
            endBlock: uint64(block.number) + blockDuration,
            tlcsPublicKey: tlcsPublicKey,
            ballotsHash: 0,
            votesFor: 0,
            votesAgainst: 0,
            votesAbstain: 0,
            finished: false,
            action: action
        });

        // Increase the voteId for the next voting process
        nextProcessId += 1;

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
        uint256 h_id,
        bytes calldata proof
    ) public {

        // Check that the voting process exists
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");

        // Check that the voting process has not ended
        require(votingProcesses[processId].endBlock > block.number, "Voting process has ended");

        // Get the process data
        VotingProcess storage process = votingProcesses[processId];

        // Check the vote correctness
        require(
            _verifyVote(
                processId,
                process.startBlockHash,
                process.tlcsPublicKey,
                a,
                b,
                n,
                h_id,
                proof
            ),
            "Vote is not correct"
        );

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

        // Check that the voting process has not finished
        require(!votingProcesses[processId].finished, "Voting process has finished");

        // Get the process data
        VotingProcess storage process = votingProcesses[processId];

        // Check the tally correctness
        require(
            _verifyTally(
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
        process.finished = true;

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

    /// @notice This function returns the block number when the voting process started
    /// @param processId The id of the voting process
    /// @return The block number when the voting process started
    function getStartBlock(uint256 processId) public view returns (uint256) {
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");
        return votingProcesses[processId].startBlock;
    }

    /// @notice This function returns the ballot hash of the voting process
    /// @param processId The id of the voting process
    /// @return The ballot hash of the voting process
    function getBallotsHash(uint256 processId) public view returns (uint256) {
        require(votingProcesses[processId].endBlock != 0, "Voting process does not exist");
        return votingProcesses[processId].ballotsHash;
    }

    /// @notice This function is used to abstract a call to the Noir Vote Verifier contract
    function _verifyVote(
        uint256 processId,
        bytes32 startBlockHash,
        uint256[2] memory tlcsPublicKey,
        uint256[2] memory a,
        uint256 b,
        uint256 n,
        uint256 h_id,
        bytes calldata proof
    ) internal returns (bool) {


        uint256 chainId;
        assembly {
            chainId := chainid()
        }

        public_args.push(_bytesToBytes32(abi.encode(a)));
        public_args.push(_bytesToBytes32(abi.encode(b)));
        public_args.push(_bytesToBytes32(abi.encode(n)));
        public_args.push(_bytesToBytes32(abi.encode(h_id)));
        public_args.push(_bytesToBytes32(abi.encode(chainId))); // Part 1 of the `id` value
        public_args.push(_bytesToBytes32(abi.encode(processId))); // Part 2 of the `id` value
        public_args.push(_bytesToBytes32(abi.encode(address(this)))); // Part 3 of the `id` value
        public_args.push(startBlockHash);
        public_args.push(_bytesToBytes32(abi.encode(tlcsPublicKey)));


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
        uint256 votesFor,
        uint256 votesAgainst,
        uint256 votesAbstain,
        uint256 ballotsHash,
        bytes calldata proof
    ) internal returns (bool) {

        public_args.push(_bytesToBytes32(abi.encode(votesFor)));
        public_args.push(_bytesToBytes32(abi.encode(votesAgainst)));
        public_args.push(_bytesToBytes32(abi.encode(votesAbstain)));
        public_args.push(_bytesToBytes32(abi.encode(ballotsHash)));

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

}
