use rpc::transport;

fn main() {
  let default_addr = "127.0.0.1:7999";

  if let Err(e) = transport::run_server(default_addr) {
    println!("[SERVER] error: {}", e);
  }
}
