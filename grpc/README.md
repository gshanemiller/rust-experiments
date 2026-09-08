# 
Developed with help from https://dockyard.com/blog/2025/04/08/grpc-basics-for-rust-developers. Note: documentation on which package(s) are required and what commands should appear in build.rs is in flux. Less politely documentation is uncharacteristically un-rust: it's sloppy, inconsistent

Example: https://crates.io/crates/tonic-build reads "tonic-build: Provides code generation for service stubs to use with tonic. For protobuf compilation via prost, use the tonic-prost-build crate instead." Then scroll down to read:

"fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/service.proto")?;
    Ok(())
}"

so which is it? tonic-build (the crate I'm reading) or tonic-prost-build? The dockyard build.rs like above example build.rs does not work with tonic-build.

#
This example works as-is by following these instructions:

1. clone this repo
2. cd grpc
3. run './scripts/rustrpc'. this adds deps, adds a build-dependency section to Cargo.toml, makes build.rs
4. run 'cargo build'
5. In one shell run 'cargo run --bin server'
6. Then in a second shell run 'cargo run --bin client'

Code generated files are written directly into Rust's cache in ./target
