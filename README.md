# Signalman

## Goals

Create a Rust interface for the Signal messaging app that can be used for account automation (bots, mailing lists, etc).

## Stages

### Stage 1 : Rust to Signal CLI

Implement Signalman in Rust as a client app interacting with a Signal CLI daemon of some sort on the same machine.

### Stage 2: Rust on Rust

Implement Signalman in Rust, using the [libsignal](https://github.com/signalapp/libsignal) Rust library directly.

## References

- <https://github.com/AsamK/signal-cli/blob/master/man/signal-cli-jsonrpc.5.adoc>
- <https://github.com/signalapp/libsignal>

## Next Steps

- Now that we're in Rust, try to get [Unix sockets](https://emmanuelbosquet.com/2022/whatsaunixsocket/) working. That would be much more appropriate than a HTTP server over localhost and would almost definitely be faster.
  - Reading from the socket would replace SSE with JSONRPC notifications
  - Writing to the socket would still require JSONRPC
- Naturally, there's a [library for JSONRPC](https://docs.rs/jsonrpc/latest/jsonrpc/) that works with UDS! Try to get that working.
