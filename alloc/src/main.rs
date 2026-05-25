#![feature(allocator_api)]

mod mem;
use mem::dbgalloc::{DefaultAllocator};

fn main() {
  let test = DefaultAllocator::new(512*1024);
  test.dump();
  let mut data: Vec<i32, DefaultAllocator> = Vec::new_in(test);
  for i in 1..=100 {
    println!("push {}", i);
    data.push(i);
  }
  println!("data has {} elements", data.len());
  println!("drop data");
  drop(data);
}
