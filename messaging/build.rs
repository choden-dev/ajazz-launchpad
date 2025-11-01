use protobuf_codegen::Customize;

fn main() {
    // Use this in build.rs
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(["protobufs"])
        // Inputs must reside in some of include paths.
        .input("protobufs/key_config.proto")
        .input("protobufs/common/keys.proto")
        .input("protobufs/common/inputs.proto")
        // Specify output directory relative to Cargo output directory.
        .out_dir("./src/protos")
        .customize(Customize::tokio_bytes(Customize::default(), true))
        .run_from_script();
}
