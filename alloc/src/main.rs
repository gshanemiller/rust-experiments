#![feature(allocator_api)]

mod mem;
use mem::dbgalloc::{DefaultAllocator};

fn main() {
  let test = DefaultAllocator::new(512*1024);
  let mut data: Vec<u32, DefaultAllocator> = Vec::new_in(test);
  for i in 0..=100 {
    println!("push {}", i);
    data.push(i);
  }
  for i in 0..=100 {
    if data[i]!=i as u32 {
      println!("index {} should be {}", i, data[i]);
    }
  }
  println!("data has {} elements", data.len());
  println!("drop data");
  drop(data);
}
