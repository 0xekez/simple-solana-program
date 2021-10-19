# Solana Hello World Client

This is a better, Rust version of Solana's
[example-helloworld](https://github.com/solana-labs/example-helloworld/tree/master/src/client)
client program.

The client program that Solana uses by default is written in
TypeScript and uses a massive amount of global state. This makes the
code effectively unreadable even if the README in that repository
actually has a good high level explainer of what is going on.

To use this client start by following the instructions in
example-helloworld to configure Solana on your machine and deploy the
example program to whatever cluster you choose to use. Once you've
done that you can run this client with `cargo run <path to example
program keypair>`.

The program is likely stored in a path similar to this one:

````
/home/admin/projects/solana/example-helloworld/dist/program/helloworld-keypair.json
```

## Status

This is still a work in progress as I'd like to eventually port the
entire hello world example over to this and improve on the conceptual
documentation provided in the Solana example.
