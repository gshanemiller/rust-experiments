use std::process;

mod mem;
mod err;
mod numa;
mod logger;

use logger::logger::{Logger};
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

  match Logger::new() {
    Ok(_) => {},
    Err(err) => {
      println!("Failed to initialize logger: {:?}", err);
      process::exit(1);
    }
  };
}
