use std::process;

mod mem;
mod cfg;
mod err;

use mem::gblalloc::{GlobalAllocator};
use cfg::parser::Parser;

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

  let parserResult = Parser::new("transport.json");
  let res = match parserResult {
    Ok(val) => { println!("result {}", val); }
    Err(err) => { println!("result {:?}", err); }
  }; 
}
