# Nouns Private Voting Design

###### tags: `Nouns DAO` `Private Voting` `Research Sprint`

:::success
This page is part of [documentation](https://hackmd.io/EUhEnQQcTAOpHZ7kbg-_9w) of the Aragon-Aztec project for the Nouns DAO Private Voting Research Sprint
:::

[TOC]

## Acronyms
<!--by alphabetic order-->
**BC** Blockchain
**LOE** League of Entropy
**DRS** Delay Relayer Service
**SC** Smart contract
**TLCS** TimeLock Cryptography Service

## Remarks

* This design assumes that we have no access to the voter's private key.
* In addition to the Voting service this design relies on the following services to be developed as part of the project:
    * The [TLCS (Time-Lock Cryptography Service)](#Time-Lock-Cryptography-Service-TLCS)  generates public keys whose corresponding private keys are published at predetermined times in the future. This service will allow us to keep ballots secret during the voting period. These ballots can then be decrypted after the voting period ends, without breaking anonymity.
    * In the  [zkGlobalRegistry](#zkGlobalRegistry-zkregeth) users can register commitments to secret "registry keys" which can then be used to generate unique nullifiers for votes. This serves to prevent double voting.
    * The [DRS (Delay-Relayer Service)](Delay-Relayer-Service-DRS), to enable third parties to pay the cost of submitting votes and also to break time correlation. It is not needed in the first phase so its design is still work in progress.

TLCS and zkGloblaRegistry are independent services that despite being developed under this project can have other applications.

## General overview
Anyone who wants to participate in a voting process must register in the zkGlobalRegistry service. This must only be done once, but **it is mandatory** and must be done before the voting process in which the voters wants to participate has started.

Nouns holders or delegates will be able to cast votes for the NFTs they own or that have been delegated to them. Delegation and un-delegation will be done as now. The voting process can be summarised as follows

* Creation of new voting process:
    * Obtain public encryption key from the Time-Lock Cryptography Service.
    * Create new vote by submitting details to the smart contract, like voting period, description of vote, and encryption key.
* During the voting period:
    *  Users connect to a UI and generate of one of more votes (depending on the number of NFTs they hold or have been delegated to).
    *  Optionally, users can choose to immediately submit these votes to the Delay-Relayer for later anonymous submission to the Ethereum contract.
    *  Users can also choose to store their votes and later submit them to the smart contract themselves, or to submit it to the Delay-Relayer via other means, e.g. Tor.
    *  Users can check if their vote is submitted to the smart contract (must have stored a hash at submission time) .
* After the voting period:
    * The private key for ballot decryption is made available by the TLCS.
    * Anyone can decrypt ballots and compute the result of the vote and generate the corresponding proof (open source software).
    * Anyone can send the result+proof to the contract.


## (Ethereum) Smart contracts
* **Voting Smart Contract** This will be the master voting contract. To deploy it, knowledge of the `nouns_NFT_Address` and of the `zk_global_registry_Address` will be necessary. This $SC$ will be used to:
    * create a new voting process
    * cast votes
    * register vote results (the tally)

* **zkGlobalRegistry Smart Contract** This will be used to store registry keys, which are then used to prevent double voting via nullifiers.


# Prerequisites
- To create a voting proces (any Ethereum address)
    - Have sufficient ETH to execute `createProcess()` function of the Voting SC
    - Know the Nouns NFT SC address
    - Know the zkGlobalRegistry SC address
- To submit a vote (voter)
    - Hold or have been delegated Nouns NFT(s)
    - Have registered with the zkGlobalRegistry SC
    - Have sufficient ETH to execute the `submitVote()` function of the Voting SC (one per vote) if submitted to Ethereum BC
- To submit a result (any Ethereum address)
    - Have sufficient ETH to execute `submitResult()` function of the Voting SC
    - Have computed the result and tally proof

# Voting Process Creation
Any Ethereum address with sufficient ETH to pay for the gas fees can create a voting process `createProcess()` function with:

- `title` of the voting process
- `description (optional)` an arbitrary string (e.g. link to additional documentation)
- `snapshot_blocknum` at which token-holdings/delegation are checked
- `start_blocknum` of the voting period
- `end_time` UNIX time at which the voting ends <!--, based on block timestamp (this is because the decryption key will be based on a timestamp and the further we go into the future, the less precise the estimation of the timestamp of a block is)-->
- `public_encryption_key` from TLCS based on `end_time` (the corresponding private key can only be obtained after `end_time`)

The SC generates a `process_id` which is a unique identifier of the voting process.

# Voting Frontend
## User Interface
This is the frontend interface to cast the votes.

With a wallet connected:
- It shows the list of all active voting processes (title, start and ending time, voting power, etc.) available to the connected wallets address
    - and allows to select one of them
- By selecting a process, it shows the list of all the NFTs associated (either held or delegated) to the address being used at the `snapshot_blocknum` of the process
    - and allows the voter to select one or more of them NFTs to vote with ("select all" is available)
- By selecting a set of NFTs, it shows the options available ("For", "Against", "Abstain")
- By selecting an option, it shows the "Confirm Vote" button, which on click:
    - Prompts and collects voter's Metamask signature
    - Generates vote(s) (including the proof(s)) for the selected NFT(s)
    - Offers the voter two options:
        - Save the vote(s) locally, to submit them at a later time
        - Send vote(s) to the DRS

## Vote generation
A vote consists of: a vote's option (expressing voter's preference) + a proof of having an NFT associated. There will be one vote per NFT.

:::warning
The intent is for the vote generation to be done entirely in the user's browser. However, depsite ongoing intensive efforts by Aztec and AZKR, it remains unclear how long the computation of such proofs in the browser will take. Therefore, as additional solution, we might consider developing a standalone module to generate these storage proofs separately. If we decide to this, we will add the corresponding design to this document.
:::

The process for voter $i$ is as follows, with $H$ denoting a zk-friendly hash function:

++Known parameters:++
- $t$: time to decrypt votes (Voting SC `end_time`)
- $PK_{t}$ the public key for time $t$ (TLCS `public_encryption_key`). We have $PK_t = g^{sk_t}$ where $sk_t$ is the corresponding secret key for time $t$ which will be published after the voting period ends.
- $RK_i$ is the voter's registry key, corresponding to the `rk_commitment` in the zkGlobalRegistry. This is known by the voter and derived from his wallet via a signature over a designated string.
- $v_i$ is the vote's option

++Before generating the proof:++
- fetch 3 [storageProof](https://eips.ethereum.org/EIPS/eip-1186)s (from eth-node) for `snapshot_blocknum`
    - 2 for NFT&ownership or NFT&delegation
    - 1 for $RK_i$
- obtain $RK_i$
- generate randomness $r_i \in^R \mathbb{Z}_p$
- compute:
    - $A_i=g^{r_i}$
    - $K_i =PK_{t}^{r_i}$ (note that this is equal to $g^{r_i sk_i}$)
    <!-- $K_i'=H_{2/3/4}(K_i)$, where $K_i = e(H_1(t), PK_{tlock})^{r_i}$-->
    - encrypted vote option: $B_i=H(K_i, v_i, chain_{ID}, process_{ID}, contract_{ADDR})$
    - the vote nullifier: $N_i = H(RK_i, NFT_{ID}, chain_{ID}, process_{ID}, contract_{ADDR})$
    - the vote signature (for malleability protection): $VS_i=H(v_i,RK_i)$

++Proof generation (Noir):++

Proofs for the following checks:
1. Right to vote at `snapshot_blocknum`
    - A. Direct Voter:
        - A.1. Voter holds Nouns NFT [EthStorageProof]
          `ownerOf[token_id] == account`
        - A.2. Voter hasn't delegated [EthStorageProof]
          `_delegates[account] == 0x0`
    - B. Delegated Voter:
        - B.1. Delegator (who is delegating) holds Nouns NFT [EthStorageProof]
          `ownerOf[token_id] == delegator`
        - B.2. Delegator has delegated to Voter [EthStorageProof]
          `_delegates[delegator] == account`
2. Valid nullifier
    - 2.1. `registry[0x0][voter_address] == rk_commitment` is in the [zkGlobalRegistry](#zkGlobalRegistry-zkregeth) [EthStorageProof] for the same `snapshot_blocknum`, where `rk_commitment=Hash(RK_i, chainID)`. For more details see [Section 7](#zkGlobalRegistry-zkregeth)
    - 2.2. $N_i = H(RK_i, NFT_{ID}, chain_{ID}, process_{ID})$
<!-- OLD - 2.3. $B_i =H_2(K_i, v_i, chain_{ID}, process_{ID}, contract_{ADDR})$, where $K_i=e(H_1(t), PK)$ -->
3. Valid vote option
    - 3.1. `vote_option` $\in \{0,~1,~2\}$ (abstain, yes, no)
4. Correct encryption of the vote
    - 4.1. There exists $r_i$ such that $g^{r_i}=A_i$ and $K_i=PK_{t}^{r_i}$
    - 4.2. $B_i =H(K_i, v_i, chain_{ID}, process_{ID}, contract_{ADDR})$, where $K_i=PK_{t}^{r_i}$
6. Vote Signature
    - 5.1. $VS_i=H(v_i,RK_i)$


++After generating the proof:++
- data sent to the smart contract:
    - $A_i$
    - $B_i$
    - $N_i$
    - `vote_proof`

## Vote submission
The votes can be submitted to:
* the Voting SC (ideally from an account that is in no way linked to the voter's account)
* to the DRS

The smart contract accepts the vote submission if:
1) The current block number is greater or equal to `start_blocknum`
2) The timestamp of the last block is strictly smaller than `end_time`.
3) No previous vote for the NFT has been submitted earlier. This is done by checking if the nullifier  $N_i$ is already in the hash table $T$. If it is not, $N_i$ is added to table $T$.
4) `vote_proof` is valid.

Moreover, if all previous checks 1-4 pass the smart contract will do the following:
The value $B_i$ that is received by the voter is hashed (using Keccak) into a value $B_K$ so that at the end of the voting period $B_K$ is the root of an unbalanced tree that contains all $B_i$'s in its leaves.
Precisely, $B_K=Keccak(B_K,B_i)$ (the value inside $Keccak$ is the previously stored value).

The SC stores the following data: $A_i$ and $B_i$ are stored as calldata. $N_i$ is added to the hash table. `vote_proof` does not need to be stored (but will remain available for inspection as they are inputs to the SC).

# Voting Result Aggregation / Tally Proof

++Known paramters:++
- $t$: time to decrypt votes, kown in the contract
- $PK_{t}$: TLCS public key

++Before generating the proof:++
- Fetch $A_i$ for $\forall i \in \{1, \ldots, n\}$
- Fetch $B_i$ for $\forall i \in \{1, \ldots, n\}$
- Fetch secret key of TLCS $sk_{t}$

++Get option for a voter $i$:++
- Compute $A_i^{sk_{t}} = g^{r_i sk_t} = K_i$.
- Find the first value $v_i\in \{0,1,2\}$ such that
    <!-- OLD - $K_i'=H_4(K_i)$, where $K_i = e(T_{sign}, A_i)$ -->
    - $B_i = H(K_i, v_i, chain_{ID}, process_{ID}, contract_{ADDR})$. (We will be able to find such value $v_i$ because the voter's ZK proof was verified succesfully.)

++Prove vote aggregation:++
- Sum all $v_i$ for each vote option to compute an array $vote_{count}$ storing # votes for, # votes against, # votes abstain.
- Given public inputs $B_K$, $chain_{ID}$, $process_{ID}$, $contract_{ADDR}$, $vote_{count}$ and witnesses $(K_i,v_i)$ we generate a zk proof of the following prorogram:
    - For all $i\in[n]$, the program computes $B_i =H(K_i, v_i, chain_{ID}, process_{ID}, contract_{ADDR})$
    - Compute $B_K' = Keccak(B_n, Keccak(B_{n-1}, Keccak(...))$ and verify that $B_k = B_K'$
    - Verify that the votes have been correctly counted, i.e. all $j\in{0,1,2}$ $vote_{count}[j]$ equals $|\{v_i|v_i=j\}|$
    - Output $1$ iff all verifications passed

++Verifier (Solidity):++
> **Note** Part of the Voting Smart Contract

Inputs (to verify the proof):
- `vote_count [(uint256, uint256, uint256)]`  Triple storing # votes for, # votes against, # votes abstain)
- `ballots_hash [uint256]` aggregated $B_K$ of all ballots known to smart contract
- `tally_proof`

In addition, the $SC$ has access to the following information:
- `process_id`
- `chain_id`
- `contract_addr`

If the `tally_proof` is correct, the $SC$ then sets the tally fields with the provided voting result, which can be then be publicly queried by other smart contracts.

# zkGlobalRegistry (zkreg.eth)

zkGlobalRegistry is the smart contract portion of a standardized scheme for users to register commitments to (usually secret) secret data tied to their wallets. The registry key can be arbitrarily decided upon by the user, for example:
1. [User Friendly] Use MetaMask signature over some fixed value string, and then hash it to obtain the Registry Key
2. [Secure] Generate a new Registry Key from entropy

The scheme is useful for standardizing the secret keys users use to access different zk applications, preventing the need for zk applications to resort to extracting and using Ethereum private keys as secret keys to achieve standardized UXs. (Many zk applications make use of secret values to introduce randomness into certain hashed components of the application, e.g. nullifiers, in order to counter bruteforce attacks.)

The zkGlobalRegistry stores a voter's commitment to the registry key. For a registry key $RK$, the commitment will be stored as:

`registry[0x0][address]` = `rk_commitment` = $Poseidon(RK, chainID)$

We can additionally extend the function of the zkGlobalRegistry to work as a mapping between a voters address and their new Public Key. This would imply that the registry system could have multiple interfaces to support different Public Key schemes, such as `BBJJPK`, `BLS12PK`, etc. The structure of the mapping is as follows:
`registry[INTERFACE][address]==value`
- `INTERFACE` is a 4 byte code of the hash of `BBJJPK`, `BLS12PK`, etc. We have decided to use `0x0` for the `Commitment` interface.
- `value` is corresponding to the `INTERFACE` and `address`

**Solidity sketch**
```solidity

contract ZKRegistry{
    mapping(uint256 => mapping(uint256 => uint256)) public registry;
    
    // - what's checked in the register method
    // - how to update a register
    // - how to revoke a register
}
```

*Key Registration*: adds or updates the key in the registry
- input:
    - `interface [byte4]` the interface the user wants to register a value for
    - `value [uint256]` the value user wants to register for a selected interface
- output:
    - None

*Key Query*: queries a key from the registry

- input:
    - account [account] Ethereum account number
    - interface [byte4] the interface the user wants to query
- output:
    - value [uint256] the value of the registry key (if any)

**Motivation**
Because we do not have access to the secret key, it appears impossible to generate a provably unique nullifier for each vote.  For example, if we used the Metamask signature of some (known) string as the nullifier, a malicious user could generate multiple different signatures of the same message. Therefore we have not found a way to prevent double voting without tying the nullifier to some on-chain data.

In fact, Ethereum's signatures are ECDSA signatures in which the ECDSA's randomness is computed deterministically as hash of the ethereum secret key's and message. So, even though two Metamask signatures for the same message are identical, a malicious user can still generate two different signatures of the same message by choosing the ECDSA's randomness in a malicious way (i.e., not as hash of secret key+message). Moreover, the fact that the ECDSA's randomness is computed as hash of secret key and message also makes hard to provide proofs of honest computation of such value: indeed, to prove that such value is honestly computed we would need the secret key as witness to the proof.


# Time Lock Cryptography Service (TLCS)

To ensure tally fairness and prevent premature vote counting, there are two primary options: a commit/reveal process without off-chain dependency, or encrypted ballots with a secret key reveal after voting ends. However, both options have drawbacks. To address this, we propose a public time-lapse cryptography service for various cryptographic schemes. The system will publish a specific public key for a chosen scheme and future date, with the corresponding private key set to be revealed on that date. This will maintain a constant supply of public keys, with new ones published and private keys uncovered regularly.

**Time Lock Service instantiation**
- League of Entropy current public key - [LOE Public API](https://drand.love/developer/http-api/#public-endpoints)
```
Example data from LOE Public API:
{
  "round":2888337,
  "randomness":"5f5cfa0b08343f04c418e8332be92a75088285df602741b75a4e8c1bda1a41db",
  "signature":"8a30c00d7ee4515d5bda5cfa4fe6d7896d2f9528549da49e0ce641b5d7ec41725e9143f0ba9bf3e5eecd08cb5873084701ae4e8589e786f2447fa42cf86a550881ee09d294025a69e38fd4e5a88e92b364446808f3a15b380c5d19eef03b74da",
  "previous_signature":"a521f6e642166ba6f5a62c6fcb0c1ed2b338892afdec29fb004f1e419adbea5835d45618249360b266e785324b273d22050de0c0fd06cb46fe63669c64cb0f14be9836753358962bb3fb7cf72de768e784fde99a9e88194cbb20287111027524"
}
// round is a monotonically increasing integer - the randomness round index
// randomness is a SHA-256 hash of the
// signature is the Boneh-Lynn-Shacham (BLS) signature for this round of randomness
// previous_signature is the signature of the previous round of randomness
```

**Blockchain Design Concept**
The Cosmos blockchain will use the EndBlocker to check if any LOE data has been scheduled to be retrieved and has reached the scheduled retrieval time. At said time, a thread will be spawned that will call the LOE API and do necessary processing. Once the processing is complete, a transaction will be generated and submitted back into the blockchain in the normal manner in which transactions are handled. The reason for the seperate thread is so that the chain progress will not be blocked.

We are currently exploring the possibilites of which node(s) will perform the LOE data retrieval. The possibilities are:
- All validator nodes
- A randomly chosen validator node
- The "lead" validator node at the given time

*Testing will be required to determine the optimum solution*

**Encryption Public Key Query:**
- input:
  - `decryption_time` UNIX timestamp at which one can decrypt the ballots
- output:
  - `encryption_public_key` An encryption public key used to encrypt the message for the future


**Decryption Key Registration:**
Used to publish new decryption key on the blockchain
- input:
    - `decryption_key` A decryption  key used to decrypt the message for the particular future
    - `decryption_time` UNIX timestamp for which we provide the decryption key
- output:
    - None if the `decryption_key` has been verified as valid for the specified `decryption_time`.

**Get Decryption Key:**
- input:
    - `decryption_time` UNIX timestamp that one wants a decryption key for
- output:
    - `decryption_key` A decryption  key for that particular UNIX timestamp (assuming the time is after the `decryption_time` of that key)


# Delay Relayer Service (DRS)

==To be defined in the comming weeks==



<style>
    /* CSS hack to add section numbers to titles */
    /* Titles numbers */
    .markdown-body {counter-reset: h1}
    .markdown-body h1 {counter-reset: h2}
    .markdown-body h2 {counter-reset: h3}
    .markdown-body h3 {counter-reset: h4}
    .markdown-body h4 {counter-reset: h5}
    .markdown-body h5 {counter-reset: h6}
    
    .markdown-body h1:before {counter-increment: h1; content: counter(h1) ". "}
    .markdown-body h2:before {counter-increment: h2; content: counter(h1) "." counter(h2) ". "}
    .markdown-body h3:before {counter-increment: h3; content: counter(h1) "." counter(h2) "." counter(h3) ". "}
    .markdown-body h4:before {counter-increment: h4; content: counter(h1) "." counter(h2) "." counter(h3) "." counter(h4) ". "}
    .markdown-body h5:before {counter-increment: h5; content: counter(h1) "." counter(h2) "." counter(h3) "." counter(h4) "." counter(h5) ". "}
    .markdown-body h6:before {counter-increment: h6; content: counter(h1) "." counter(h2) "." counter(h3) "." counter(h4) "." counter(h5) "." counter(h6) ". "}

    .markdown-body h1.nocount:before, .markdown-body h2.nocount:before, .markdown-body h3.nocount:before, .markdown-body h4.nocount:before, .markdown-body h5.nocount:before, .markdown-body h6.nocount:before { markdown-body: ""; counter-increment: none }
    .markdown-body h1:before, .markdown-body h2:before, .markdown-body h3:before, .markdown-body h4:before, .markdown-body h5:before, .markdown-body h6:before {
       color: #737373!important;
    }

    /* TOC numbers */
    .toc ul {
        list-style-type: none;
        counter-reset: css-counters 0;
    }

    .toc ul li:before {
        color: #919191!important;
        counter-increment: css-counters;
        content: counters(css-counters, ".") " ";
    }
</style>