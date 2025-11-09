## `messaging` workspace

Contains the shared components that allow the `backend-process` to communicate with any consumers.

### `Protobuf`

These messages are defined using [`protobuf`] and compiled to rust types for (de)serialization in any crate. There is
also the option for compiling to other languages; however, this is not used in this repo.

### `Client Wrapper`

This module (demonstrated by `client_sending_key_config_to_client`) helps to provide an api that any consumer that needs
to configure the launchpad can use. New commands are to be added here to make it clear what the server can accept.

### `Proto Builders`

Helper functions to avoid too much repeated code when creating protobuf objects. It is completely _optional_ to use.

### `Socket`

The `socket` module is a wrapper around the rust [unix sockets], and provides a `Client` and `Server` struct that
handles connection and the sending/receiving protocol internally. The consumer of these structs are responsible for
serializing and deserializing protobufs before sending/receiving

### Notes

The protobufs will automatically build as per `build.rs` when the `messaging` crate is compiled. The generated files are
placed in `src/protos` and re-exported as the `protos` module.

Other workspaces will consume this via:

```toml
messaging = { path = "../messaging" }
```

inside their `Cargo.toml`.


[`protobuf`]: https://protobuf.dev/reference/rust/rust-generated/

[unix sockets]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html