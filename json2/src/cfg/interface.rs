use std::fs;
use tinyjson::{JsonParser, JsonValue};
use crate::err::error::{Error};

pub trait Verify {
  fn verify(&self, obj: &JsonValue) -> Result<(), Error>;

  fn isMacAddress(&self, addr: &String) -> Result<(), Error> {
    let mut ret = true;
    let mut numbCount = 0;
    for num in addr.split(":") {
      numbCount += 1;
      let parseResult = u32::from_str_radix(num, 16);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => { return Err(Error::Num(err)); }
      };
      // MAC in [0,255] by index
      ret = ret && val<=255;
    }

    // MAC is four ints
    if ret && numbCount==6 {
      return Ok(());
    } else {
      return Err(Error::InvalidFormat);
    }
  }

  fn isIpv4Address(&self, addr: &String) -> Result<(), Error> {
    let mut ret = true;
    let mut numbCount = 0;
    for num in addr.split(".") {
      numbCount += 1;
      let parseResult = u32::from_str_radix(num, 10);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => { return Err(Error::Num(err)); }
      };
      // IPV4 in [0,255] by index
      ret = ret && val<=255;
    }

    // IPV4 is four ints
    if ret && numbCount==4 {
      return Ok(());
    } else {
      return Err(Error::InvalidFormat);
    }
  }

  fn isIpv6Address(&self, addr: &String) -> Result<(), Error> {
    let mut ret = true;
    let mut numbCount = 0;
    for num in addr.split(":") {
      numbCount += 1;
      let parseResult = u32::from_str_radix(num, 16);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => { return Err(Error::Num(err)); }
      };
      // IPV6 in [0,65535] by index
      ret = ret && val<=0xffff;
    }

    // IPV6 is eight ints
    if ret && numbCount==8 {
      return Ok(());
    } else {
      return Err(Error::InvalidFormat);
    }
  }

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

    // Parse + verify
    match self.verify(&jsonObject) {
      Ok(()) => {},
      Err(err) => { return Err(err); }
    };

    return Ok(());
  }
}
