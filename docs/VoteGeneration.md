# Nouns voter proof generation
> Assumptions:
> - EthStorageProof **not** feasible in the browser
> - EthStorageProof is feasible in a server
>
> If EthStorageProof ends up being feasible in the browser, the same scheme & code can be run directly in the browser.
>
> Simple solution: server trusted for privacy but not integrity. Integrity means that server cannot vote on behalf of voter in the current election for a different voting option and in any future election on any voting option. Later on we show how to achieve full privacy against a malicious server.



Recall that in the ZK registry we have $RCK=g^{RK}$, $PK_t$ represent the TLCS public key for a given time $t$ used for the election, and $R$ is the root of the Ethereum state used for the election.
Henceforth, $id=\{chain_{ID}, process_{ID}, contract_{ADDR}\}$

The voter computes:
- Signatures $\sigma=DS.Sign(RK,(NFT_{id},id)),~ \tau=DS.Sign(RK,v),$ where $v$ is the voter's preference and $RK$ is used as signing key (so that $RCK$ will be the corresponding verification key).
- nullifier $N=Poseidon(\sigma, id)$.
- $A=g^{r}$,$K =PK_{t}^{r},$ for some randomness $r\in Z_p$. (note that this is equal to $g^{r\cdot sk}$).
- $B=H(K, v, id).$
- $H_{id}=Poseidon(NFT_{id},id)$
- The path $p_1$ from the root $R$ to the relevant information needed to prove ownership of the token $NFT_{id}$ and the path $p_2$ to the registry commitment key $RCK_i$, and the path(s) $p_3$ needed to prove that $NFT_{id}$ is not delegated.
  The voter sends to the server the tuple $(RCK,N,id,NFT_id,H_{id},r,v,A,K,B,\sigma,\tau,p_1,p_2,p_3)$.

> The Sign algorithm is for a DS scheme that has the following property: it is hard for an adversary to produce two different signatures of the same message (BLS and RSA have this property).
> In other words, $\sigma$ is for a deterministic unique signature. Alternatively we can use PLUME in future. For the moment we will use EdDSA.


Consider the following Noir program $P$.
$P$ has public inputs $(A,B,N,H_{id},id,R)$ and as witness $(v,\sigma,address,\tau, NFT_id,RCK,p_1,p_2,p_3)$.
$P$ does the following.
1. Check that $DS.Ver(RCK,\sigma,(NFT_{id},id))=1$, that is that $\sigma$ is a signature of $id$ under pubk $RCK$.
2. Check that $DS.Ver(RCK,\tau,v)=1$, that is that $\tau$ is a signature of $v$ under pubk $RCK$.
3. Check that $H_{id}=Poseidon(NFT_{id},id)$.
4. Check that $N=Poseidon(\sigma,id)$.
5. Correct encryption of the vote:
    - 4.1. $g^{r}=A$ and $K=PK_{t}^{r}$.
    - 4.2. $B =Poseidon(K, v,id)$.
    - 4.3. $v\in\{0,1,2\}$.
6. Use the path $p_1$ to check that the Ethereum's state committed to in $R$ includes in the ZK registry $RCK_i$ that is associated with an Ethereum's user with address $address$.
7. Use the path $p_2$ to check that the Ethereum's state committed to in $R$ contains a token with identifier $NFT_{id}$ owned by an address $address$ ($addr$ holds the $NFT_{id}$).
8.  Similarly to before use $p_3$ to check that $NFT_{id}$ is not delegated.
9. ==TODO== check that signature randomness is deterministic $r = Poseidon(msg ~||~ Poseidon(RK))$


