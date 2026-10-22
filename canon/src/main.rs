use std::io;
use std::fs;

fn canonicalFileName(inFilePath: &String) -> std::io::Result<String> {                                            
  // Change any '\' to '/'                                                                                            
  let unixPath = inFilePath.replace('\\', "/");                                                                       
  let trimPath = unixPath.trim();
  // Strip leading '/' and '.'                                                                                        
  let mut i: usize = 0;                                                                                                
  for ch in trimPath.chars() {
    if ch=='/' || ch=='.' {
      i += 1;
    } else {
      break;
    }
  }
  if i<trimPath.len() {
    return Ok(trimPath[i..].to_string());
  }
  return Err(io::Error::new(io::ErrorKind::NotFound, "bad filename"));
}                                                                                                                     

fn main() {
  {
    let s = String::from("  //\\     ");
    match canonicalFileName(&s) {
      Ok(o) => println!("ok in='{}' out='{}'", s, o),
      Err(e) => println!("fail in='{}' {:?}", s, e),
    }
    match fs::canonicalize(s.as_str()) {
      Ok(p) => println!("ok in='{}' out='{:?}'", s, p),
      Err(e) => println!("fail in='{}' {:?}", s, e),
    }
  }

  {
    let s = String::from("abc.txt");
    match canonicalFileName(&s) {
      Ok(o) => println!("ok in='{}' out='{}'", s, o),
      Err(e) => println!("fail in='{}' {:?}", s, e),
    }
    match fs::canonicalize(s.as_str()) {
      Ok(p) => println!("ok in='{}' out='{:?}'", s, p),
      Err(e) => println!("fail in='{}' {:?}", s, e),
    }
  }

  {
    let s = String::from("./abc.txt   ");
    match canonicalFileName(&s) {
      Ok(o) => println!("ok in='{}' out='{}'", s, o),
      Err(e) => println!("fail in='{}' {:?}", s, e),
    }
    match fs::canonicalize(s.as_str()) {
      Ok(p) => println!("ok in='{}' out='{:?}'", s, p),
      Err(e) => println!("fail in='{}' {:?}", s, e),
    }
  }

  {
    let s = String::from("a/./bc.txt   ");
    match canonicalFileName(&s) {
      Ok(o) => println!("ok in='{}' out='{}'", s, o),
      Err(e) => println!("fail in='{}' {:?}", s, e),
    }
    match fs::canonicalize(s.as_str()) {
      Ok(p) => println!("ok in='{}' out='{:?}'", s, p),
      Err(e) => println!("fail in='{}' {:?}", s, e),
    }
  }
}
