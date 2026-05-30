use crate::cfg::limit;
use crate::err::error;
use crate::cfg::common;
use crate::cfg::interface::Verify;
use tinyjson::{JsonValue};
use std::collections::HashMap;

#[allow(non_snake_case)]
struct NIC {
  pub macAddress: String,
  pub srptName: String,
  pub ipv4Address: String,
  pub ipv6Address: String,
  pub pciAddress: String,
  pub mtuSizeBytes: u32,
  pub linkSpeedGbit: u32,
  pub maxTransports: u32,
  pub numaNode: u32,
}

impl NIC {
  pub fn new() -> Self {
    Self {
      macAddress: String::new(),
      srptName: String::new(),
      ipv4Address: String::new(),
      ipv6Address: String::new(),
      pciAddress: String::new(),
      mtuSizeBytes: 0,
      linkSpeedGbit: 0,
      maxTransports: 0,
      numaNode: 0,
    }
  }

  pub fn verify(&self, nic: &TestNIC) -> Result<(), error::Error> {
    let mut ret = true;
    let mut found = false;

    ret = ret && self.macAddress.len()>0;
    ret = ret && self.srptName.len()>0;
    ret = ret && (self.ipv4Address.len()>0 || self.ipv6Address.len()>0);
    ret = ret && self.pciAddress.len()>0;
    ret = ret && self.mtuSizeBytes>=limit::Constant::MTUSizeBytesMin;
    ret = ret && self.mtuSizeBytes<=limit::Constant::MTUSizeBytesMax;
    ret = ret && self.maxTransports>=limit::Constant::TransportsMin;
    ret = ret && self.maxTransports<=limit::Constant::TransportsMax;
    ret = ret && self.numaNode>=limit::Constant::NUMANodeMin;
    ret = ret && self.numaNode<=limit::Constant::NUMANodeMax;

    for val in limit::LinkBandWidthGbitPerSec {
      if val==self.linkSpeedGbit {
        found = true;
        break;
      }
    }
    ret = ret && found;

    let mut parseResult = nic.isMacAddress(&self.macAddress);
    match parseResult {
      Ok(_) => true,
      Err(_) => { return parseResult; }
    };

    parseResult = nic.isPciAddress(&self.pciAddress);
    match parseResult {
      Ok(_) => true,
      Err(_) => { return parseResult; }
    };

    if self.ipv4Address.len()>0 {
      parseResult = nic.isIpv4Address(&self.ipv4Address);
      match parseResult {
        Ok(_) => true,
        Err(_) => { return parseResult; }
      };
    }

    if self.ipv6Address.len()>0 {
      parseResult = nic.isIpv6Address(&self.ipv6Address);
      match parseResult {
        Ok(_) => true,
        Err(_) => { return parseResult; }
      };
    }

    if ret {
      return Ok(());
    } else {
      return Err(error::Error::OutOfRange);
    }
  }

  pub fn crossVerify(&self, nic: &TestNIC) -> Result<(), error::Error> {
    return Ok(());
  }
}

#[allow(non_snake_case)]
struct NICQueue {
  pub ringSize: u32,
  pub allocatorName: String,
}

impl NICQueue {
  pub fn new() -> Self {
    Self {
      ringSize: 0,
      allocatorName: String::new(),
    }
  }

  pub fn verify(&self, nic: &TestNIC) -> Result<(), error::Error> {
    let mut ret = true;

    ret = ret && self.ringSize>=limit::Constant::RingCountMin;
    ret = ret && self.ringSize<=limit::Constant::RingCountMax;
    ret = ret && self.allocatorName.len()>0;

    if ret {
      return Ok(());
    } else {
      return Err(error::Error::OutOfRange);
    }
  }

  pub fn crossVerify(&self, nic: &TestNIC) -> Result<(), error::Error> {
    return Ok(());
  }
}

#[allow(non_snake_case)]
struct NICQueuePair {
  pub rxq: String,
  pub txq: String,
}

impl NICQueuePair {
  pub fn new() -> Self {
    Self {
      rxq: String::new(),
      txq: String::new(),
    }
  }

