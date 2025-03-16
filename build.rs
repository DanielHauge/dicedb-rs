fn main() {
    tonic_build::compile_protos("protos/cmd.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
}
