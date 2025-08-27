# Signalman

## Goals

Create a Rust interface for the Signal messaging app that can be used for account automation (bots, mailing lists, etc).

## Stages

### Stage 1 : Rust to Signal CLI

Implement Signalman in Rust as a client app interacting with a Signal CLI daemon of some sort on the same machine.

### Stage 2: Rust to Rust

Implement Signalman in Rust, using the [libsignal](https://github.com/signalapp/libsignal) Rust library directly.

## References

- [`signal-cli`'s JSON RPC daemon MAN page](https://github.com/AsamK/signal-cli/blob/master/man/signal-cli-jsonrpc.5.adoc)
- [Documentation for Rust library `serde`](https://docs.rs/serde_json/latest/serde_json/index.html)
- [Documentation for Rust library `jsonrpc-types`](https://lib.rs/crates/jrpc-types)
- [`libsignal` source code](https://github.com/signalapp/libsignal)

## Progress

- Got [Unix sockets](https://emmanuelbosquet.com/2022/whatsaunixsocket/) working!
  - Default Signal CLI socket appears to be `/run/user/1000/signal-cli/socket`.
  - [x] Able to read and deserialize JSONRPC notifications from a socket!
  - [ ] Figure out how to easily write JSONRPC to the socket
- The JSON RPC library we're using is `jrpc_types`, because we already need to use `serde_json` and we're writing a client, not a server.
