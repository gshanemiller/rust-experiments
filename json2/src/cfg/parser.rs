use std::fs;
use tinyjson::{JsonParser};
use super::super::err::error::{Error};

pub struct Parser;

impl Parser {
  pub fn new(fname: &str) -> Result<bool, Error> {
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
    if !parsed.is_object() {
      return Err(Error::JSONSchema);
    }

    return Ok(true);
  }
}
