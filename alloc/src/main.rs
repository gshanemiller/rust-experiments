#![feature(allocator_api)]

mod mem;
use mem::dbgalloc::{DefaultAllocator};

fn main() {
  let test = DefaultAllocator::new(512*12);
  let mut data: Vec<i32, DefaultAllocator> = Vec::new_in(test);
  for i in 1..=100 {
    data.push(i);
  }
  println!("data has {} elements", data.len());
  drop(data);
}
