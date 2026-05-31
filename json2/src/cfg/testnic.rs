use crate::cfg::limit;
use crate::err::error;
use crate::cfg::common;
use crate::cfg::interface::Verify;
use tinyjson::{JsonValue};
use std::collections::HashMap;

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

struct Tag;

impl Tag {
  pub const Name: &str = "Name";
  pub const ParentName: &str = "ParentName";
  pub const AllocatorName: &str = "AllocatorName";

  pub const SizeKB : &str = "SizeKB";
  pub const ByteAlignment: &str = "ByteAlignment";
  pub const HugePageCount: &str = "HugePageCount";

  pub const NIC: &str = "NIC";
  pub const RXQ: &str = "RXQ";
  pub const TXQ: &str = "TXQ";
  pub const SRPT: &str = "SRPT";
  pub const HugePage: &str = "HugePage";
  pub const Transport: &str = "Transport";
  pub const TransportSet: &str = "TransportSet";
  pub const HeapAllocator: &str = "HeapAllocator";
  pub const ChildAllocator: &str = "ChildAllocator";

  pub const Capacity: &str = "Capacity";
  pub const CpuHwCore: &str = "CpuHwCore";
  pub const RequestRingCount: &str = "RequestRingCount";
  pub const ResponseRingCount: &str = "ResponseRingCount";
  pub const ScheduledPriority: &str = "ScheduledPriority";
  pub const UnscheduledPriority: &str = "UnscheduledPriority";
  pub const OverCommitmentCount: &str = "OverCommitmentCount";
}

pub struct TestNIC {
  nicMap: HashMap<String, NIC>,
  nameMap: HashMap<String, bool>,
  srptMap: HashMap<String, common::SRPT>,
  hugePageMap: HashMap<String, common::HugePage>,
  heapAllocMap: HashMap<String, common::HeapAllocator>,
  childAllocMap: HashMap<String, common::ChildAllocator>,
  transportMap: HashMap<String, Transport>,
}

impl TestNIC {
  pub fn new() -> Self {
    Self {
      nicMap: HashMap::new(),
      nameMap: HashMap::new(),
      srptMap: HashMap::new(),
      hugePageMap: HashMap::new(),
      heapAllocMap: HashMap::new(),
      childAllocMap: HashMap::new(),
      transportMap: HashMap::new(),
    }
  }

  fn createSRPT(&mut self, key: &str) -> Option<&mut common::SRPT> {
    if !self.nameMap.contains_key(key) && !self.srptMap.contains_key(key) {
      self.nameMap.insert(key.to_string(), true);
      return Some(self.srptMap.entry(key.to_string()).or_insert(common::SRPT::new()));
    }
    return None;
  }

  fn createHugePage(&mut self, key: &str) -> Option<&mut common::HugePage> {
    if !self.nameMap.contains_key(key) && !self.hugePageMap.contains_key(key) {
      self.nameMap.insert(key.to_string(), true);
      return Some(self.hugePageMap.entry(key.to_string()).or_insert(common::HugePage::new()));
    }
    return None;
  }

  fn createHeapAllocator(&mut self, key: &str) -> Option<&mut common::HeapAllocator> {
    if !self.nameMap.contains_key(key) && !self.heapAllocMap.contains_key(key) {
      self.nameMap.insert(key.to_string(), true);
      return Some(self.heapAllocMap.entry(key.to_string()).or_insert(common::HeapAllocator::new()));
    }
    return None;
  }

  fn createChildAllocator(&mut self, key: &str) -> Option<&mut common::ChildAllocator> {
    if !self.nameMap.contains_key(key) && !self.childAllocMap.contains_key(key) {
      self.nameMap.insert(key.to_string(), true);
      return Some(self.childAllocMap.entry(key.to_string()).or_insert(common::ChildAllocator::new()));
    }
    return None;
  }

  fn jsonInteger(obj: &JsonValue) -> Result<u64, error::Error> {
    if obj.is_number() {
      // API forces f64, so make sure it's an integer
      let result: Option<&f64> = obj.get();
      match result {
        Some(val) => { if *val>=0.0 && val.fract()==0.0 { return Ok(*val as u64); } }
        None => {}
      };
    }
    return Err(error::Error::JSONSchema);
  }

