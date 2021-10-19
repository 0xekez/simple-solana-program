# Simple Solana Smart Contract Example

This repository demonstrates how to create and invoke a program on the
Solana blockchain. In Solana the word program is used to describe what
is often described as a smart contract in other contexts.

The Solana program provided here counts the number of times that it
has been executed and stores that information on chain. It is
functionally identical to Solana's hello world
[example](https://github.com/solana-labs/example-helloworld).

This program differs from the Solana example code in that the client
which invokes the Solana program is written in Rust. Contrary to what
you might guess the most complicated part of writing a smart contract
in Solana is the client.

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

## How this works

This repository is divided into two parts. There is a `program/`
directory which contains the smart contract that is actually deployed
to the Solana blockchain and a `client/` program which handles
collecting funds, creating accounts, and invoking the deployed
program.

Details about how the deployed program works can be found
[here](https://github.com/solana-labs/example-helloworld#learn-about-the-on-chain-program). The
actually interesting part of this repository is the client which I'll
discuss below.

## The client program

In order to execute a program that has been deployed to the Solana
blockchain we need the following things:

1. A connection to a Solana cluster
2. An account with a large enough balance to pay for the program's
   execution.
3. An account that we will transfer to the program to store state for
   the program.

The third item on that list is confusing. In Solana programs are
entirely stateless. Instead, they operate on accounts and store data
in those accounts between executions as needed.

Accounts are a really bad name for files. You can read the technical
details
[here](https://docs.solana.com/developing/programming-model/accounts).

In order for a program to modify the contents of an account that
program must own the account. This is the reason for the "transfer"
part of the third item above. Our client program will create an
account to store its program state in and then transfer ownership of
that account to the program. The client will then later read the data
in the account to see the results of the program's execution.

Why does it have to be like this? This seems like pain. The reason is
that storing data on the blockchain costs money. The user of the
program is expected to pay for the cost of running the program and so
they must pay to create the account.

## The technical details

I'll now take some time to walk through the technical details of how
the client collects what is needed and then submits the transaction to
Solana.

### Connection establishment

The function `establish_connection` in `client/src/client.rs` creates
a RPC connection to Solana over which we'll do all of our
communication with the blockchain. The URL that we use for connecting
to the blockchain is read from the Solana config in
`~/.config/solana/cli/config.yml`. It can be changed by running
`solana config set --url <URL>`.

### The data that we will be storing

Our program that we will deploy tracks the number of times that a
given user has said hello to it. This requires a small amount of state
which we represent with the following Rust structure:

```rust
struct GreetingSchema {
    counter: u32,
}
```

In order to make sure that this data can be serialized and
unserialized independently to how Rust lays out a struct like that we
use a serialization protocol called [borsh](https://borsh.io/). We can
determine the size of this after serialization by serializing an
instance of this struct with borsh and then getting its length.

### Determining the balance requirement

To determine how much the program invocation will cost we use the
function `get_balance_requirement` located in
`client/src/client.rs`. The total cost of the invocation will be the
cost of submitting the invocation transaction and the cost of storing
the program state on the blockchain.

On Solana the cost of storing data on the blockchain is zero if that
data is inside an account with a balance greater than the cost of two
years of rent. You can read more about that
[here](https://docs.solana.com/implemented-proposals/rent).

It appears as if the standard is to load two years of rent into
accounts so that is what we do. Source: the "programs and accounts"
section of
[this](https://2501babe.github.io/posts/solana101.html#programs%20and%20accounts)
writeup. We can determine what this "two years of rent" amount is by
running `connection.get_minimum_balance_for_rent_exemption(data_size)`
where `data_size` is the amount of data that we will be storing in
bytes.

### Determining the payer and their balance

In order to determine who will be paying for this transaction we once
again consult the solana config in
`~/.config/solana/cli/config.yml`. In that file there is a
`keypair_path` field and we read the keypair from where that points.

To determine the payer's balance we use our connection and run
`connection.get_balance(player_pubkey)`.

### Increasing the balance if there are not enough funds

If there are not enough funds in the payer's account we will need to
airdrop funds there. This is done via the `request_airdrop` function
in `client/src/client.rs`. Airdrops are only available on test
networks so this will not work on the mainnet.

### Locating the program that we will execute

Both the payer and the program are accounts on Solana. The only
difference is that the program account is executable.

Our client program takes a single argument which is the path to the
keypair of the program that has been deployed to the blockchain. In
the `get_program` function located in `client/src/client.rs` the
keypair is loaded again and then it is verified that the specified
program is executable.

## Creating an account to store state

## Sending the "hello" transaction

## Querying the account that stores state
