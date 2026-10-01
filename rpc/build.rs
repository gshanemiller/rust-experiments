use prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Protobuf codegen writing to Rust build cache
  let mut pbgen = prost_build::Config::new();
  pbgen
    .type_attribute(
      ".",
      "#[derive(serde::Serialize, serde::Deserialize)]"
    )
    .extern_path(
      ".google.protobuf.Any",
      "::prost_wkt_types::Any"
    )
    .compile_protos(
      &["protobuf/rpc.proto"],
      &["protobuf", "/home/smiller53/local/include"]
    ).unwrap();

  return Ok(());
}
