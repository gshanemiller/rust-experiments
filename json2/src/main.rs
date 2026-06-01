use log;
mod err;
mod cfg;
mod mem;
mod numa;
mod logger;

use std::process;
use crate::cfg::interface::Verify;
use mem::gblalloc::{GlobalAllocator};

#[global_allocator]
static GBLALLOC: GlobalAllocator = GlobalAllocator::new(512*1024);

#[allow(non_snake_case)]
extern "C" fn dumpGlobalStatsAtExit() {
  let stats = GBLALLOC.stats();
  stats.dump("GlobalAllocator @ process termination");
}

fn main() {
  unsafe {
    if libc::atexit(dumpGlobalStatsAtExit) != 0 {
      eprintln!("Failed to register atexit handler 'destroyAllocStats'");
      process::exit(1);
    }
  }

  _ = logger::logger::Logger::new();

  let fname = "./transport.json";
  let mut cfg: cfg::testnic::TestNIC = cfg::testnic::TestNIC::new();
  match cfg.parseFile(&fname) {
    Ok(_) => { log::info!("'{}' valid", fname); }
    Err(err) => { log::error!("'{}' invalid: {:?}", fname, err); }
  };

  let stats = GBLALLOC.stats();
  stats.dump("end-of-main");
}
