// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.6;

import "nouns/NounsToken.sol";
import "./ZKRegistry.sol";
import "./INoirVerifier.sol";

contract NounsVotingContract {


    struct VotingProcess {
        /// The storage root of the NounsToken contract selected for voting
        uint256 nounsTokenStorageRoot;
        /// The storage root of the ZKRegistry contract selected for voting
        uint256 zkRegistryStorageRoot;
        /// The block number from which the storage roots were obtained
        uint256 startBlock;
        /// The block number at which the voting process will end
        uint256 endBlock;
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

    /// @dev The address of the NounsToken contract
    NounsToken public nounsTokenAddress;
    /// @dev The address of the ZKRegistry contract
    ZKRegistry public registryAddress;
    /// @dev The current chain id
    uint256 public chainId;
    /// @dev The Noir Vote Verifier contract address
    INoirVerifier public voteVerifierAddress;
    /// @dev The Noir Tally Verifier contract address
    INoirVerifier public tallyVerifierAddress;

    mapping (uint256 => VotingProcess) public votingProcesses;
    uint256 public voteId = 0;

    constructor(
        NounsToken _nounsTokenAddress,
        ZKRegistry _registryAddress,
        INoirVerifier _voteVerifierAddress,
        INoirVerifier _tallyVerifierAddress,
        uint256 _chainId
    ) {

        nounsTokenAddress = _nounsTokenAddress;
        registryAddress = _registryAddress;
        chainId = _chainId;

        voteVerifierAddress = _voteVerifierAddress;
        tallyVerifierAddress = _tallyVerifierAddress;

    }

    /// @notice This function is called to generate a new voting process
    /// @param _nounsTokenStorageRoot The storage root of the NounsToken contract selected for voting
    /// @param _zkRegistryStorageRoot The storage root of the ZKRegistry contract selected for voting
    /// @param _endBlock The block number at which the voting process will end
    /// @param _tlcsPublicKey The public key of the TLCS service that encrypts the votes to the point in the future. We use the BabyJubJub curve for public/private key encryption, represented in Affine coordinates {x, y}.
    /// @dev The storage roots should be for the same block
    /// @warning To make the voting process secure, instead of using the storage roots directly, we should use the block hash obtained inside the contract. This will be done in a future version.
    /// @return The id of the voting process
    function createProcess(
        uint256 nounsTokenStorageRoot,
        uint256 zkRegistryStorageRoot,
        uint256 endBlock,
        uint256[2] tlcsPublicKey
    ) public returns(uint256) {

        votingProcesses[voteId] = VotingProcess({
            nounsTokenStorageRoot: nounsTokenStorageRoot,
            zkRegistryStorageRoot: zkRegistryStorageRoot,
            startBlock: block.number,
            endBlock: endBlock,
            tlcsPublicKey: tlcsPublicKey,
            votesFor: 0,
            votesAgainst: 0,
            finished: false
        });

        // Increase the voteId for the next voting process
        voteId += 1;

        return voteId - 1;
    }

    /// @notice This function is called by voter to submit their vote
    /// @param processId The id of the voting process
    /// @param a The first part of the encrypted vote
    /// @param b The second part of the encrypted vote
    /// @param n The nullifier of the encrypted vote
    /// @param proof The proof of the vote correctness
    /// @warning We should consider doing this using Account Abstraction to allow anyone to submit the vote on behalf of the voter
    function submitVote(
        uint256 processId,
        uint256 a,
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
                process.nounsTokenStorageRoot,
                process.zkRegistryStorageRoot,
                process.tlcsPublicKey,
                a,
                b,
                n,
                h_id,
                proof
            ),
            "Vote is not correct"
        );

        // Recalculate the election state value
        // Right now we do so as H(electionStateValue, b)
        process.ballotsHash = uint256(keccak256(abi.encodePacked(process.ballotsHash, b)));
    }

    /// @notice This function is called to end the voting process
    /// @param processId The id of the voting process
    /// @param votesFor The number of votes for the voting process
    /// @param votesAgainst The number of votes against the voting process
    /// @param votesAbstain The number of votes abstaining from voting for the voting process
    /// @param proof The proof of the tally correctness
    function submitTallyResult(
        uint256 processId,
        uint256 votesFor,
        uint256 votesAgainst,
        uint256 votesAbstain,
        bytes calldata proof
    ) public {

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
            (bool success, ) = process.executableAction.target.call(
                abi.encodeWithSelector(action.funcSignature, action.args)
            );
        }

    }


    /// @notice This function is used to abstract a call to the Noir Vote Verifier contract
    function _verifyVote(
        uint256 processId,
        uint256 nounsTokenStorageRoot,
        uint256 zkRegistryStorageRoot,
        uint256[2] tlcsPublicKey,
        uint256 a,
        uint256 b,
        uint256 n,
        uint256 h_id,
        bytes calldata proof
    ) internal returns(bool) {
        return voteVerifierAddress.verify(
            proof,
            abi.encodePacked(
                a,
                b,
                n,
                h_id,
                chainId, // Part 1 of the `id` value
                processId, // Part 2 of the `id` value
                address(this), // Part 3 of the `id` value
                /// @warning This should be the block hash instead of the storage root
                nounsTokenStorageRoot,
                /// @warning This should be the block hash instead of the storage root
                zkRegistryStorageRoot,
                tlcsPublicKey
            )
        );
    }


    /// @notice This function is used to abstract a call to the Noir Tally Verifier contract
    /// @dev We assume that the processId existss
    function _verifyTally(
        uint256 votesFor,
        uint256 votesAgainst,
        uint256 votesAbstain,
        uint256 ballotsHash,
        bytes calldata proof
    ) internal returns(bool) {
        return tallyVerifierAddress.verify(
            proof,
            abi.encodePacked(
                votesFor,
                votesAgainst,
                votesAbstain,
                ballotsHash
            )
        );
    }

}