  pub fn verify(&self, nic: &TestNIC) -> Result<(), error::Error> {
    let ret = self.rxq.len()>0 && self.txq.len()>0;

    if ret {
      return Ok(());
    } else {
      return Err(error::Error::OutOfRange);
    }
  }
}

#[allow(non_snake_case)]
struct Transport {
  pub nicName: String,
  pub queuePair: Vec<NICQueuePair>,
  pub ipv4Suffix: common::VLANPort,
  pub ipv6Suffix: common::VLANPort,
  pub ipv4ErrorSuffix: common::VLANPort,
  pub ipv6ErrorSuffix: common::VLANPort,
  pub callbackCapacity: u32,
  pub readyCapacity: u32,
  pub reserveCapacity: u32,
  pub allocatorName: String,
  pub cpuHwCore: u32,
}

impl Transport {
  pub fn new() -> Self {
    return Self {
      nicName: String::new(),
      queuePair: Vec::new(),
      ipv4Suffix: common::VLANPort::new(),
      ipv6Suffix: common::VLANPort::new(),
      ipv4ErrorSuffix: common::VLANPort::new(),
      ipv6ErrorSuffix: common::VLANPort::new(),
      callbackCapacity: 0,
      readyCapacity: 0,
      reserveCapacity: 0,
      allocatorName: String::new(),
      cpuHwCore: 0,
    }
  }

  pub fn verify(&self, nic: &TestNIC) -> Result<(), error::Error> {
    let mut ret = true;

    ret = ret && self.nicName.len()>0;
    ret = ret && self.queuePair.len()>0;
    ret = ret && self.callbackCapacity>=limit::Constant::RPCCallbackCapacityMin;
    ret = ret && self.callbackCapacity<=limit::Constant::RPCCallbackCapacityMax;
    ret = ret && self.readyCapacity>=limit::Constant::RPCReadyCapacityMin;
    ret = ret && self.readyCapacity<=limit::Constant::RPCReadyCapacityMax;
    ret = ret && self.reserveCapacity>=limit::Constant::RPCReserveCapacityMin;
    ret = ret && self.reserveCapacity<=limit::Constant::RPCReserveCapacityMax;
    ret = ret && self.allocatorName.len()>0;
    ret = ret && self.cpuHwCore>=limit::Constant::CPUCoreMin;
    ret = ret && self.cpuHwCore<=limit::Constant::CPUCoreMax;

    let mut ipv4 = true;
    match self.ipv4Suffix.verify() {
      Ok(_) => ipv4 = ipv4 && true,
      Err(err) => { return Err(err); }
    };
    match self.ipv4ErrorSuffix.verify() {
      Ok(_) => ipv4 = ipv4 && true,
      Err(err) => { return Err(err); }
    };

    let mut ipv6 = true;
    match self.ipv6Suffix.verify() {
      Ok(_) => ipv6 = ipv6 && true,
      Err(err) => { return Err(err); }
    };
    match self.ipv6ErrorSuffix.verify() {
      Ok(_) => ipv6 = ipv6 && true,
      Err(err) => { return Err(err); }
    };

    ret = ret && (ipv4 || ipv6);

    if ret {
      return Ok(());
    } else {
      return Err(error::Error::OutOfRange);
    }
  }

  pub fn crossVerify(&self, nic: &TestNIC) -> Result<(), error::Error> {
    return Ok(());
  }
}

#[allow(non_snake_case)]
pub struct TestNIC {
  nicMap: HashMap<String, NIC>,
  hugePageMap: HashMap<String, common::HugePage>,
  heapAllocMap: HashMap<String, common::HeapAllocator>,
  childAllocMap: HashMap<String, common::ChildAllocator>,
  transportMap: HashMap<String, Transport>,
}

impl TestNIC {
  pub fn new() -> Self {
    Self {
      nicMap: HashMap::new(),
      hugePageMap: HashMap::new(),
      heapAllocMap: HashMap::new(),
      childAllocMap: HashMap::new(),
      transportMap: HashMap::new(),
    }
  }
}

impl Verify for TestNIC {
  fn verify(&self, obj: &JsonValue) -> Result<(), error::Error> {
    return Ok(());
  }
}
