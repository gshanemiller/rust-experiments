pub struct Constant;

#[allow(non_upper_case_globals)]
impl Constant {
  pub const CPUCoreMin: u32 = 0;
  pub const CPUCoreMax: u32 = 255;
  pub const VLANIdMin: u32 = 0;
  pub const VLANIdMax: u32 = 4095;
  pub const NUMANodeMin: u32 = 0;
  pub const NUMANodeMax: u32 = 8;
  pub const IPV4PortMin: u32 = 1024;
  pub const IPV4PortMax: u32 = 65535;
  pub const TransportsMin: u32 = 1;
  pub const TransportsMax: u32 = 8;
  pub const RingCountMin: u32 = 4;
  pub const RingCountMax: u32 = 32768;
  pub const MTUSizeBytesMin: u32 = 128;
  pub const MTUSizeBytesMax: u32 = 65535;
  pub const SRPTCapacityMin: u32 = 4;
  pub const SRPTCapacityMax: u32 = 128;
  pub const ByteAlignmentMin: u32 = 8;
  pub const ByteAlignmentMax: u32 = 512;
  pub const SRPTRingCountMin: u32 = 1;
  pub const SRPTRingCountMax: u32 = 8;
  pub const SRPTScheduledPriorities: u32 = 2;
  pub const SRPTUnscheduledPriorities: u32 = 6;
  pub const SRPTOverCommitmentCountMin: u32 = 0;
  pub const SRPTOverCommitmentCountMax: u32 = 8;
  pub const RPCReadyCapacityMin: u32 = 8;
  pub const RPCReadyCapacityMax: u32 = 256;
  pub const RPCReserveCapacityMin: u32 = 8;
  pub const RPCReserveCapacityMax: u32 = 256;
  pub const RPCCallbackCapacityMin: u32 = 1;
  pub const RPCCallbackCapacityMax: u32 = 8;
  pub const JSONConfigFileSizeBytesMax: u32 = 8192;
}

#[allow(non_upper_case_globals)]
pub const LinkBandWidthGbitPerSec: [u32; 10] = [
  0,
  1,
  5,
  10,
  20,
  25,
  50,
  100,
  200,
  400,
];

#[allow(non_upper_case_globals)]
pub const HugePageSizeKB: [u32; 2] = [
  2048,
  1048576,
];
