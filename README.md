# nouns-anonymous-voting

# ⛔ DISCLAIMER: This is a work in progress and does not fully work yet 💀

This repository consists of a library, CLI and smart contracts for the Nouns private voting project.

This is a work in progress. Please do not use this in production.

## Running the client

### Pre-requisites

1. Deploy the [Nouns voting contract](contracts/README.md) to an Ethereum network.
2. Copy the `.env.template` file to `.env` and fill in the values.
3. Have the Nouns CLI installed or have a source code to build it from.

If at any point of time you see that some of the environment variables are not being picked up, try
running `source .env` to load them into the current shell.

You can also always refer to the CLI's help manual by running it with the `--help` flag or any of its subcommands with
the `-h` flag.

### Reg-Key

This function is used to register a new private key inside the Zk-Registry contract. This is a one-time operation you
need to do before you can vote in any process.

For that, you need a Private Key (an arbitrary 32-byte value) that can be submitted as part of the command or set as an
environment variable. Refer to the `.env.template` file for more information.

1. To run from source with the private key set as an environment variable:

```bash
    nouns-cli reg-key
```

2. To run from source with the private key set as a command line argument:

```bash
    nouns-cli reg-key -k 043c3780cb30f913d1c34d80437f7c61c973461595986e899ee6a8171143db1d
```

### Create Process

This function is used to create a new voting process. It will return a process ID that you can use to refer to the
process in the future.

For that, you need to provide the following information:

1. The IPFS address of the proposal (raw binary format; sha2-256 hash).
2. The delay period of the voting process, which may be expressed in minutes, hours or days. If this argument is omitted, it is assumed to be 0.
3. The duration of the voting process, also expressed in minutes, hours or days.

_**Note** You will need an account that has at least one Nouns to participate in the voting process. If you are running
in a local test network, you can mint a Nouns to your account by running `cargo run --bin premint_nouns`
command._

1. To create a process with duration of 1 day:

```bash
    nouns-cli create-process -h -d 1d
```

2. To create a process with duration of 10 hours and a delay period of 1 hour:

```bash
    nouns-cli create-process -h  -s 1h -d 10h
```

### Vote

This function is used to vote in a process.

As part of the vote, you need to provide the following information:

1. The process ID of the process you want to vote in.
2. The NFT Index of the noun you want to vote for.
3. The Registry Private Key of the Account you want to vote with.
4. The Vote Option you want to vote for. This can be either `Yes`, `No` or `Abstain`.

_**Note:** make sure that the NFT indeed exists in the Nouns Token contract._

```bash
    nouns-cli vote -p 0 -n 0 -k 043c3780cb30f913d1c34d80437f7c61c973461595986e899ee6a8171143db1d -v y
```

### Tally

This function is used to tally the votes in a process.

For the tally, all you need to private is the process ID of the voting process you wish to tally.

**Note** That you can only run this command after the voting process has ended. If you are working on a local test net,
you can mine these blocks by running `cargo run --bin mine_blocks` command. Note that 1 block is counted as 12
seconds.

```bash
    nouns-cli tally -p 0
```


