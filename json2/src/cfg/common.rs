use crate::cfg::limit;
use crate::err::error;

#[allow(non_snake_case)]
pub struct HugePage {
  pub pageCount: u32,
  pub pageSizeKB: u32,
  pub byteAlignment: u32,
  pub numaNode: u32,
}

impl HugePage {
  pub fn new() -> Self {
    Self {
      pageCount: 0,
      pageSizeKB: 0,
      byteAlignment: 0,
      numaNode: 0,
    }
  }

  pub fn verify(&self) -> Result<(), error::Error> {
    let mut ret = true;
    let mut found = false;

    ret = ret && self.pageCount>=limit::Constant::NUMANodeMin;
    ret = ret && self.pageCount<=limit::Constant::NUMANodeMax;
    ret = ret && self.byteAlignment>=limit::Constant::ByteAlignmentMin;
    ret = ret && self.byteAlignment<=limit::Constant::ByteAlignmentMax;
    ret = ret && self.numaNode>=limit::Constant::NUMANodeMin;
    ret = ret && self.numaNode<=limit::Constant::NUMANodeMax;

    for pgSz in limit::HugePageSizeKB {
      if pgSz==self.pageSizeKB {
        found = true;
        break;
      }
    }
    ret = ret && found;

    if ret {
      return Ok(());
    } else {
      return Err(error::Error::OutOfRange);
    }
  }
}

#[allow(non_snake_case)]
pub struct HeapAllocator {
  pub sizeKB: u32,
  pub byteAlignment: u32,
}

impl HeapAllocator {
  pub fn new() -> Self {
    Self {
      sizeKB: 0,
      byteAlignment: 0,
    }
  }

  pub fn verify(&self) -> Result<(), error::Error> {
    let mut ret = true;

    ret = ret && self.sizeKB>0;
    ret = ret && self.byteAlignment>=limit::Constant::ByteAlignmentMin;
    ret = ret && self.byteAlignment<=limit::Constant::ByteAlignmentMax;

    if ret {
      return Ok(());
    } else {
      return Err(error::Error::OutOfRange);
    }
  }
}

#[allow(non_snake_case)]
pub struct ChildAllocator {
  pub parentName: String,
  pub sizeKB: u32,
  pub byteAlignment: u32,
}

impl ChildAllocator {
  pub fn new() -> Self {
    Self {
      parentName: String::new(),
      sizeKB: 0,
      byteAlignment: 0,
    }
  }

  pub fn verify(&self) -> Result<(), error::Error> {
    let mut ret = true;

    ret = ret && self.parentName.len()>0;
    ret = ret && self.sizeKB>0;
    ret = ret && self.byteAlignment>=limit::Constant::ByteAlignmentMin;
    ret = ret && self.byteAlignment<=limit::Constant::ByteAlignmentMax;

    if ret {
      return Ok(());
    } else {
      return Err(error::Error::OutOfRange);
    }
  }

  pub fn crossVerify(&self) -> Result<(), error::Error> {
    return Ok(());
  }
}

#[allow(non_snake_case)]
pub struct SRPT {
  pub capacity: u32,
  pub overCommitmentCount: u32,
  pub responseRingCount: u32,
  pub requestRingCount: u32,
  pub unscheduledPriority: [u32; 6],
  pub scheduledPriority: [u32; 2],
  pub allocatorName: String,
  pub cpuHwCore: u32,
}

impl SRPT {
  pub fn new() -> Self {
    Self {
      capacity: 0,
      overCommitmentCount: 0,
      responseRingCount: 0,
      requestRingCount: 0,
      unscheduledPriority: [0,0,0,0,0,0],
      scheduledPriority: [0,0],
      allocatorName: String::new(),
      cpuHwCore: 0,
    }
  }

  pub fn verify(&self) -> Result<(), error::Error> {
    let mut ret = true;

    ret = ret && self.capacity>=limit::Constant::SRPTCapacityMin;
    ret = ret && self.capacity<=limit::Constant::SRPTCapacityMax;
    ret = ret && self.overCommitmentCount>=limit::Constant::SRPTOverCommitmentCountMin;
    ret = ret && self.overCommitmentCount<=limit::Constant::SRPTOverCommitmentCountMax;
    ret = ret && self.requestRingCount>=limit::Constant::SRPTRingCountMin;
    ret = ret && self.requestRingCount<=limit::Constant::SRPTRingCountMax;
    ret = ret && self.responseRingCount>=limit::Constant::SRPTRingCountMin;
    ret = ret && self.responseRingCount<=limit::Constant::SRPTRingCountMax;
    ret = ret && self.allocatorName.len()>0;
    ret = ret && self.cpuHwCore>=limit::Constant::CPUCoreMin;
    ret = ret && self.cpuHwCore<=limit::Constant::CPUCoreMax;

    // Make sure non-zero increasing only
    for (i, val) in self.unscheduledPriority.iter().enumerate() {
      ret = ret && *val>0;
      if i>0 {
        ret = ret && *val>self.unscheduledPriority[i-1];
      }
    }
    for (i, val) in self.scheduledPriority.iter().enumerate() {
      ret = ret && *val>0;
      if i>0 {
        ret = ret && *val>self.scheduledPriority[i-1];
      }
    }
    ret = ret && self.scheduledPriority[0] > self.unscheduledPriority[5];
    ret = ret && self.scheduledPriority[1]==0xffffffff;

    if ret {
      return Ok(());
    } else {
      return Err(error::Error::OutOfRange);
    }
  }

  pub fn crossVerify(&self) -> Result<(), error::Error> {
    return Ok(());
  }
}

#[allow(non_snake_case)]
pub struct VLANPort {
  pub port: u32,
  pub vlan: u32,
}

impl VLANPort {
  pub fn new() -> Self {
    Self {
      port: 0,
      vlan: 0,
    }
  }

  pub fn verify(&self) -> Result<(), error::Error> {
    let mut ret = true;

    ret = ret && self.port>=limit::Constant::IPV4PortMin;
    ret = ret && self.port<=limit::Constant::IPV4PortMax;
    ret = ret && self.vlan>=limit::Constant::VLANIdMin;
    ret = ret && self.vlan<=limit::Constant::VLANIdMax;

    if ret {
      return Ok(());
    } else {
      return Err(error::Error::OutOfRange);
    }
  }
}