> Note: if we can remove "delegation" from the scope, step 6. is not needed (1 EthStorageProof's opening less).

The server using the public inputs and the witnesses computes a proof $\pi$ and sends back to the voter $\pi$.

The voter sends to the smart contract $(A,B,N,\pi)$.
The smart contract keeps a value $B_K$ that at beginning is null. If the ballot $(A,B,N,\pi)$ is the first ballot received with a valid proo, then the smart contract sets $B_K=B$. If the ballot $(A,B,N,\pi)$ comes with a valid proof but is not the first received the smart contract sets $B_K=Keccac(B_K,B)$.

If $N$ voters submitted valid proofs we call $(A_i,B_i,N_i,\pi_i)$, for $i\in[N]$, the values such voter sent to the smart contract.
# Nouns tally proof generation and verification


++Known paramters:++
- $t$: time to decrypt votes, kown in the contract
- $PK_{t}$: TLCS public key for time t
- identifier $id$ as previously defined.

++Before generating the proof:++
- Fetch $A_i$ for $\forall i \in \{1, \ldots, n\}$
- Fetch $B_i$ for $\forall i \in \{1, \ldots, n\}$
- Fetch secret key of TLCS $sk_{t}$
- Fetch value $B_K$ from the smart contract.

++Get option for a voter $i$:++
- Compute $A_i^{sk_{t}} = g^{r_i sk_t} = K_i$.
- Find the first value $v_i\in \{0,1,2\}$ such that
    <!-- OLD - $K_i'=H_4(K_i)$, where $K_i = e(T_{sign}, A_i)$ -->
    - $B_i = Poseidon(K_i, v_i, id)$. (We will be able to find such value $v_i$ because the voter's ZK proof was verified succesfully.)

++Prove vote aggregation:++
- Sum all $v_i$ for each vote option to compute an array $vote_{count}$ storing # votes for, # votes against, # votes abstain.
- Given public inputs $B_K$, $chain_{ID}$, $process_{ID}$, $contract_{ADDR}$, $vote_{count}$ and witnesses $(K_i,v_i)$ we generate a zk proof of the following prorogram:
    - For all $i\in[n]$, the program computes $B_i =Poseidon(K_i, v_i, chain_{ID}, process_{ID}, contract_{ADDR})$
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
### Implementation details
We instantiate our construction working on the Babyjubjub EC that is SNARK friendly for SNARKs that over over the BN254 bilinear group and with the EdDSA signature scheme (with step $9$ in the Noir voter circuit added to guarantee the unique signature property).

## Future work: Achieving full privacy against the server
The idea is simple.
Consider the previous design without the encryption layer.
The voter invokes the previous procedure $3$ times for each voting option $v_{i,0}=0,v_{i,1}=1,v_{i,2}=2$.

The voter will get respectively $3$ proofs $\pi_{i,0},\pi_{i,1},\pi_{i,2}$ from the server and re-randomizes them to get proofs respectively $\pi_{i,0}',\pi_{i,1}',\pi_{i,2}'$.
The voter sends to the smart contract $(v_i,N_i,\pi_{i,v_i}')$, where $v_i$ is his actual voting option.
The problem with this approach is that the server can cast the ballot on behalf of the voter before the voter gets any proof.
The easy patch is the following. The voter chooses a random value $s_{i,v}$ for each voting option $v\in\{0,1,2\}$ and sets $y_{i,v}=Keccac(s_{i,v})$.
Signature $\tau_{i,v}$ that before was equal to $DS.Sign(RK_i,v_i)$ is now changed to:
- $\tau_{i,v}=DS.Sign(RK_i,(v,y_{i,v}))$, that is the signature signs both the voting option $v$ and the value $y_{i,v}$.
For each of the $3$ invocations for $v=0,1,2$ of the server, the voter also sends the value $y_{i,v}$ to the server and the server uses such value as additional public input for the Noir program that is identical to before except for the way the signature $\tau_{i,v}$ is verified in step $2$:
- 2. Check that $DS.Ver(RCK_i,\tau_i,(v,y_{i,v}))=1$, that is that $\tau_i$ is a signature of $(v,y_{i,v})$ under pubk $RCK_i$.
The voter sends to the smart contract $(v_i,N_i,y_{i,v_i},s_{i,v_i},\pi_{i,v_i}')$, where $v_i$ is his actual voting option. As usual the smart contract checks the proof with respect to the public inputs $(v_i,N_i,y_{i,v_i})$ but in addition also checks that $y_{i,v_i}=Keccac(s_{i,v_i}).$
The hardness of inverting the Keccac output makes hard for a malicious server to cast votes on behalf of the voter.
### Re-randomizable vs non-re-randomizable proofs.
The previous protocol requires re-randomizable proofs. Groth16 proofs are re-randomizable but proofs obtained by making IOP non-interactive using the FS heuristic are usually not re-randomizable. Question: should it be possible to adapt the protocol so to be able to re-randomize proofs? Hint: maybe we need a bit of interaction between the voter and the server? If instead there is no way to use re-randomizable proofs then recursive proofs come to rescue. The voter could sends to the smart contract the public inputs and the hash of the proof obtained from the server and prove that the preimage of this hashed value is a proof that is verified for the public inputs.

Note: we achieve full privacy but not anonymity because the server will know the identity of the voter. Can we also achieve anonymity against the server? This is theoretically possible.
