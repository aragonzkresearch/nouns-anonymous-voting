# Nouns Smart Contract

This repository contains all the smart contracts for the Nouns Anonymous Voting project.

## Setup

To redeploy the contracts, first copy the `.env.template` file to `.env` and fill in the values.

Make sure you also have `forge` installed. You can read more about Foundry and
Forge [here](https://book.getfoundry.sh/getting-started/installation).

## Deploying

### Local Network

To deploy the full set of contracts to your local network, first start a local network using `anvil`.

```bash
  anvil
```

Then set the `PRIVATE_KEY` value in the `.env` file to the private key of one of the accounts that `anvil` created.

Lastly, deploy the contracts using `forge`:

```bash
source .env
 forge script script/2_NounsVoting.s.sol:NounsVotingDeployScript --fork-url http://127.0.0.1:8545 --broadcast --verify -vvvv
```

### Sepolia Network

**Note:** Make sure you have the correct `SEPOLIA_RPC_URL` set in your `.env` file, as well as the `PRIVATE_KEY` maps to
an account that has enough funds to deploy the contracts.

Run the following commands to deploy the contracts:

```bash
source .env

forge script script/2_NounsVoting.s.sol:NounsVotingDeployScript --rpc-url $SEPOLIA_RPC_URL --broadcast --verify -vvvv
```

### Mainnet

To deploy the contract to mainnet, populate the `.env` file with the correct values and run the following command:

```bash
forge script script/3_NounsMainnetVoting.s.sol:NounsMainnetVotingDeployScript --rpc-url $MAINNET_RPC_URL --broadcast --verify -vvvv
```