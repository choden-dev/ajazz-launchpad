## `messaging` workspace

Contains the shared components that allow the `backend-process` to communicate with any consumers.

These messages are defined using [`protobuf`] and compiled to rust types for (de)serialization in any crate. There is
also the option for compiling to other languages; however, this is not used in this repo.

### Notes

The protobufs will automatically build as per `build.rs` when the `messaging` crate is compiled. The generated files are
placed in `src/protos` and re-exported as the `protos` module. 

Other workspaces will consume this via:

```toml
messaging = { path = "../messaging" }
```

inside their `Cargo.toml`.



[`protobuf`]: https://protobuf.dev/reference/rust/rust-generated/
