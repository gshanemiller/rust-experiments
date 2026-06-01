use crate::cfg::limit;
use crate::err::error;
use crate::cfg::common;
use crate::numa::util;
use crate::cfg::interface::Verify;
use tinyjson::{JsonValue};
use std::collections::HashMap;

struct NIC {
  pub name: String,
  pub macAddress: String,
  pub ipv4Address: String,
  pub ipv6Address: String,
  pub pciAddress: String,
  pub mtuSizeBytes: u32,
  pub linkSpeedGbit: u32,
  pub maxTransports: u32,
  //
  pub numaNode: u32,
}

impl NIC {
  pub fn new() -> Self {
    Self {
      name: String::new(),
      macAddress: String::new(),
      ipv4Address: String::new(),
      ipv6Address: String::new(),
      pciAddress: String::new(),
      mtuSizeBytes: 0,
      linkSpeedGbit: 0,
      maxTransports: 0,
      numaNode: 0,
    }
  }

  pub fn verify(&self, owner: &TestNIC) -> Result<(), error::Error> {
    let mut ret = true;
    let mut found = false;

    ret = ret && self.name.len()>0;
    ret = ret && self.macAddress.len()>0;
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

    let mut parseResult = owner.isMacAddress(&self.macAddress);
    match parseResult {
      Ok(_) => true,
      Err(_) => { return parseResult; }
    };

    parseResult = owner.isPciAddress(&self.pciAddress);
    match parseResult {
      Ok(_) => true,
      Err(_) => { return parseResult; }
    };

    if self.ipv4Address.len()>0 {
      parseResult = owner.isIpv4Address(&self.ipv4Address);
      match parseResult {
        Ok(_) => true,
        Err(_) => { return parseResult; }
      };
    }

    if self.ipv6Address.len()>0 {
      parseResult = owner.isIpv6Address(&self.ipv6Address);
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
  pub name: String,
  pub ringSize: u32,
  pub allocatorName: String,
}

impl NICQueue {
  pub fn new() -> Self {
    Self {
      name: String::new(),
      ringSize: 0,
      allocatorName: String::new(),
    }
  }

  pub fn verify(&self) -> Result<(), error::Error> {
    let mut ret = true;

    ret = ret && self.name.len()>0;
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
  pub name: String,
  pub queuePair: Vec<NICQueuePair>,
  pub ipv4Suffix: common::VLANPort,
  pub ipv6Suffix: common::VLANPort,
  pub ipv4ErrorSuffix: common::VLANPort,
  pub ipv6ErrorSuffix: common::VLANPort,
  pub callbackCapacity: u32,
  pub readyCapacity: u32,
  pub reserveCapacity: u32,
  pub allocatorName: String,
  pub srptScheduleName: String,
  pub cpu: u32,
}

impl Transport {
  pub fn new() -> Self {
    return Self {
      name: String::new(),
      queuePair: Vec::new(),
      ipv4Suffix: common::VLANPort::new(),
      ipv6Suffix: common::VLANPort::new(),
      ipv4ErrorSuffix: common::VLANPort::new(),
      ipv6ErrorSuffix: common::VLANPort::new(),
      callbackCapacity: 0,
      readyCapacity: 0,
      reserveCapacity: 0,
      allocatorName: String::new(),
      srptScheduleName: String::new(),
      cpu: 0,
    }
  }

  pub fn verify(&self) -> Result<(), error::Error> {
    let mut ret = true;

    ret = ret && self.name.len()>0;
    ret = ret && self.queuePair.len()>0;
    ret = ret && self.callbackCapacity>=limit::Constant::RPCCallbackCapacityMin;
    ret = ret && self.callbackCapacity<=limit::Constant::RPCCallbackCapacityMax;
    ret = ret && self.readyCapacity>=limit::Constant::RPCReadyCapacityMin;
    ret = ret && self.readyCapacity<=limit::Constant::RPCReadyCapacityMax;
    ret = ret && self.reserveCapacity>=limit::Constant::RPCReserveCapacityMin;
    ret = ret && self.reserveCapacity<=limit::Constant::RPCReserveCapacityMax;
    ret = ret && self.allocatorName.len()>0;
    ret = ret && self.srptScheduleName.len()>0;
    ret = ret && self.cpu>=limit::Constant::CPUCoreMin;
    ret = ret && self.cpu<=limit::Constant::CPUCoreMax;

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

struct TransportSet {
  pub name: String,
  pub nic: NIC,
  pub rxqVec: Vec<NICQueue>,
  pub txqVec: Vec<NICQueue>,
  pub srptVec: Vec<common::SRPT>,
  pub hugePageVec: Vec<common::HugePage>,
  pub heapAllocVec: Vec<common::HeapAllocator>,
  pub childAllocVec: Vec<common::ChildAllocator>,
  pub transportVec: Vec<Transport>,
}

impl TransportSet {
  pub fn new() -> Self {
    Self {
      name: String::new(),
      nic: NIC::new(),
      rxqVec: Vec::new(),
      txqVec: Vec::new(),
      srptVec: Vec::new(),
      hugePageVec: Vec::new(),
      heapAllocVec: Vec::new(),
      childAllocVec: Vec::new(),
      transportVec: Vec::new(),
    }
  }
}

struct Tag;

impl Tag {
  pub const Name: &str = "Name";
  pub const ParentName: &str = "ParentName";
  pub const AllocatorName: &str = "AllocatorName";
  pub const SRPTScheduleName: &str = "SRPTScheduleName";

  pub const SizeKB : &str = "SizeKB";
  pub const ByteAlignment: &str = "ByteAlignment";
  pub const PageCount: &str = "PageCount";
  pub const PageSizeKB: &str = "PageSizeKB";

  pub const NIC: &str = "NIC";
  pub const RXQ: &str = "RXQ";
  pub const TXQ: &str = "TXQ";
  pub const SRPT: &str = "SRPT";
  pub const HugePage: &str = "HugePage";
  pub const Transport: &str = "Transport";
  pub const TransportSet: &str = "TransportSet";
  pub const HeapAllocator: &str = "HeapAllocator";
  pub const ChildAllocator: &str = "ChildAllocator";

  pub const CPU: &str = "CPU";
  pub const Capacity: &str = "Capacity";
  pub const RingSize: &str = "RingSize";
  pub const RequestRingCount: &str = "RequestRingCount";
  pub const ResponseRingCount: &str = "ResponseRingCount";
  pub const ScheduledPriority: &str = "ScheduledPriority";
  pub const UnscheduledPriority: &str = "UnscheduledPriority";
  pub const OverCommitmentCount: &str = "OverCommitmentCount";
  
  pub const MACAddress: &str = "MACAddress";
  pub const IPV4Address: &str = "IPV4Address";
  pub const IPV6Address: &str = "IPV6Address";
  pub const PciDeviceId: &str = "PciDeviceId";
  pub const MTUSizeBytes: &str = "MTUSizeBytes";
  pub const LinkSpeedGbit: &str = "LinkSpeedGbit";
  pub const MaximumTransports: &str = "MaximumTransports";

  pub const RXQName: &str = "RXQName";
  pub const TXQName: &str = "TXQName";
  pub const QueuePair: &str = "QueuePair";
  pub const ReadyCapacity: &str = "ReadyCapacity";
  pub const ReserveCapacity: &str = "ReserveCapacity";
  pub const CallbackCapacity: &str = "CallbackCapacity";

  pub const Port: &str = "Port";
  pub const VLANId : &str = "VLANId";
  pub const IPV4Endpoint: &str = "IPV4Endpoint";
  pub const IPV6Endpoint: &str = "IPV6Endpoint";
  pub const ErrorIPV4Endpoint: &str = "ErrorIPV4Endpoint";
  pub const ErrorIPV6Endpoint: &str = "ErrorIPV6Endpoint";
}

pub struct TestNIC {
  nameMap: HashMap<String, bool>,
  transportSetVec: Vec<TransportSet>,
}

impl TestNIC {
  pub fn new() -> Self {
    Self {
      nameMap: HashMap::new(),
      transportSetVec: Vec::new(),
    }
  }

  fn jsonInteger(obj: &JsonValue, value: &mut u32, fqn: &String, objKind: &str, key: &str) -> Result<(), error::Error> {
    if obj.is_number() {
      // API forces f64, so make sure it's an integer
      let result: Option<&f64> = obj.get();
      match result {
        Some(val) => { if *val>=0.0 && val.fract()==0.0 { *value = *val as u32; return Ok(()); } }
        None => {}
      };
    }
    log::error!("'{}' object '{}.{}' malformed integer", objKind, fqn, key);
    return Err(error::Error::JSONSchema);
  }

  fn jsonBool(obj: &JsonValue, value: &mut bool, fqn: &String, objKind: &str, key: &str) -> Result<(), error::Error> {
    if obj.is_bool() {
      let result: Option<&bool> = obj.get();
      match result {
        Some(val) => { *value = *val; return Ok(()); }
        None => {}
      };
    }
    log::error!("'{}' object '{}.{}' malformed bool", objKind, fqn, key);
    return Err(error::Error::JSONSchema);
  }

  fn jsonString(obj: &JsonValue, value: &mut String, fqn: &String, objKind: &str, key: &str) -> Result<(), error::Error> {
    if obj.is_string() {
      let result: Option<&String> = obj.get();
      match result {
        // Clone string so when JSON dropped, copy exists
        Some(val) => { if val.len()>0 { *value = val.clone(); return Ok(()); } }
        None => {}
      };
    }
    log::error!("'{}' object '{}.{}' malformed string", objKind, fqn, key);
    return Err(error::Error::JSONSchema);
  }

  fn jsonArray(jsonObj: &JsonValue, data: &mut [u32], fqn: &String, objKind: &str, key: &str) -> Result<(), error::Error> {
    if !jsonObj.is_array() {
      log::error!("'{}' object '{}' is not an array", objKind, fqn);
      return Err(error::Error::JSONSchema);
    }

    // Visit each item in array
    let mut idx: usize = 0;
    let ary: &Vec<_> = jsonObj.get().unwrap();
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
      log::error!("'{}' object '{}.{}' is not an valid array of u32 length {}",
          objKind, fqn, key, data.len());
      return Err(error::Error::JSONSchema);
    } else {
      return Ok(());
    }
  }

  fn parseEndpoint(obj: &JsonValue, endPoint: &mut common::VLANPort, fqn: &String, kind: &str, parentKey: &str, _isIPV4: bool)
    -> Result<(), error::Error> {
    // Make sure it's an object
    if !obj.is_object() {
      log::error!("'{}' object '{}.{}' not an object", kind, fqn, parentKey);
      return Err(error::Error::JSONSchema);
    }

    // Process required endpoint fields
    let map: &HashMap<_, _> = obj.get().unwrap();
    for (key, value) in map {
      match key.as_str() {
        Tag::Port => {
          match TestNIC::jsonInteger(value, &mut endPoint.port, &fqn, Tag::Transport, key) {
            Ok(_) => {}
            Err(err) => { return Err(err); }
          };
        }
        Tag::VLANId => {
          match TestNIC::jsonInteger(value, &mut endPoint.vlan, &fqn, Tag::Transport, key) {
            Ok(_) => {}
            Err(err) => { return Err(err); }
          };
        }
        _ => {
          log::error!("'{}' object '{}.{}.{}' unknown", Tag::TXQ, fqn, parentKey, key);
          return Err(error::Error::JSONSchema);
        }
      };
    }

    return Ok(());
  }

  fn findAndAddName(parentObj: &JsonValue, name: &mut String, nameMap: &mut HashMap<String, bool>, kind: &str, parentName: &str)
    -> Result<String, error::Error> {
    debug_assert!(parentObj.is_object());

    // Get map from parentObj, and find Name. The direct route
    // parentObj[Tag::Name] risks panic if does not exist
    let map: &HashMap<_, _> = parentObj.get().unwrap();
    if !map.contains_key(Tag::Name) {
      log::error!("'{}' object '{}' missing name", kind, parentName);
      return Err(error::Error::JSONSchema);
    }

    // Make sure name is a non-empty, unique string
    let jsonObj = &parentObj[Tag::Name];
    if jsonObj.is_string() {
      let strRef: Option<&String> = jsonObj.get();
      match strRef {
        Some(val) => {
          if val.len()>0 {
            *name = val.clone();
            let fqn = format!("{}.{}", parentName, val);
            if nameMap.contains_key(&fqn) {
              log::error!("'{}' object '{}' duplicate name", kind, fqn);
              return Err(error::Error::JSONSchema);
            }
            nameMap.insert(fqn.clone(), true);
            return Ok(fqn.clone());
          }
        }
        None => {}
      };
    }

    log::error!("'{}' object in '{}' field '{}' malformed string", kind, parentName, Tag::Name);
    return Err(error::Error::JSONSchema);
  }

  fn parseHugePage(&mut self, parentName: &String, array: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an array
    if !array.is_array() {
      log::error!("'{}' object '{}' not an array", Tag::HugePage, parentName);
      return Err(error::Error::JSONSchema);
    }

    // Get the transportSet for these hugePages
    let transportSet = match self.transportSetVec.last_mut() {
      Some(obj) => obj,
      None => {
        log::error!("'{}' object '{}' internal error: transport set not found", Tag::HugePage, parentName);
        return Err(error::Error::JSONSchema);
      }
    };

    // Visit each item in ary
    let ary: &Vec<_> = array.get().unwrap();
    for item in ary {
      if !item.is_object() {
        log::error!("'{}' object '{}' not an array of objects", Tag::HugePage, parentName);
        return Err(error::Error::JSONSchema);
      }

      // Make fqn for huge page
      let mut name = String::new();
      let fqn = match TestNIC::findAndAddName(item, &mut name, &mut self.nameMap, Tag::HugePage, parentName) {
        Ok(val) => val,
        Err(err) => { return Err(err); }
      };

      log::debug!("parsing  '{}' '{}'", Tag::HugePage, fqn);

      // Create huge page object
      debug_assert!(self.nameMap.contains_key(fqn.as_str()));
      let hp = transportSet.hugePageVec.push_mut(common::HugePage::new());

      // Find all other key-value pairs
      let map: &HashMap<_, _> = item.get().unwrap();
      for (key, value) in map {
        match key.as_str() {
          Tag::Name => {
            match TestNIC::jsonString(value, &mut hp.name, &fqn, Tag::HugePage, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::PageCount => {
            match TestNIC::jsonInteger(value, &mut hp.pageCount, &fqn, Tag::HugePage, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::PageSizeKB => {
            match TestNIC::jsonInteger(value, &mut hp.pageSizeKB, &fqn, Tag::HugePage, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::ByteAlignment => {
            match TestNIC::jsonInteger(value, &mut hp.byteAlignment, &fqn, Tag::HugePage, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          _ => {
            log::error!("'{}' object '{}.{}' unknown", Tag::HugePage, fqn, key);
            return Err(error::Error::JSONSchema);
          }
        }
      }
    }

    return Ok(());
  }

  fn parseHeapAllocator(&mut self, parentName: &String, array: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an array
    if !array.is_array() {
      log::error!("'{}' object '{}' not an array", Tag::HeapAllocator, parentName);
      return Err(error::Error::JSONSchema);
    }

    // Get the transportSet for these heapAllocators
    let transportSet = match self.transportSetVec.last_mut() {
      Some(obj) => obj,
      None => {
        log::error!("'{}' object '{}' internal error: transport set not found", Tag::HeapAllocator, parentName);
        return Err(error::Error::JSONSchema);
      }
    };

    // Visit each item in ary
    let ary: &Vec<_> = array.get().unwrap();
    for item in ary {
      if !item.is_object() {
        log::error!("'{}' object '{}' not an array of objects", Tag::HeapAllocator, parentName);
        return Err(error::Error::JSONSchema);
      }

      // Make fqn for child allocator
      let mut name = String::new();
      let fqn = match TestNIC::findAndAddName(item, &mut name, &mut self.nameMap, Tag::HeapAllocator, parentName) {
        Ok(val) => val,
        Err(err) => { return Err(err); }
      };

      log::debug!("parsing  '{}' '{}'", Tag::HeapAllocator, fqn);

      // Create heap allocator object
      debug_assert!(self.nameMap.contains_key(fqn.as_str()));
      let hp = transportSet.heapAllocVec.push_mut(common::HeapAllocator::new());

      // Find all other key-value pairs
      let map: &HashMap<_, _> = item.get().unwrap();
      for (key, value) in map {
        match key.as_str() {
          Tag::Name => {
            match TestNIC::jsonString(value, &mut hp.name, &fqn, Tag::HugePage, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::SizeKB => {
            match TestNIC::jsonInteger(value, &mut hp.sizeKB, &fqn, Tag::HeapAllocator, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::ByteAlignment => {
            match TestNIC::jsonInteger(value, &mut hp.byteAlignment, &fqn, Tag::HeapAllocator, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          _ => {
            log::error!("'{}' object '{}.{}' unknown", Tag::HeapAllocator, fqn, key);
            return Err(error::Error::JSONSchema);
          }
        }
      }
    }

    return Ok(());
  }

  fn parseChildAllocator(&mut self, parentName: &String, array: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an array
    if !array.is_array() {
      log::error!("'{}' object '{}' not an array", Tag::ChildAllocator, parentName);
      return Err(error::Error::JSONSchema);
    }

    // Get the transportSet for these childAllocators
    let transportSet = match self.transportSetVec.last_mut() {
      Some(obj) => obj,
      None => {
        log::error!("'{}' object '{}' internal error: transport set not found", Tag::ChildAllocator, parentName);
        return Err(error::Error::JSONSchema);
      }
    };

    // Visit each item in ary
    let ary: &Vec<_> = array.get().unwrap();
    for item in ary {
      if !item.is_object() {
        log::error!("'{}' object '{}' not an array of objects", Tag::ChildAllocator, parentName);
        return Err(error::Error::JSONSchema);
      }

      // Make fqn for child allocator
      let mut name = String::new();
      let fqn = match TestNIC::findAndAddName(item, &mut name, &mut self.nameMap, Tag::ChildAllocator, parentName) {
        Ok(val) => val,
        Err(err) => { return Err(err); }
      };

      log::debug!("parsing  '{}' '{}'", Tag::ChildAllocator, fqn);

      // Create child allocator object
      debug_assert!(self.nameMap.contains_key(fqn.as_str()));
      let hp = transportSet.childAllocVec.push_mut(common::ChildAllocator::new());

      // Find all other key-value pairs
      let map: &HashMap<_, _> = item.get().unwrap();
      for (key, value) in map {
        match key.as_str() {
          Tag::Name => {
            match TestNIC::jsonString(value, &mut hp.name, &fqn, Tag::HugePage, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::ParentName => {
            match TestNIC::jsonString(value, &mut hp.parentName, &fqn, Tag::ChildAllocator, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::SizeKB => {
            match TestNIC::jsonInteger(value, &mut hp.sizeKB, &fqn, Tag::ChildAllocator, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::ByteAlignment => {
            match TestNIC::jsonInteger(value, &mut hp.byteAlignment, &fqn, Tag::ChildAllocator, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          _ => {
            log::error!("'{}' object '{}.{}' unknown", Tag::ChildAllocator, fqn, key);
            return Err(error::Error::JSONSchema);
          }
        }
      }
    }

    return Ok(());
  }

  fn parseSRPT(&mut self, parentName: &String, array: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an array
    if !array.is_array() {
      log::error!("'{}' object '{}' not an array", Tag::SRPT, parentName);
      return Err(error::Error::JSONSchema);
    }

    // Get the transportSet for these childAllocators
    let transportSet = match self.transportSetVec.last_mut() {
      Some(obj) => obj,
      None => {
        log::error!("'{}' object '{}' internal error: transport set not found", Tag::SRPT, parentName);
        return Err(error::Error::JSONSchema);
      }
    };

    // Visit each item in ary
    let ary: &Vec<_> = array.get().unwrap();
    for item in ary {
      if !item.is_object() {
        log::error!("'{}' object '{}' not an array of objects", Tag::SRPT, parentName);
        return Err(error::Error::JSONSchema);
      }

      // Make fqn for SRPT
      let mut name = String::new();
      let fqn = match TestNIC::findAndAddName(item, &mut name, &mut self.nameMap, Tag::SRPT, parentName) {
        Ok(val) => val,
        Err(err) => { return Err(err); }
      };

      log::debug!("parsing  '{}' '{}'", Tag::SRPT, fqn);

      // Create SRPT object
      debug_assert!(self.nameMap.contains_key(fqn.as_str()));
      let hp = transportSet.srptVec.push_mut(common::SRPT::new());
      hp.name = name;

      // Process required SRPT fields
      let map: &HashMap<_, _> = item.get().unwrap();
      for (key, value) in map {
        match key.as_str() {
          Tag::Name => {}
          Tag::Capacity => {
            match TestNIC::jsonInteger(value, &mut hp.capacity, &fqn, Tag::SRPT, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::OverCommitmentCount => {
            match TestNIC::jsonInteger(value, &mut hp.overCommitmentCount, &fqn, Tag::SRPT, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::ResponseRingCount => {
            match TestNIC::jsonInteger(value, &mut hp.responseRingCount, &fqn, Tag::SRPT, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::RequestRingCount => {
            match TestNIC::jsonInteger(value, &mut hp.requestRingCount, &fqn, Tag::SRPT, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::CPU => {
            match TestNIC::jsonInteger(value, &mut hp.cpu, &fqn, Tag::SRPT, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::AllocatorName => {
            match TestNIC::jsonString(value, &mut hp.allocatorName, &fqn, Tag::SRPT, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::UnscheduledPriority => {
            match TestNIC::jsonArray(value, &mut hp.unscheduledPriority, &fqn, Tag::SRPT, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::ScheduledPriority => {
            match TestNIC::jsonArray(value, &mut hp.scheduledPriority, &fqn, Tag::SRPT, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          _ => {
            log::error!("'{}' object '{}.{}' unexpected", Tag::SRPT, fqn, key);
            return Err(error::Error::JSONSchema);
          }
        }
      }
    }

    return Ok(());
  }

  fn parseNIC(&mut self, parentName: &String, obj: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an object
    if !obj.is_object() {
      log::error!("'{}' object '{}' not an object", Tag::NIC, parentName);
      return Err(error::Error::JSONSchema);
    }

    // Get the transportSet for this NIC
    let transportSet = match self.transportSetVec.last_mut() {
      Some(obj) => obj,
      None => {
        log::error!("'{}' object '{}' internal error: transport set not found", Tag::NIC, parentName);
        return Err(error::Error::JSONSchema);
      }
    };

    // Make fqn for NIC
    let mut name = String::new();
    let fqn = match TestNIC::findAndAddName(obj, &mut name, &mut self.nameMap, Tag::NIC, parentName) {
      Ok(val) => val,
      Err(err) => { return Err(err); }
    };

    log::debug!("parsing  '{}' '{}'", Tag::NIC, fqn);

    // Create NIC
    debug_assert!(self.nameMap.contains_key(fqn.as_str()));
    transportSet.nic.name = name;

    // Process required NIC fields
    let map: &HashMap<_, _> = obj.get().unwrap();
    for (key, value) in map {
      match key.as_str() {
        Tag::Name => {}
        Tag::MACAddress => {
          match TestNIC::jsonString(value, &mut transportSet.nic.macAddress, &fqn, Tag::NIC, key) {
            Ok(_) => {}
            Err(err) => { return Err(err); }
          };
        }
        Tag::IPV4Address => {
          match TestNIC::jsonString(value, &mut transportSet.nic.ipv4Address, &fqn, Tag::NIC, key) {
            Ok(_) => {}
            Err(err) => { return Err(err); }
          };
        }
        Tag::IPV6Address => {
          match TestNIC::jsonString(value, &mut transportSet.nic.ipv6Address, &fqn, Tag::NIC, key) {
            Ok(_) => {}
            Err(err) => { return Err(err); }
          };
        }
        Tag::PciDeviceId => {
          match TestNIC::jsonString(value, &mut transportSet.nic.pciAddress, &fqn, Tag::NIC, key) {
            Ok(_) => {}
            Err(err) => { return Err(err); }
          };
        }
        Tag::MTUSizeBytes => {
          match TestNIC::jsonInteger(value, &mut transportSet.nic.mtuSizeBytes, &fqn, Tag::NIC, key) {
            Ok(_) => {}
            Err(err) => { return Err(err); }
          };
        }
        Tag::LinkSpeedGbit => {
          match TestNIC::jsonInteger(value, &mut transportSet.nic.linkSpeedGbit, &fqn, Tag::NIC, key) {
            Ok(_) => {}
            Err(err) => { return Err(err); }
          };
        }
        Tag::MaximumTransports => {
          match TestNIC::jsonInteger(value, &mut transportSet.nic.maxTransports, &fqn, Tag::NIC, key) {
            Ok(_) => {}
            Err(err) => { return Err(err); }
          };
        }
        _ => {
          log::error!("'{}' object '{}.{}' unexpected", Tag::NIC, fqn, key);
          return Err(error::Error::JSONSchema);
        }
      }
    }

    return Ok(());
  }

  fn parseNICRxq(&mut self, parentName: &String, array: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an array
    if !array.is_array() {
      log::error!("'{}' object '{}' not an array", Tag::RXQ, parentName);
      return Err(error::Error::JSONSchema);
    }

    // Get the transportSet for this NIC queue
    let transportSet = match self.transportSetVec.last_mut() {
      Some(obj) => obj,
      None => {
        log::error!("'{}' object '{}' internal error: transport set not found", Tag::RXQ, parentName);
        return Err(error::Error::JSONSchema);
      }
    };

    // Visit each item in ary
    let ary: &Vec<_> = array.get().unwrap();
    for item in ary {
      if !item.is_object() {
        log::error!("'{}' object '{}' not an array of objects", Tag::RXQ, parentName);
        return Err(error::Error::JSONSchema);
      }

      // Make fqn for NIC queue
      let mut name = String::new();
      let fqn = match TestNIC::findAndAddName(item, &mut name, &mut self.nameMap, Tag::RXQ, parentName) {
        Ok(val) => val,
        Err(err) => { return Err(err); }
      };

      log::debug!("parsing  '{}' '{}'", Tag::RXQ, fqn);

      // Create RXQ queue object
      debug_assert!(self.nameMap.contains_key(fqn.as_str()));
      let hp = transportSet.rxqVec.push_mut(NICQueue::new());
      hp.name = name;

      // Find all other key-value pairs
      let map: &HashMap<_, _> = item.get().unwrap();
      for (key, value) in map {
        match key.as_str() {
          Tag::Name => {}
          Tag::RingSize => {
            match TestNIC::jsonInteger(value, &mut hp.ringSize, &fqn, Tag::RXQ, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::AllocatorName => {
            match TestNIC::jsonString(value, &mut hp.allocatorName, &fqn, Tag::RXQ, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          _ => {
            log::error!("'{}' object '{}.{}' unknown", Tag::RXQ, fqn, key);
            return Err(error::Error::JSONSchema);
          }
        }
      }
    }

    return Ok(());
  }

  fn parseNICTxq(&mut self, parentName: &String, array: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an array
    if !array.is_array() {
      log::error!("'{}' object '{}' not an array", Tag::TXQ, parentName);
      return Err(error::Error::JSONSchema);
    }

    // Get the transportSet for this NIC queue
    let transportSet = match self.transportSetVec.last_mut() {
      Some(obj) => obj,
      None => {
        log::error!("'{}' object '{}' internal error: transport set not found", Tag::TXQ, parentName);
        return Err(error::Error::JSONSchema);
      }
    };

    // Visit each item in ary
    let ary: &Vec<_> = array.get().unwrap();
    for item in ary {
      if !item.is_object() {
        log::error!("'{}' object '{}' not an array of objects", Tag::TXQ, parentName);
        return Err(error::Error::JSONSchema);
      }

      // Make fqn for NIC queue
      let mut name = String::new();
      let fqn = match TestNIC::findAndAddName(item, &mut name, &mut self.nameMap, Tag::TXQ, parentName) {
        Ok(val) => val,
        Err(err) => { return Err(err); }
      };

      log::debug!("parsing  '{}' '{}'", Tag::TXQ, fqn);

      // Create TXQ queue object
      debug_assert!(self.nameMap.contains_key(fqn.as_str()));
      let hp = transportSet.txqVec.push_mut(NICQueue::new());

      // Find all other key-value pairs
      let map: &HashMap<_, _> = item.get().unwrap();
      for (key, value) in map {
        match key.as_str() {
          Tag::Name => {
            match TestNIC::jsonString(value, &mut hp.name, &fqn, Tag::TXQ, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::RingSize => {
            match TestNIC::jsonInteger(value, &mut hp.ringSize, &fqn, Tag::TXQ, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::AllocatorName => {
            match TestNIC::jsonString(value, &mut hp.allocatorName, &fqn, Tag::TXQ, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          _ => {
            log::error!("'{}' object '{}.{}' unknown", Tag::TXQ, fqn, key);
            return Err(error::Error::JSONSchema);
          }
        }
      }
    }

    return Ok(());
  }

  fn parseTransport(&mut self, parentName: &String, array: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an array
    if !array.is_array() {
      log::error!("'{}' object '{}' not an array", Tag::Transport, parentName);
      return Err(error::Error::JSONSchema);
    }

    // Get the transportSet for transports
    let transportSet = match self.transportSetVec.last_mut() {
      Some(obj) => obj,
      None => {
        log::error!("'{}' object '{}' internal error: transport set not found", Tag::RXQ, parentName);
        return Err(error::Error::JSONSchema);
      }
    };

    // Visit each item in ary
    let ary: &Vec<_> = array.get().unwrap();
    for item in ary {
      if !item.is_object() {
        log::error!("'{}' object '{}' not an array of objects", Tag::Transport, parentName);
        return Err(error::Error::JSONSchema);
      }

      // Make fqn for transport
      let mut name = String::new();
      let fqn = match TestNIC::findAndAddName(item, &mut name, &mut self.nameMap, Tag::Transport, parentName) {
        Ok(val) => val,
        Err(err) => { return Err(err); }
      };

      log::debug!("parsing  '{}' '{}'", Tag::Transport, fqn);

      // Create transport
      debug_assert!(self.nameMap.contains_key(fqn.as_str()));
      let hp = transportSet.transportVec.push_mut(Transport::new());
      hp.name = name;

      // Process required transport fields
      let map: &HashMap<_, _> = item.get().unwrap();
      for (key, value) in map {
        match key.as_str() {
          Tag::Name => {}
          Tag::CPU => {
            match TestNIC::jsonInteger(value, &mut hp.cpu, &fqn, Tag::Transport, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::ReadyCapacity => {
            match TestNIC::jsonInteger(value, &mut hp.readyCapacity, &fqn, Tag::Transport, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::ReserveCapacity => {
            match TestNIC::jsonInteger(value, &mut hp.reserveCapacity, &fqn, Tag::Transport, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::CallbackCapacity => {
            match TestNIC::jsonInteger(value, &mut hp.callbackCapacity, &fqn, Tag::Transport, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::IPV4Endpoint => {
            match TestNIC::parseEndpoint(value, &mut hp.ipv4Suffix, &fqn, Tag::Transport, key, true) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::IPV6Endpoint => {
            match TestNIC::parseEndpoint(value, &mut hp.ipv6Suffix, &fqn, Tag::Transport, key, false) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::ErrorIPV4Endpoint => {
            match TestNIC::parseEndpoint(value, &mut hp.ipv4ErrorSuffix, &fqn, Tag::Transport, key, true) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::ErrorIPV6Endpoint => {
            match TestNIC::parseEndpoint(value, &mut hp.ipv6ErrorSuffix, &fqn, Tag::Transport, key, false) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::QueuePair => {
            if !value.is_array() {
              log::error!("'{}' object '{}.{}' not array", Tag::Transport, fqn, key);
              return Err(error::Error::JSONSchema);
            }
            let subAry: &Vec<_> = value.get().unwrap();
            for subItem in subAry {
              if !subItem.is_object() {
                log::error!("'{}' object '{}.{}' not an array of objects", Tag::Transport, fqn, key);
                return Err(error::Error::JSONSchema);
              }
              let mut subHp = NICQueuePair::new(); 
              let subMap: &HashMap<_, _> = subItem.get().unwrap();
              for (subKey, subValue) in subMap {
                match subKey.as_str() {
                  Tag::RXQName => {
                    match TestNIC::jsonString(subValue, &mut subHp.rxq, &fqn, Tag::Transport, subKey) {
                      Ok(_) => {}
                      Err(err) => { return Err(err); }
                    };
                  }
                  Tag::TXQName => {
                    match TestNIC::jsonString(subValue, &mut subHp.txq, &fqn, Tag::Transport, subKey) {
                      Ok(_) => {}
                      Err(err) => { return Err(err); }
                    };
                  }
                  _ => {
                    log::error!("'{}' object '{}.{}.{}' unknown", Tag::Transport, fqn, key, subKey);
                    return Err(error::Error::JSONSchema);
                  }
                };
              }
              // Save it
              hp.queuePair.push(subHp);
            }
          }
          Tag::AllocatorName => {
            match TestNIC::jsonString(value, &mut hp.allocatorName, &fqn, Tag::TXQ, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          Tag::SRPTScheduleName => {
            match TestNIC::jsonString(value, &mut hp.srptScheduleName, &fqn, Tag::TXQ, key) {
              Ok(_) => {}
              Err(err) => { return Err(err); }
            };
          }
          _ => {
            log::error!("'{}' object '{}.{}' unknown", Tag::Transport, fqn, key);
            return Err(error::Error::JSONSchema);
          }
        }
      }
    }

    return Ok(());
  }

  fn parseTransportSet(&mut self, item: &JsonValue) -> Result<(), error::Error> {
    debug_assert!(item.is_object());

    let parentName = "root";
    let mut name = String::new();
    let fqn = match TestNIC::findAndAddName(item, &mut name, &mut self.nameMap, Tag::TransportSet, &parentName) {
      Ok(val) => val,
      Err(err) => { return Err(err); }
    };

    // Append new transportSet
    self.transportSetVec.push(TransportSet::new());
    // Get the transportSet for transports
    match self.transportSetVec.last_mut() {
      Some(obj) => { obj.name = fqn.clone(); }
      None => {
        log::error!("'{}' object '{}' internal error: transport set not found", fqn, parentName);
        return Err(error::Error::JSONSchema);
      }
    };

    log::debug!("parsing '{}'", fqn);

    // Get inner object then find, parse sub-objects
    let map: &HashMap<_, _> = item.get().unwrap();

    // Parse HugePageAllocator(s)
    if map.contains_key(Tag::HugePage) {
      match self.parseHugePage(&fqn, &item[Tag::HugePage]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    // Parse HeapAllocators
    if map.contains_key(Tag::HeapAllocator) {
      match self.parseHeapAllocator(&fqn, &item[Tag::HeapAllocator]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    // Parse ChildAllocators
    if map.contains_key(Tag::ChildAllocator) {
      match self.parseChildAllocator(&fqn, &item[Tag::ChildAllocator]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    // Parse SRPT
    if map.contains_key(Tag::SRPT) {
      match self.parseSRPT(&fqn, &item[Tag::SRPT]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    // Parse NIC
    if map.contains_key(Tag::NIC) {
      match self.parseNIC(&fqn, &item[Tag::NIC]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    // Parse RXQ
    if map.contains_key(Tag::RXQ) {
      match self.parseNICRxq(&fqn, &item[Tag::RXQ]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    // Parse TXQ
    if map.contains_key(Tag::TXQ) {
      match self.parseNICTxq(&fqn, &item[Tag::TXQ]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    // Parse Transport
    if map.contains_key(Tag::Transport) {
      match self.parseTransport(&fqn, &item[Tag::Transport]) {
        Ok(_) => {},
        Err(err) => { return Err(err); }
      };
    }

    return Ok(());
  }

  fn verifyObjects(&self) -> Result<(), error::Error> {
    for transportSet in &self.transportSetVec {
      match transportSet.nic.verify(self) {
        Ok(_) => {},
        Err(err) => {
          log::error!("object '{}.{}' invalid contents: {:?}", transportSet.name, transportSet.nic.name, err);
          return Err(err);
        }
      }

      for item in &transportSet.rxqVec {
        match item.verify() {
          Ok(_) => {},
          Err(err) => {
            log::error!("object '{}.{}' invalid contents: {:?}", transportSet.name, item.name, err);
            return Err(err);
          }
        }
      }

      for item in &transportSet.txqVec {
        match item.verify() {
          Ok(_) => {},
          Err(err) => {
            log::error!("object '{}.{}' invalid contents: {:?}", transportSet.name, item.name, err);
            return Err(err);
          }
        }
      }

      for item in &transportSet.srptVec {
        match item.verify() {
          Ok(_) => {},
          Err(err) => {
            log::error!("object '{}.{}' invalid contents: {:?}", transportSet.name, item.name, err);
            return Err(err);
          }
        }
      }

      for item in &transportSet.hugePageVec {
        match item.verify() {
          Ok(_) => {},
          Err(err) => {
            log::error!("object '{}.{}' invalid contents: {:?}", transportSet.name, item.name, err);
            return Err(err);
          }
        }
      }

      for item in &transportSet.heapAllocVec {
        match item.verify() {
          Ok(_) => {},
          Err(err) => {
            log::error!("object '{}.{}' invalid contents: {:?}", transportSet.name, item.name, err);
            return Err(err);
          }
        }
      }

      for item in &transportSet.childAllocVec {
        match item.verify() {
          Ok(_) => {},
          Err(err) => {
            log::error!("object '{}.{}' invalid contents: {:?}", transportSet.name, item.name, err);
            return Err(err);
          }
        }
      }

      for item in &transportSet.transportVec {
        match item.verify() {
          Ok(_) => {},
          Err(err) => {
            log::error!("object '{}.{}' invalid contents: {:?}", transportSet.name, item.name, err);
            return Err(err);
          }
        }
      }
    }

    return Ok(());
  }

  fn numaLookups(&mut self) -> Result<(), error::Error> {
    for transportSet in &mut self.transportSetVec {
      match util::pciNumaNode(&transportSet.nic.pciAddress) {
        Ok(val) => { transportSet.nic.numaNode = val; },
        Err(err) => {
          log::error!("object '{}' NIC '{}' invalid pciAddress: {:?}", transportSet.name, transportSet.nic.name, err);
          return Err(err);
        }
      }

      for item in &mut transportSet.srptVec {
        match util::isCpuOnNumaNode(item.cpu, transportSet.nic.numaNode) {
          Ok(val) => {},
          Err(err) => {
            log::error!("object '{}' srptSchedule '{}' cpu {} invalid or not on NIC numaNode {}: {:?}", transportSet.name, item.name,
              item.cpu, transportSet.nic.numaNode, err);
            return Err(err);
          }
        };
      }

      for item in &mut transportSet.transportVec {
        match util::isCpuOnNumaNode(item.cpu, transportSet.nic.numaNode) {
          Ok(val) => {},
          Err(err) => {
            log::error!("object '{}' transport '{}' cpu {} invalid or not on NIC numaNode {}: {:?}", transportSet.name, item.name,
              item.cpu, transportSet.nic.numaNode, err);
            return Err(err);
          }
        };
      }
    }

    return Ok(());
  }
}

impl Verify for TestNIC {
  fn verify(&mut self, obj: &JsonValue) -> Result<(), error::Error> {
    // Make sure it's an object
    if !obj.is_object() {
      log::error!("malformed JSON");
      return Err(error::Error::JSONSchema);
    }

    // Make sure 'TransportSet' exists
    let map: &HashMap<_, _> = obj.get().unwrap();
    if !map.contains_key(Tag::TransportSet) {
      log::error!("missing '{}' object", Tag::TransportSet);
      return Err(error::Error::JSONSchema);
    }

    // Make sure TransportSet is an array
    let transportSet = &obj[Tag::TransportSet];
    if !transportSet.is_array() {
      log::error!("'{}' object not an array", Tag::TransportSet);
      return Err(error::Error::JSONSchema);
    }

    // Make sure array has 1+ elements
    let ary: &Vec<_> = transportSet.get().unwrap();
    if ary.len()==0 {
      log::error!("'{}' object empty array", Tag::TransportSet);
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

    // Do NUMA lookups
    match self.numaLookups() {
      Ok(_) => {},
      Err(err) => { return Err(err); }
    };

    // Do simple verifications not needing cross-verify
    match self.verifyObjects() {
      Ok(_) => {},
      Err(err) => { return Err(err); }
    };

    return Ok(());
  }
}
