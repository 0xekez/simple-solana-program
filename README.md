# Solana Hello World Client

This is a better, Rust version of Solana's
[example-helloworld](https://github.com/solana-labs/example-helloworld/tree/master/src/client)
client program.

I made this because I felt as if the provided client implementation in
TypeScript lacked some detail in its comments and used too much global
state.

To use this client start by following the instructions in
example-helloworld to configure Solana on your machine and deploy the
example program to whatever cluster you choose to use. Once you've
done that you can run this client with `cargo run <path to example
program keypair>`.

The program is likely stored in a path similar to this one:

```
/home/admin/projects/solana/example-helloworld/dist/program/helloworld-keypair.json
```

## Status

This is still a work in progress as I'd like to eventually port the
entire hello world example over to this and improve on the conceptual
documentation provided in the Solana example.
