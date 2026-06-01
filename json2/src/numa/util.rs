use log;
use std::fs;
use crate::err::error;

fn ubuntuPciNumaNode(addr: &String) -> Result<u32, error::Error> {
  debug_assert!(addr.len()==12);
  let prefix = &addr[0..7];
  let fname = std::format!("/sys/class/pci_bus/{}/device/{}/numa_node", prefix, addr);

  // Read data
  let dataResult = fs::read_to_string(&fname);
  let data = match dataResult {
    Ok(val) => val,
    Err(err) => {
      log::error!("open '{}': {:?}", fname, err);
      return Err(error::Error::Io(err));
    }
  };

  // Parse data base-10
  let parseResult = u32::from_str_radix(data.trim_end(), 10);
  let numaNode = match parseResult {
    Ok(val) => {
      log::trace!("read '{}': '{}' asInt {}", fname, data.trim_end(), val);
      val
    }
    Err(err) => {
      log::error!("read '{}': '{}': {:?}", fname, data.trim_end(), err);
      return Err(error::Error::Num(err));
    }
  };

  return Ok(numaNode);
}

fn ubuntuIsCpuOnNumaNode(cpuHwCore: u32, numaNode: u32) -> Result<bool, error::Error> {
  let fname = std::format!("/sys/devices/system/node/node{}/cpumap", numaNode);

  // Read data
  let dataResult = fs::read_to_string(&fname);
  let data = match dataResult {
    Ok(val) => val,
    Err(err) => {
      log::error!("open '{}': {:?}", fname, err);
      return Err(error::Error::Io(err));
    }
  };

  // Parse data base-16
  let parseResult = u64::from_str_radix(data.trim_end(), 16);
  let cpuMask = match parseResult {
    Ok(val) => {
      log::trace!("read '{}': '{}' asInt {}", fname, data.trim_end(), val);
      val
    }
    Err(err) => {
      log::error!("read '{}': '{}': {:?}", fname, data.trim_end(), err);
      return Err(error::Error::Num(err));
    }
  };

  // Return true if cpu bit is on in mask
  let ok = (1<<(cpuHwCore as u64)) & cpuMask;
  return Ok(ok!=0);
}

pub fn pciNumaNode(addr: &String) -> Result<u32, error::Error> {
  return ubuntuPciNumaNode(addr);
}

pub fn isCpuOnNumaNode(cpuHwCore: u32, numaNode: u32) -> Result<bool, error::Error> {
  return ubuntuIsCpuOnNumaNode(cpuHwCore, numaNode);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn testPciNumaNode() {
    let addr = "0000:05:00.0".to_string();
    let result = pciNumaNode(&addr);
    match result {
      Ok(val) => { assert_eq!(val,0); }
      Err(err) => { println!("failed error: {:?}", err); assert!(false); }
    }
  }

  #[test]
  fn testIsCpuOnNumaNode() {
    let result = isCpuOnNumaNode(0,0);
    match result {
      Ok(val) => { assert_eq!(val, true); }
      Err(err) => { println!("failed error: {:?}", err); assert!(false); }
    }
  }
}
