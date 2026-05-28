struct Constant;

#[allow(non_upper_case_globals)]
impl Constant {
  const CPUCoreMin: u32 = 0;
  const CPUCoreMax: u32 = 255;
  const VLANIdMin: u32 = 0;
  const VLANIdMax: u32 = 4095;
  const NUMANodeMin: u32 = 0;
  const NUMANodeMax: u32 = 8;
  const IPV4PortMin: u32 = 1024;
  const IPV4PortMax: u32 = 65535;
  const TransportsMin: u32 = 1;
  const TransportsMax: u32 = 8;
  const RingCountMin: u32 = 4;
  const RingCountMax: u32 = 32768;
  const MTUSizeBytesMin: u32 = 1;
  const MTUSizeBytesMax: u32 = 65535;
  const SRPTCapacityMin: u32 = 4;
  const SRPTCapacityMax: u32 = 128;
  const ByteAlignmentMin: u32 = 8;
  const ByteAlignmentMax: u32 = 512;
  const SRPTScheduledPriorities: u32 = 2;
  const SRPTUnscheduledPriorities: u32 = 6;
  const SRPTOverCommitmentCountMin: u32 = 0;
  const SRPTOverCommitmentCountMax: u32 = 8;
  const RPCReadyCapacityMin: u32 = 8;
  const RPCReadyCapacityMax: u32 = 256;
  const RPCReserveCapacityMin: u32 = 8;
  const RPCReserveCapacityMax: u32 = 256;
  const RPCCallbackCapacityMin: u32 = 1;
  const RPCCallbackCapacityMax: u32 = 8;
  const JSONConfigFileSizeBytesMax: u32 = 8192;
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
