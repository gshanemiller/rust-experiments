use std::fs;
use std::process;
use tinyjson::JsonValue;
use std::collections::HashMap;

mod mem;
use mem::gblalloc::{GlobalAllocator};

#[global_allocator]
static GBLALLOC: GlobalAllocator = GlobalAllocator::new(512*1024);

#[allow(non_snake_case)]
extern "C" fn dumpGlobalStatsAtExit() {
  GBLALLOC.dump();
}

fn visit (obj: &JsonValue, prefix: String) {
  if obj.is_object() {
    visitObject(obj, prefix.clone());
    return;
  }

  if obj.is_array() {
    visitArray(obj, prefix.clone());
    return;
  }

  if obj.is_null() {
    return;
  }

  if obj.is_number() {
    let value: Option<&f64> = obj.get();
    if !value.is_none() {
      println!("'{}' = '{}'", prefix, value.unwrap());
      return;
    }
  }

  if obj.is_bool() {
    let value: Option<&bool> = obj.get();
    if !value.is_none() {
      println!("'{}' = '{}'", prefix, value.unwrap());
      return;
    }
  }

  if obj.is_string() {
    let value: Option<&String> = obj.get();
    if !value.is_none() {
      println!("'{}' = '{}'", prefix, value.unwrap());
      return;
    }
  }

  panic!("unhandled case: {:?}", obj);
}

fn visitObject(obj: &JsonValue, prefix: String) {
  let map: &HashMap<_, _> = obj.get().unwrap();
  for (k, v) in map {
    let prefix = format!("{}.{}", prefix, k);
    visit(&v, prefix.clone());
  }
}

fn visitArray(obj: &JsonValue, prefix: String) {
  let vect: &Vec<_> = obj.get().unwrap();
  for item in vect {
    visit(&item, prefix.clone());
  }
}

fn main() {
  unsafe {
    if libc::atexit(dumpGlobalStatsAtExit) != 0 {
      eprintln!("Failed to register atexit handler 'destroyAllocStats'");
      process::exit(1);
    }
  }

  let filename = "./transport.json";
  let input = fs::read_to_string(filename).unwrap();
  let parsed: JsonValue = input.parse().unwrap();

  if parsed.is_object() {
    println!("top level object object");
  }

  visit(&parsed, "root".to_string());

  GBLALLOC.dump();
  let stats = GBLALLOC.stats();
  stats.dump("Hi!!!!");
}
