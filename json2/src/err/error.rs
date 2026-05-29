use std::io;
use std::num;

#[allow(non_snake_case)]
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
  Io(io::Error),
  Num(num::ParseIntError),
}
