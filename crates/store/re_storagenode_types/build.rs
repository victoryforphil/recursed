
fn main() {
    tonic_build::compile_protos("src/proto/storage_node.proto").unwrap();
}
