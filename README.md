# Signalman

## Goals

Create a Rust interface for the Signal messaging app that can be used for account automation (bots, mailing lists, etc).

## Stages

### Stage 1 : Rust to Signal CLI

Implement Signalman in Rust as a client app interacting with a Signal CLI daemon using a Unix domain socket.

### Stage 2: Total oxidification

Implement Signalman purely in Rust, using the [libsignal](https://github.com/signalapp/libsignal) Rust library directly.

## References

### Rust Library Documentation

- [`serde`](https://docs.rs/serde_json/latest/serde_json/index.html) for serialization
- [`jsonrpc-types`](https://lib.rs/crates/jrpc-types) for JSON RPC standards
- [`clap`](https://kbknapp.github.io/clap-rs/clap/index.html) for command-line parsing
- [`tokio`](https://docs.rs/tokio/latest/tokio/index.html) for the async runtime
  - [Spawning](https://tokio.rs/tokio/tutorial/spawning)
  - [Channels](https://tokio.rs/tokio/tutorial/channels)

### Signal

- [`signal-cli`'s JSON RPC daemon MAN page](https://github.com/AsamK/signal-cli/blob/master/man/signal-cli-jsonrpc.5.adoc)
- [`libsignal` source code](https://github.com/signalapp/libsignal)

### Technologies

- [Unix sockets in Rust tutorial](https://emmanuelbosquet.com/2022/whatsaunixsocket/)

## Progress

- Got [Unix sockets](https://emmanuelbosquet.com/2022/whatsaunixsocket/) working!
  - Default Signal CLI socket appears to be `/run/user/1000/signal-cli/socket`.
  - [x] Able to read and deserialize JSONRPC notifications from a socket!
  - [x] Able to write JSONRPC to the socket and receive the response
  - [ ] Combine the two into one module, and show a listener that replies to messages
- The JSON RPC library we're using is `jrpc_types`, because we already need to use `serde_json` and we're writing a client, not a server.
