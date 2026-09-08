fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Protobuf translation into build cache
  tonic_prost_build::configure().compile_protos(
    &["protobuf/rpc.proto"],
    &["protobuf", "/home/smiller53/local/include"]).unwrap();
  return Ok(());
}
