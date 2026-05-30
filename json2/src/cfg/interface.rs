use std::fs;
use crate::err::error;
use tinyjson::{JsonParser, JsonValue};

pub trait Verify {
  fn verify(&self, obj: &JsonValue) -> Result<(), error::Error>;

  fn isPciAddress(&self, addr: &String) -> Result<(), error::Error> {
    let mut ret = true;
    let mut numbCount = 0;
    for num in addr.split(&[':','.']) {
      numbCount += 1;
      let parseResult = u32::from_str_radix(num, 16);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => { return Err(error::Error::Num(err)); }
      };
      if numbCount==1 {
        ret = ret && val<=65535;
      } else if numbCount>=1 && numCount<=3 {
        ret = ret && val<=255;
      } else {
        ret = ret && val<=15;
      }
    }

    // PCI is four ints
    if ret && numbCount==4 {
      return Ok(());
    } else {
      return Err(error::Error::InvalidFormat);
    }
  }

  fn isMacAddress(&self, addr: &String) -> Result<(), error::Error> {
    let mut ret = true;
    let mut numbCount = 0;
    for num in addr.split(":") {
      numbCount += 1;
      let parseResult = u32::from_str_radix(num, 16);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => { return Err(error::Error::Num(err)); }
      };
      // MAC in [0,255] by index
      ret = ret && val<=255;
    }

    // MAC is six ints
    if ret && numbCount==6 {
      return Ok(());
    } else {
      return Err(error::Error::InvalidFormat);
    }
  }

  fn isIpv4Address(&self, addr: &String) -> Result<(), error::Error> {
    let mut ret = true;
    let mut numbCount = 0;
    for num in addr.split(".") {
      numbCount += 1;
      let parseResult = u32::from_str_radix(num, 10);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => { return Err(error::Error::Num(err)); }
      };
      // IPV4 in [0,255] by index
      ret = ret && val<=255;
    }

    // IPV4 is four ints
    if ret && numbCount==4 {
      return Ok(());
    } else {
      return Err(error::Error::InvalidFormat);
    }
  }

  fn isIpv6Address(&self, addr: &String) -> Result<(), error::Error> {
    let mut ret = true;
    let mut numbCount = 0;
    for num in addr.split(":") {
      numbCount += 1;
      let parseResult = u32::from_str_radix(num, 16);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => { return Err(error::Error::Num(err)); }
      };
      // IPV6 in [0,65535] by index
      ret = ret && val<=0xffff;
    }

    // IPV6 is eight ints
    if ret && numbCount==8 {
      return Ok(());
    } else {
      return Err(error::Error::InvalidFormat);
    }
  }

  fn parseFile(&self, fname: &str) -> Result<(), error::Error> {
    // Read JSON
    let jsonResult = fs::read_to_string(fname);
    let json = match jsonResult {
      Ok(val) => val,
      Err(err) => { return Err(error::Error::Io(err)); }
    };

    // Parse JSON
    let mut parser = JsonParser::new(json.chars());
    let jsonObject = match parser.parse() {
      Ok(val) => val,
      Err(err) => { return jsonObject; }
    };

    // Make sure it's an object
    if !jsonObject.is_object() {
      return Err(error::Error::JSONSchema);
    }

    // Parse + verify
    match self.verify(&jsonObject) {
      Ok(()) => {},
      Err(err) => { return Err(err); }
    };

    return Ok(());
  }
}
