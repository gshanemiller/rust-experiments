use std::collections::HashMap;

struct HugePage {                                                                    
  pageCount: u32,                                                                                 
  pageSizeKB: u32,                                                                                    
  byteAlignment: u32,                                                                                 
}

struct HeapAllocator {                                                                    
  sizeKB: u32,                                                                                    
  byteAlignment: u32,                                                                                 
}

struct ChildAllocator {                                                                    
  parentName: String,
  sizeKB: u32,                                                                                    
  byteAlignment: u32,                                                                                 
  parentObject: HugePage,
}

struct SRPT {
  capacity: u32,
  overCommitmentCount: u32,
  responseRingCount: u32,
  requestRingCount: u32,
  unscheduledPriority: [u32; 6];
  scheduledPriority: [u32; 2];
  allocatorName: String,
  cpuHwCore: u32,
}

struct NIC {
  mac: String,
  srptName: String,
  ipv4Address: String,
  ipv6Address: String,
  pciAddress: String,
  mtuSizeBytes: u32,
  linkSpeedGbit: u32,
  maxTransports: u32,
  numaNode: u32,
}

struct NICQueue {
  ringSize: u32,
  allocatorName: String,
}

struct NICQueuePair {
  rxq: NICQueue,
  txq: NICQueue,
}

struct VLANPort {
  port: u32,
  vlan: u32,
}

struct Transport {
  nic: NIC,
  rxQueuePair: &Vec<NICQueue>,
  txQueuePair: &Vec<NICQueue>,
  ipv4Suffix: VLANPort,
  ipv6Suffix: VLANPort,
  ipv4ErrorSuffix: VLANPort,
  ipv6ErrorSuffix: VLANPort,
  callbackCapacity: u32,
  readyCapacity: u32,
  reserveCapacity: u32,
  allocatorName: String,
  cpuHwCore: u32,
}

pub struct ParserUtil {
  nicMap: HashMap,
  hugePageMap: HashMap,
  heapAllocMap: HashMap,
  childAllocMap: HashMap,
  transportMap: HashMap,
}
