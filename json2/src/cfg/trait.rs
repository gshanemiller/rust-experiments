use super::parserutil::{ParserUtil};
use super::super::err::error::{Error};

pub trait Verify {
  fn verify(&self, util: &ParserUtil) -> Result<bool, Error>;
  fn crossVerify(&self, util: &ParserUtil) -> Result<bool, Error>;
  fn numaVerify(&self, util: &ParserUtil) -> Result<bool, Error>;
  fn createAllocators(&self, util: &ParserUtil) -> Result<bool, Error>;
}