  fn jsonBool(obj: &JsonValue) -> Result<bool, error::Error> {
    if obj.is_bool() {
      let result: Option<&bool> = obj.get();
      match result {
        Some(val) => { return Ok(*val); }
        None => {}
      };
    }
    return Err(error::Error::JSONSchema);
  }

  fn jsonString(obj: &JsonValue) -> Result<String, error::Error> {
    if obj.is_string() {
      let result: Option<&String> = obj.get();
      match result {
        // Clone string so when JSON dropped, copy exists
        Some(val) => { if val.len()>0 { return Ok(val.clone()); } }
        None => {}
      };
    }
    return Err(error::Error::JSONSchema);
  }

  fn findAndAddName(parentObj: &JsonValue, nameMap: &mut HashMap<String, bool>, kind: &str, parentName: &str)
    -> Result<String, error::Error> {
    debug_assert!(parentObj.is_object());

    // Get map from parentObj, and find Name. The direct route
    // parentObj[Tag::Name] risks panic if does not exist
    let map: &HashMap<_, _> = parentObj.get().unwrap();
    if !map.contains_key(Tag::Name) {
      log::error!(target: "json", "'{}' object '{}' missing name", kind, parentName);
      return Err(error::Error::JSONSchema);
    }

    // Make sure name is a non-empty, unique string
    let jsonObj = &parentObj[Tag::Name];
    if jsonObj.is_string() {
      let strRef: Option<&String> = jsonObj.get();
      match strRef {
        Some(val) => {
          if val.len()>0 {
            let fqn = format!("{}.{}", parentName, val);
            if nameMap.contains_key(&fqn) {
              log::error!(target: "json", "'{}' object '{}' duplicate name", kind, fqn);
              return Err(error::Error::JSONSchema);
            }
            nameMap.insert(fqn.clone(), true);
            return Ok(fqn.clone());
          }
        }
        None => {}
      };
    }

    log::error!(target: "json", "'{}' object in '{}' field '{}' malformed string", kind, parentName, Tag::Name);
    return Err(error::Error::JSONSchema);
  }

  fn jsonArray(ary: &Vec<JsonValue>, data: &mut [u32], fqn: &String, objKind: &str, key: &str) -> Result<(), error::Error> {
    let mut idx: usize = 0;
    let mut ret = ary.len()==data.len();
    if ret {
      for obj in ary {
        if !obj.is_number() {
          ret = false;
          break;
        }
        // API forces f64, so cast
        let result: Option<&f64> = obj.get();
        match result {
          Some(val) => { if *val>=0.0 && val.fract()==0.0 { data[idx] = *val as u32; idx += 1; } }
          None => { ret = false; }
        };
      }
    }
    if !ret {
      log::error!(target: "json", "'{}' object '{}.{}' is not an valid array of u32 length {}",
          objKind, fqn, key, data.len());
      return Err(error::Error::JSONSchema);
    } else {
      return Ok(());
    }
  }

  fn parseHugePage(&mut self, parentName: &str, array: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an array
    if !array.is_array() {
      return Err(error::Error::JSONSchema);
    }

    // Visit each item in ary
    let ary: &Vec<_> = array.get().unwrap();
    for item in ary {
      if !item.is_object() {
        return Err(error::Error::JSONSchema);
      }

      // Access inner hashmap
      let map: &HashMap<_, _> = item.get().unwrap();

      // Find the huge page name
      if !map.contains_key(Tag::Name) {
        return Err(error::Error::JSONSchema);
      }
      let name = match TestNIC::jsonString(map.get(Tag::Name).unwrap()) {
        Ok(val) => val,
          Err(err) => { return Err(err); }
      };

      // Create hugePage
      let fqn = format!("{}.{}", parentName, name);
      log::debug!(target: "json", "verifying '{}'", fqn);
      let hp = match self.createHugePage(fqn.as_str()) {
        Some(val) => val,
          None => {
            log::error!(target: "json", "hugePage '{}' duplicated", fqn);
            return Err(error::Error::DupReference);
          }
      };

      // Find all other key-value pairs
      for (key, value) in map {
        match key.as_str() {
          Tag::Name => {}
          Tag::HugePageCount => {
            match TestNIC::jsonInteger(value) {
              Ok(val) => { hp.pageCount = val as u32; }
              Err(err) => {
                log::error!(target: "json", "hugePage '{}.{}' invalid: {:?}", fqn, key, err);
                return Err(err);
              }
            }
          }
          Tag::SizeKB => {
            match TestNIC::jsonInteger(value) {
              Ok(val) => { hp.pageSizeKB = val as u32; }
              Err(err) => {
                log::error!(target: "json", "hugePage '{}.{}' invalid: {:?}", fqn, key, err);
                return Err(err);
              }
            }
          }
          Tag::ByteAlignment => {
            match TestNIC::jsonInteger(value) {
              Ok(val) => { hp.byteAlignment = val as u32; }
              Err(err) => {
                log::error!(target: "json", "hugePage '{}.{}' invalid: {:?}", fqn, key, err);
                return Err(err);
              }
            }
          }
          other => {
            log::error!(target: "json", "hugePage '{}.{}' unknown", fqn, key);
            return Err(error::Error::JSONSchema);
          }
        }
      }

      // Verify contents
      match hp.verify() {
        Ok(()) => {}
        Err(err) => {
          log::error!(target: "json", "hugePage '{}' invalid contents: {:?}", fqn, err);
          return Err(err);
        }
      };
    }

    return Ok(());
  }

