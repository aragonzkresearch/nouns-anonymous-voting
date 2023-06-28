# nouns-voting-proofs

This crate consists of

- the client-side anonymous voting application implementation, and
- the server-side tally proof generator
  for the Nouns private voting project.

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

1. To create a process with duration of 1 day:

```bash
    cargo run -- create-process -d 1d -t '0x0882c07dfb863de7cb769152e581f987b01f723d3cf9a00b3801fd3c206b9537, 0x1f3179c62406bf009ae22a0b15d8d5cf156b9d6945c23aabedea2def1d929364'`
```

2. To create a process with duration of 10 hours:

```bash
    cargo run -- create-process -d 10h -t '0x0882c07dfb863de7cb769152e581f987b01f723d3cf9a00b3801fd3c206b9537, 0x1f3179c62406bf009ae22a0b15d8d5cf156b9d6945c23aabedea2def1d929364'`
```

