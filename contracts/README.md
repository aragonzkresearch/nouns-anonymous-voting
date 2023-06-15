# Nouns Smart Contract

This repository contains all the smart contracts for the Nouns Anonymous Voting project.

## Setup

To redeploy the contracts, first copy the `.env.template` file to `.env` and fill in the values.

Make sure you also have `forge` installed. You can read more about Foundry and Forge [here](https://book.getfoundry.sh/getting-started/installation)

## Deploying

In the example we will deploy the `ZKRegistry` contract to the `Sepolia` network.

_To deploy to a local test network, you should start it, for example using `anvil` and then provide the testnet rpc url, such as `http://127.0.0.1:8545`._

1. Deploy the `ZKRegistry` contract

```bash
 forge script script/0_ZKRegistry.s.sol:ZKRegistryScript --rpc-url $SEPOLIA_RPC_URL --broadcast --verify -vvvv
```

2. Deploy the `NounsToken` contract

```bash
 forge script script/1_NounsToken.s.sol:NounsTokenScript --rpc-url $SEPOLIA_RPC_URL --broadcast --verify -vvvv
```