  fn parseHeapAllocator(&mut self, parentName: &str, array: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an array
    if !array.is_array() {
      return Err(error::Error::JSONSchema);
    }

    // Visit each item in ary
    let ary: &Vec<_> = array.get().unwrap();
    for item in ary {
      if !item.is_object() {
        return Err(error::Error::JSONSchema);
      }

      // Access inner hashmap
      let map: &HashMap<_, _> = item.get().unwrap();

      // Find the huge page name
      if !map.contains_key(Tag::Name) {
        return Err(error::Error::JSONSchema);
      }
      let name = match TestNIC::jsonString(map.get(Tag::Name).unwrap()) {
        Ok(val) => val,
          Err(err) => { return Err(err); }
      };

      // Create heapAllocator
      let fqn = format!("{}.{}", parentName, name);
      log::debug!(target: "json", "verifying '{}'", fqn);
      let hp = match self.createHeapAllocator(fqn.as_str()) {
        Some(val) => val,
          None => {
            log::error!(target: "json", "heapAllocator '{}' duplicated", fqn);
            return Err(error::Error::DupReference);
          }
      };

      // Find all other key-value pairs
      for (key, value) in map {
        match key.as_str() {
          Tag::Name => {}
          Tag::SizeKB => {
            match TestNIC::jsonInteger(value) {
              Ok(val) => { hp.sizeKB = val as u32; }
              Err(err) => {
                log::error!(target: "json", "heapAllocator '{}.{}' invalid: {:?}", fqn, key, err);
                return Err(err);
              }
            }
          }
          Tag::ByteAlignment => {
            match TestNIC::jsonInteger(value) {
              Ok(val) => { hp.byteAlignment = val as u32; }
              Err(err) => {
                log::error!(target: "json", "heapAllocator '{}.{}' invalid: {:?}", fqn, key, err);
                return Err(err);
              }
            }
          }
          other => {
            log::error!(target: "json", "heapAllocator '{}.{}' unknown", fqn, key);
            return Err(error::Error::JSONSchema);
          }
        }
      }

      // Verify contents
      match hp.verify() {
        Ok(()) => {}
        Err(err) => {
          log::error!(target: "json", "heapAllocator '{}' invalid contents: {:?}", fqn, err);
          return Err(err);
        }
      };
    }

    return Ok(());
  }

