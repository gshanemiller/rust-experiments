use std::io;
use std::num;

#[derive(Debug)]
pub enum Error {
  JSONError,
  JSONSchema,
  NoCapacity,
  NoFreeMemory,
  NumaMismatch,
  DoesNotExist,
  BadReference,
  DupReference,
  OutOfRange,
  SRPTPriority,
  InvalidFormat,
  Io(io::Error),
  Num(num::ParseIntError),
}
