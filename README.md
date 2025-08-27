# Signalman

## References

- <https://github.com/AsamK/signal-cli/blob/master/man/signal-cli-jsonrpc.5.adoc>
- <https://learn.microsoft.com/en-us/dotnet/fundamentals/networking/sockets/socket-services>

## Bad Paths

- `jsonRpc`: Spinning up the JVM every single time is going to kill performance.
- `daemon --dbus`: No language I know has good bindings for DBUS and I couldn't get code generation for C# DBUS bindings to work. This is also experimental.
- `daemon --socket`: No good documentation on Unix domain sockets in Dotnet.
- `daemon --tcp 0.0.0.0:7583`: Can't figure out how to read from it.

## Stages

### Stage 1 : Rust around HTTP

Implement Signalman in Rust as a HTTP client app interacting with a SignalCli daemon.

### Stage 2: Rust on Rust

Implement Signalman in Rust, using the [libsignal](https://github.com/signalapp/libsignal) Rust library directly.