  fn parseChildAllocator(&mut self, parentName: &str, array: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an array
    if !array.is_array() {
      return Err(error::Error::JSONSchema);
    }

    // Visit each item in ary
    let ary: &Vec<_> = array.get().unwrap();
    for item in ary {
      if !item.is_object() {
        return Err(error::Error::JSONSchema);
      }

      // Access inner hashmap
      let map: &HashMap<_, _> = item.get().unwrap();

      // Find the huge page name
      if !map.contains_key(Tag::Name) {
        return Err(error::Error::JSONSchema);
      }
      let name = match TestNIC::jsonString(map.get(Tag::Name).unwrap()) {
        Ok(val) => val,
          Err(err) => { return Err(err); }
      };

      // Create childAllocator
      let fqn = format!("{}.{}", parentName, name);
      log::debug!(target: "json", "verifying '{}'", fqn);
      let hp = match self.createChildAllocator(fqn.as_str()) {
        Some(val) => val,
          None => {
            log::error!(target: "json", "childAllocator '{}' duplicated", fqn);
            return Err(error::Error::DupReference);
          }
      };

      // Find all other key-value pairs
      for (key, value) in map {
        match key.as_str() {
          Tag::Name => {}
          Tag::ParentName => {
            match TestNIC::jsonString(value) {
              Ok(val) => { hp.parentName = val; }
              Err(err) => {
                log::error!(target: "json", "childAllocator '{}.{}' invalid: {:?}", fqn, key, err);
                return Err(err);
              }
            }
          }
          Tag::SizeKB => {
            match TestNIC::jsonInteger(value) {
              Ok(val) => { hp.sizeKB = val as u32; }
              Err(err) => {
                log::error!(target: "json", "childAllocator '{}.{}' invalid: {:?}", fqn, key, err);
                return Err(err);
              }
            }
          }
          Tag::ByteAlignment => {
            match TestNIC::jsonInteger(value) {
              Ok(val) => { hp.byteAlignment = val as u32; }
              Err(err) => {
                log::error!(target: "json", "childAllocator '{}.{}' invalid: {:?}", fqn, key, err);
                return Err(err);
              }
            }
          }
          other => {
            log::error!(target: "json", "childAllocator '{}.{}' unknown", fqn, key);
            return Err(error::Error::JSONSchema);
          }
        }
      }

      // Verify contents
      match hp.verify() {
        Ok(()) => {}
        Err(err) => {
          log::error!(target: "json", "childAllocator '{}' invalid contents: {:?}", fqn, err);
          return Err(err);
        }
      };
    }

    return Ok(());
  }

  fn parseSRPT(&mut self, parentName: &str, obj: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an object
    if !obj.is_object() {
      return Err(error::Error::JSONSchema);
    }

    // Access inner hashmap
    let map: &HashMap<_, _> = obj.get().unwrap();

    // Find the SRPT name
    if !map.contains_key(Tag::Name) {
      return Err(error::Error::JSONSchema);
    }
    let name = match TestNIC::jsonString(map.get(Tag::Name).unwrap()) {
      Ok(val) => val,
        Err(err) => { return Err(err); }
    };

    // Create SRPT
    let fqn = format!("{}.{}", parentName, name);
    log::debug!(target: "json", "verifying '{}'", fqn);
    let hp = match self.createSRPT(fqn.as_str()) {
      Some(val) => val,
        None => {
          log::error!(target: "json", "SRPT '{}' duplicated", fqn);
          return Err(error::Error::DupReference);
        }
    };

    // Find all other key-value pairs
    for (key, value) in map {
      match key.as_str() {
        Tag::Name => {}
        Tag::Capacity => {
          match TestNIC::jsonInteger(value) {
            Ok(val) => { hp.capacity = val as u32; }
            Err(err) => {
              log::error!(target: "json", "SRPT '{}.{}' invalid: {:?}", fqn, key, err);
              return Err(err);
            }
          }
        }
        Tag::OverCommitmentCount => {
          match TestNIC::jsonInteger(value) {
            Ok(val) => { hp.overCommitmentCount = val as u32; }
            Err(err) => {
              log::error!(target: "json", "SRPT '{}.{}' invalid: {:?}", fqn, key, err);
              return Err(err);
            }
          }
        }
        Tag::ResponseRingCount => {
          match TestNIC::jsonInteger(value) {
            Ok(val) => { hp.responseRingCount = val as u32; }
            Err(err) => {
              log::error!(target: "json", "SRPT '{}.{}' invalid: {:?}", fqn, key, err);
              return Err(err);
            }
          }
        }
        Tag::RequestRingCount => {
          match TestNIC::jsonInteger(value) {
            Ok(val) => { hp.requestRingCount = val as u32; }
            Err(err) => {
              log::error!(target: "json", "SRPT '{}.{}' invalid: {:?}", fqn, key, err);
              return Err(err);
            }
          }
        }
        Tag::CpuHwCore => {
          match TestNIC::jsonInteger(value) {
            Ok(val) => { hp.cpuHwCore = val as u32; }
            Err(err) => {
              log::error!(target: "json", "SRPT '{}.{}' invalid: {:?}", fqn, key, err);
              return Err(err);
            }
          }
        }
        Tag::AllocatorName => {
          match TestNIC::jsonString(value) {
            Ok(val) => { hp.allocatorName = val; }
            Err(err) => {
              log::error!(target: "json", "SRPT '{}.{}' invalid: {:?}", fqn, key, err);
              return Err(err);
            }
          }
        }
        Tag::UnscheduledPriority => {
          if value.is_array() {
            let ary: &Vec<_> = value.get().unwrap();
            match TestNIC::jsonArray(ary, &mut hp.unscheduledPriority, &fqn, Tag::SRPT, key) {
              Ok(val) => {}
              Err(err) => {
                log::error!(target: "json", "'{}.{}.{}' invalid: {:?}", fqn, Tag::SRPT, key, err);
                return Err(err);
              }
            };
          } else {
            log::error!(target: "json", "'{}.{}.{}' not an array", fqn, Tag::SRPT, key);
            return Err(error::Error::JSONSchema);
          }
        }
        Tag::ScheduledPriority => {
          if value.is_array() {
            let ary: &Vec<_> = value.get().unwrap();
            match TestNIC::jsonArray(ary, &mut hp.scheduledPriority, &fqn, Tag::SRPT, key) {
              Ok(val) => {}
              Err(err) => {
                log::error!(target: "json", "'{}.{}.{}' invalid: {:?}", fqn, Tag::SRPT, key, err);
                return Err(err);
              }
            };
          } else {
            log::error!(target: "json", "'{}.{}.{}' not an array", fqn, Tag::SRPT, key);
            return Err(error::Error::JSONSchema);
          }
        }
        other => {
          log::error!(target: "json", "'{}.{}.{}' unknown field", fqn, Tag::SRPT, key);
          return Err(error::Error::JSONSchema);
        }
      }

      // Verify contents
      match hp.verify() {
        Ok(()) => {}
        Err(err) => {
          log::error!(target: "json", "SRPT '{}' invalid contents: {:?}", fqn, err);
          return Err(err);
        }
      };
    }

    return Ok(());
  }

