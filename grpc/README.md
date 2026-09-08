# Usage
This example works as-is by following these instructions:

1. clone this repo
2. cd grpc
3. run './scripts/rustrpc'. this adds deps, adds a build-dependency section to Cargo.toml, makes build.rs
4. run 'cargo build'
5. In one shell run 'cargo run --bin server'
6. Then in a second shell run 'cargo run --bin client'

# Build.rs
This code fragment:

```
  tonic_prost_build::configure().compile_protos(
    &["protobuf/rpc.proto"],
    &["protobuf", "/home/smiller53/local/include"]).unwrap();
```

gives an array of protobuf files to codegen in the first arguement (here `protobuf/rpc.proto`), and an array of directories to look in to resolve dependent protobuf files or types (here `protobuf, /home/smiller53/local/include`).

# Details
1. Code generated files are written directly into Rust's cache in ./target
2. The instructions in the previous section assume `protoc` was pre-installed, and can be found in $PATH by running `which protoc`. All you need to do is download a prebuilt binary from `https://github.com/protocolbuffers/protobuf/releases/tag/v36.1`. My version is `libprotoc 34.0
3. The instructions in the previous section do NOT require `protoc-gen-rust-grpc` was pre-installed
4. If you protobuf references non-trivial types like Duration, Any, or messages provided in other files  you will need to include a directory that holds those definitions in the second argument to `tonic_prost_build`. The installation of `protoc` comes with a directory called `google` which I've located under `/home/smiller53/local/include` to resolve Google specific types like Any. Other custom messages presumably appear in their own protobuf files located in the `protobuf` directory together with `protobuf/rpc.proto`

# Watch out for the noise
Developed with help from https://dockyard.com/blog/2025/04/08/grpc-basics-for-rust-developers. Note: documentation on which package(s) are required and what commands should appear in build.rs is in flux. Less politely documentation is uncharacteristically un-rust: it's sloppy, inconsistent

Example: https://crates.io/crates/tonic-build reads "tonic-build: Provides code generation for service stubs to use with tonic. For protobuf compilation via prost, use the tonic-prost-build crate instead." Then scroll down to read:

"fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/service.proto")?;
    Ok(())
}"

so which is it? tonic-build (the crate I'm reading) or tonic-prost-build or prost? Guys, get your darn story straight!

# Task File sizes

```
3.3M  target/release/server
2.9M  target/release/client
43M   target/debug/server
33M   target/debug/client
```

The release sizes are not too terrible considering the number of imported dependencies for a trivial textbook example 
