# Simple Solana Smart Contract Example

This repository demonstrates how to create and invoke a program on the
Solana blockchain. The program used is the same one used by Solana's
hello world
[example](https://github.com/solana-labs/example-helloworld). This
differs from that example though in that the client program which
executes the Solana program is written in Rust.

The Solana program provided here counts the number of times that it
has been executed and stores that information on chain.

## Getting started

In order to run this example program you will need to install Rust and
Solana. Information about installing Rust can be found
[here](https://rustup.rs/) and information about installing Solana can
be found [here](https://docs.solana.com/cli/install-solana-cli-tools).

Once you've completed the Solana installation run the following
commands to configure you machine for local development:

```
solana config set --url localhost
solana-keygen new
```

These two commands create Solana config files in `~/.config/solana/`
which solana command line tools will read in to determine what cluster
to connect to and what keypair to use.

Having done that run a local Solana validator by running:

```
solana-test-validator
```

This program must be left running in the background.

## Deploying the Solana program

To deploy the Solana program in this repository to the Solana cluster
that you have configured run:

```
./run.sh deploy
```

## Running the client program

To run the client program you must have already deployed the Solana
program. The client program sends a transaction to the Solana
blockchain asking it to execute the deployed program and reports the
results.

```
./run.sh client
```

Will build and execute the client program. You ought to see results
that look something like this:

```
Connected to remote solana node running version (1.7.16).
(1418720) lamports are required for this transaction.
(499999997700801080) lamports are owned by player.
creating greeting account
(1) greetings have been sent.
```

On future executions you will see that the greetings counter
increases.

```
(2) greetings have been sent.
```
