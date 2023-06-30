# nouns-anonymous-voting

# ⛔ DISCLAIMER: This is a work in progress and does not fully work yet 💀

This repository consists of a library, CLI and smart contracts for the Nouns private voting project.

This is a work in progress. Please do not use this in production.

## Running the client

### Pre-requisites

1. Deploy the [Nouns voting contract]() to a Ethereum network.
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
    cargo run -- reg-key
```

2. To run from source with the private key set as a command line argument:

```bash
    cargo run -- reg-key -k 043c3780cb30f913d1c34d80437f7c61c973461595986e899ee6a8171143db1d
```

### Create Process

This function is used to create a new voting process. It will return a process ID that you can use to refer to the
process in the future.

For that, you need to provide the following information:

1. The TLCS Public Encryption Key that will be used to encrypt the votes. You should get this from the TLCS server.
2. The duration of the voting process. This can be provided in minutes, hours and days.

**Note** You will need an account that has at least one Nouns to participate in the voting process. If you are running
in a local test network, you can mint a Nouns to your account by running `cargo test -- premint_nouns --nocaputre`
command.

1. To create a process with duration of 1 day:

```bash
    cargo run -- create-process -d 1d -t '234056D968BAF183FE8D237D496D1C04188220CD33E8F8D14DF9B84479736B20,2624393FAD9B71C04B3B14D8AC45202DBB4EAFF4C2D1350C9453FC08D18651FE'
```

2. To create a process with duration of 10 hours:

```bash
    cargo run -- create-process -d 10h -t '234056D968BAF183FE8D237D496D1C04188220CD33E8F8D14DF9B84479736B20,2624393FAD9B71C04B3B14D8AC45202DBB4EAFF4C2D1350C9453FC08D18651FE'
```

### Vote

This function is used to vote in a process.

As part of the vote, you need to provide the following information:

1. The process ID of the process you want to vote in.
2. The NFT Index of the noun you want to vote for.
3. The Registry Private Key of the Account you want to vote with.
4. The TLCS Public Encryption Key that will be used to encrypt the vote. You should get this from the TLCS server.
5. The Vote Option you want to vote for. This can be either `Yes`, `No` or `Abstain`.

**Note:** make sure that the NFT indeed exists in the Nouns Token contract.

```bash
    cargo run -- vote -p 0 -n 0 -k 043c3780cb30f913d1c34d80437f7c61c973461595986e899ee6a8171143db1d -v y -t '234056D968BAF183FE8D237D496D1C04188220CD33E8F8D14DF9B84479736B20,2624393FAD9B71C04B3B14D8AC45202DBB4EAFF4C2D1350C9453FC08D18651FE'
```

### Tally

This function is used to tally the votes in a process.

As part of the tally, you need to provide the following information:

1. The process ID of the process you want to tally.
2. The TLCS private key that will be used to decrypt the votes. You should get this from the TLCS server.

**Note** That you can only run this command after the voting process has ended. If you are working on a local test net,
you can mine these blocks by running `cargo test -- mine_blocks --nocapture` command. Note that 1 block is counted as 12
seconds.

```bash
    cargo run -- tally -p 0 -t 059D6B0FE7AD950D220261FE28B7C8B514E3B06D8EBC17179C469120A366B8C9
```


