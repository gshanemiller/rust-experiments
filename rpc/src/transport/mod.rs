use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct UserRequest {
  pub user_id: u32,
  pub status_code: u16,
  pub payload_value: u64,
}

impl UserRequest {
  fn serialize(&self) -> Vec<u8> {
    let mut buf = Vec::with_capacity(14);
    buf.extend_from_slice(&self.user_id.to_be_bytes());
    buf.extend_from_slice(&self.status_code.to_be_bytes());
    buf.extend_from_slice(&self.payload_value.to_be_bytes());
    buf
  }

  fn deserialize(buf: &[u8]) -> Result<Self, &'static str> {
    if buf.len() < 14 {
      return Err("Buffer too short for deserialization");
    }
        
    let user_id = u32::from_be_bytes(buf[0..4].try_into().unwrap());
    let status_code = u16::from_be_bytes(buf[4..6].try_into().unwrap());
    let payload_value = u64::from_be_bytes(buf[6..14].try_into().unwrap());

    Ok(UserRequest {
      user_id,
      status_code,
      payload_value,
    })
  }
}

pub fn run_server(addr: &str) -> std::io::Result<()> {
  let listener = TcpListener::bind(addr)?;
  println!("[SERVER] Listening on {}", addr);

  for stream in listener.incoming() {
    let mut stream = stream?;
    println!("[SERVER] New client connected: {:?}", stream.peer_addr());

    let mut buf = [0u8; 14]; // Expecting exactly 14 bytes

    match stream.read_exact(&mut buf) {
      Ok(_) => {
        match UserRequest::deserialize(&buf) {
          Ok(req) => {
            println!("[SERVER] Received RPC request: {:?}", req);
            // Process RPC (e.g., mutate status code to acknowledge receipt)
            let mut response = req;
            response.status_code = 200; 
                            
            // Send response back
            let response_bytes = response.serialize();
            if let Err(e) = stream.write_all(&response_bytes) {
              println!("[SERVER] Failed to send response: {}", e);
            } else {
              println!("[SERVER] Sent RPC response back.");
            }
          },
          Err(e) => {
            println!("[SERVER] Deserialization error: {}", e);
          },
        }
      },
      Err(e) => {
        println!("[SERVER] Read error: {}", e);
      },
    }
  }
  Ok(())
}

pub fn run_client(addr: &str) -> std::io::Result<()> {
  let mut stream = TcpStream::connect(addr)?;
  println!("[CLIENT] Connected to server at {}", addr);

  // Create the RPC payload
  let req = UserRequest {
      user_id: 1337,
      status_code: 100,
      payload_value: 999999999,
  };
  println!("[CLIENT] Sending RPC request: {:?}", req);

  // Serialize and send
  let bytes = req.serialize();
  stream.write_all(&bytes)?;

  // Read the server response
  let mut buf = [0u8; 14];
  stream.read_exact(&mut buf)?;

  match UserRequest::deserialize(&buf) {
    Ok(res) => println!("[CLIENT] Received RPC response from server: {:?}", res),
    Err(e) => eprintln!("[CLIENT] Deserialization error: {}", e),
  }

  Ok(())
}
