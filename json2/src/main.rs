mod err;
mod cfg;
mod mem;
mod numa;
mod logger;

use crate::cfg::interface::Verify;

fn main() {
  let fname = "./transport.json";
  let mut cfg: cfg::testnic::TestNIC = cfg::testnic::TestNIC::new();
  _ = cfg.parseFile(&fname);
}
