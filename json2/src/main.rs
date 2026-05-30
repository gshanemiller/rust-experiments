use log;
mod err;
mod cfg;
mod mem;
mod numa;
mod logger;

use crate::cfg::interface::Verify;

fn main() {
  _ = logger::logger::Logger::new();

  let fname = "./transport.json";
  let mut cfg: cfg::testnic::TestNIC = cfg::testnic::TestNIC::new();
  match cfg.parseFile(&fname) {
    Ok(_) => { log::info!(target: "json", "'{}' valid", fname); }
    Err(err) => { log::error!(target: "json", "'{}' invalid: {:?}", fname, err); }
  };
}
