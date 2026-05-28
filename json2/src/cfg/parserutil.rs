use std::collections::HashMap;

struct HugePage {                                                                    
  d_pageCount: u32,                                                                                 
  d_pageSizeKB: u32,                                                                                    
  d_byteAlignment: u32,                                                                                 
}

struct HeapAllocator {                                                                    
  d_sizeKB: u32,                                                                                    
  d_byteAlignment: u32,                                                                                 
}

struct ChildAllocator {                                                                    
  d_parentName: String,
  d_sizeKB: u32,                                                                                    
  d_byteAlignment: u32,                                                                                 
  d_parentObject: HugePage,
}

struct SRPT {
  d_capacity: u32,
  d_overCommitmentCount: u32,
  d_responseRingCount: u32,
  d_requestRingCount: u32,
  d_unscheduledPriority: [u32; 6];
  d_scheduledPriority: [u32; 2];
  d_allocatorName: String,
  d_cpuHwCore: u32,
}

struct NIC {
  d_mac: String,
  d_srptName: String,
  d_ipv4Address: String,
  d_ipv6Address: String,
  d_pciAddress: String,
  d_mtuSizeBytes: u32,
  d_linkSpeedGbit: u32,
  d_maxTransports: u32,
  d_numaNode: u32,
}

struct NICQueue {
  d_ringSize: u32,
  d_allocatorName: String,
}

struct NICQueuePair {
  d_rxq: NICQueue,
  d_txq: NICQueue,
}

struct VLANPort {
  d_port: u32,
  d_vlan: u32,
}

struct Transport {
  d_nic: NIC,
  d_rxQueuePair: &Vec<NICQueue>,
  d_txQueuePair: &Vec<NICQueue>,
  d_ipv4Suffix: VLANPort,
  d_ipv6Suffix: VLANPort,
  d_ipv4ErrorSuffix: VLANPort,
  d_ipv6ErrorSuffix: VLANPort,
  d_callbackCapacity: u32,
  d_readyCapacity: u32,
  d_reserveCapacity: u32,
  d_allocatorName: String,
  d_cpuHwCore: u32,
}

pub struct ParserUtil {
  nicMap: HashMap<String, NIC>,
  hugePageMap: HashMap<String, HugePage>,
  heapAllocMap: HashMap<String, HeapAllocator>,
  childAllocMap: HashMap<String, ChildAllocator>,
  transportMap: HashMap<String, Transport>,
}
