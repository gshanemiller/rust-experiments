use log;
use std::fs;
use crate::err::error;
use tinyjson::{JsonParser, JsonValue};

pub trait Verify {
  fn verify(&mut self) -> Result<(), error::Error>;
  fn parse(&mut self, obj: &JsonValue) -> Result<(), error::Error>;

  fn isPciAddress(&self, addr: &String) -> Result<(), error::Error> {
    let mut ret = true;
    let mut count = 0;
    for num in addr.split(&[':','.']) {
      count += 1;
      let parseResult = u32::from_str_radix(num, 16);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => {
          log::error!("'{}' is not a valid full PCI address", addr);
          return Err(error::Error::Num(err));
        }
      };
      if count==1 {
        ret = ret && val<=65535;
      } else if count>=2 && count<=3 {
        ret = ret && val<=255;
      } else {
        ret = ret && val<=15;
      }
    }

    // PCI is four ints
    if ret && count==4 {
      return Ok(());
    } else {
      return Err(error::Error::InvalidFormat);
    }
  }

  fn isMacAddress(&self, addr: &String) -> Result<(), error::Error> {
    let mut ret = true;
    let mut count = 0;
    for num in addr.split(":") {
      count += 1;
      let parseResult = u32::from_str_radix(num, 16);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => {
          log::error!("'{}' is not a valid MAC address", addr);
          return Err(error::Error::Num(err));
        }
      };
      // MAC in [0,255] by index
      ret = ret && val<=255;
    }

    // MAC is six ints
    if ret && count==6 {
      return Ok(());
    } else {
      return Err(error::Error::InvalidFormat);
    }
  }

  fn isIpv4Address(&self, addr: &String) -> Result<(), error::Error> {
    let mut ret = true;
    let mut count = 0;
    for num in addr.split(".") {
      count += 1;
      let parseResult = u32::from_str_radix(num, 10);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => {
          log::error!("'{}' is not a valid MAC address", addr);
          return Err(error::Error::Num(err));
        }
      };
      // IPV4 in [0,255] by index
      ret = ret && val<=255;
    }

    // IPV4 is four ints
    if ret && count==4 {
      return Ok(());
    } else {
      return Err(error::Error::InvalidFormat);
    }
  }

  fn isIpv6Address(&self, addr: &String) -> Result<(), error::Error> {
    let mut ret = true;
    let mut count = 0;
    for num in addr.split(":") {
      count += 1;
      let parseResult = u32::from_str_radix(num, 16);
      let val = match parseResult {
        Ok(val) => val,
        Err(err) => {
          log::error!("'{}' is not a valid MAC address", addr);
          return Err(error::Error::Num(err));
        }
      };
      // IPV6 in [0,65535] by index
      ret = ret && val<=0xffff;
    }

    // IPV6 is eight ints
    if ret && count==8 {
      return Ok(());
    } else {
      return Err(error::Error::InvalidFormat);
    }
  }

  fn parseFile(&mut self, fname: &str) -> Result<(), error::Error> {
    // Read JSON
    log::info!("reading '{}'", fname);
    let jsonResult = fs::read_to_string(fname);
    let json = match jsonResult {
      Ok(val) => val,
      Err(err) => { return Err(error::Error::Io(err)); }
    };

    // Parse JSON
    log::info!("parsing json '{}'", fname);
    let mut parser = JsonParser::new(json.chars());
    let jsonObject = match parser.parse() {
      Ok(val) => val,
      Err(_) => { return Err(error::Error::JSONError); }
    };

    // Parse
    log::info!("parsing json contents '{}'", fname);
    match self.parse(&jsonObject) {
      Ok(()) => {},
      Err(err) => { return Err(err); }
    };

    println!("drop json");
    drop(jsonObject);
    println!("drop done");

    // Verify
    log::info!("verifying json contents '{}'", fname);
    match self.verify() {
      Ok(()) => {},
      Err(err) => { return Err(err); }
    };

    return Ok(());
  }
}