  fn parseTransportSet(&mut self, item: &JsonValue) -> Result<(), error::Error> {
    debug_assert!(item.is_object());

    let parentName = "root";
    let fqn = match TestNIC::findAndAddName(item, &mut self.nameMap, Tag::TransportSet, &parentName) {
      Ok(val) => val,
      Err(err) => { return Err(err); }
    };

    log::debug!(target: "json", "verifying '{}'", fqn);

    // Get inner object then find, parse sub-objects 
    let map: &HashMap<_, _> = item.get().unwrap();

    // Parse HugePageAllocator(s)
    if map.contains_key(Tag::HugePage) {
      let result = match self.parseHugePage(fqn.as_str(), &item[Tag::HugePage]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    // Parse HeapAllocators
    if map.contains_key(Tag::HeapAllocator) {
      let result = match self.parseHeapAllocator(fqn.as_str(), &item[Tag::HeapAllocator]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    // Parse ChildAllocators
    if map.contains_key(Tag::ChildAllocator) {
      let result = match self.parseChildAllocator(fqn.as_str(), &item[Tag::ChildAllocator]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    // Parse SRPT
    if map.contains_key(Tag::SRPT) {
      let result = match self.parseSRPT(fqn.as_str(), &item[Tag::SRPT]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    return Ok(());
  }
}

impl Verify for TestNIC {
  fn verify(&mut self, obj: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an object
    if !obj.is_object() {
      return Err(error::Error::JSONSchema);
    }

    // Make sure 'TransportSet' exists
    let map: &HashMap<_, _> = obj.get().unwrap();
    if !map.contains_key(Tag::TransportSet) {
      return Err(error::Error::JSONSchema);
    }

    // Make sure TransportSet is an array
    let transportSet = &obj[Tag::TransportSet];
    if !transportSet.is_array() {
      return Err(error::Error::JSONSchema);
    }

    // Make sure array has 1+ elements
    let ary: &Vec<_> = transportSet.get().unwrap();
    if ary.len()==0 {
      return Err(error::Error::JSONSchema);
    }

    // Iterate over each entry in transportSet
    for item in ary {
      // Make sure it's an object
      if !item.is_object() {
        return Err(error::Error::JSONSchema);
      }
      // Parse one transportSet item
      match self.parseTransportSet(item) {
        Ok(_) => {},
          Err(err) => { return Err(err); }
      }
    };

    return Ok(());
  }
}
