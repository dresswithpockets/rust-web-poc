use tonic_build;

fn main() {
    tonic_build::compile_protos("model/idgen.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
