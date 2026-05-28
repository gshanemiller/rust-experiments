use std::fs;                                                                                                            
use tinyjson::{JsonParser, JsonValue};                                                                                             
use super::parserutil::{ParserUtil};
use super::super::err::error::{Error};

pub trait Verify {
  fn verify(&self, util: &ParserUtil, obj: &JsonValue) -> Result<(), Error>;
  fn numaVerify(&self, util: &ParserUtil, obj: &JsonValue) -> Result<(), Error>;
  fn crossVerify(&self, util: &ParserUtil, obj: &JsonValue) -> Result<(), Error>;
  fn createAllocators(&self, util: &ParserUtil, obj: &JsonValue) -> Result<(), Error>;

  fn parseFile(&self, fname: &str, util: &ParserUtil) -> Result<(), Error> {
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

    // Do NUMA verification
    match self.numaVerify(util, &jsonObject) {
      Ok(()) => {},
      Err(err) => { return Err(err); }
    };                                                                                                                  

    // Do cross verification
    match self.crossVerify(util, &jsonObject) {
      Ok(()) => {},
      Err(err) => { return Err(err); }
    };                                                                                                                  

    // Make allocators
    match self.createAllocators(util, &jsonObject) {
      Ok(()) => {},
      Err(err) => { return Err(err); }
    };                                                                                                                  
                                                                                                                        
    return Ok(());                                                                                                    
  }  
}
