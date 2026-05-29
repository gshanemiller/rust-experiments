use std::fs;                                                                                                            
use tinyjson::{JsonParser, JsonValue};                                                                                             
use super::parserutil::{ParserUtil};
use super::super::err::error::{Error};

pub trait Verify {
  fn verify(&self, obj: &JsonValue) -> Result<(), Error>;

  fn isMacAddress(&self, addr: &String) -> bool {
  }

  fn isIpv4Address(&self, addr: &String) -> bool {
  }

  fn isIpv6Address(&self, addr: &String) -> bool {
  }

  fn 

  fn parseFile(&self, fname: &str) -> Result<(), Error> {
    // Read JSON                                                                                                        
    let jsonResult = fs::read_to_string(fname);                                                                         
    let json = match jsonResult {                                                                                       
      Ok(val) => val,                                                                                                   
      Err(err) => { return Err(Error::Io(err)); }                                                                       
    };                                                                                                                  
                                                                                                                        
    // Parse JSON                                                                                                       
    let mut parser = JsonParser::new(json.chars());                                                                     
    let jsonObject = match parser.parse() {                                                                             
      Ok(val) => val,                                                                                                   
      Err(err) => { return Err(Error::JSONError); }                                                                     
    };                                                                                                                  
                                                                                                                        
    // Make sure it's an object                                                                                         
    if !jsonObject.is_object() {                                                                                            
      return Err(Error::JSONSchema);                                                                                    
    }                                                                                                                   

    // Do basic verification
    match self.verify(util, &jsonObject) {
      Ok(()) => {},
      Err(err) => { return Err(err); }
    };                                                                                                                  

    return Ok(());                                                                                                    
  }  
}
