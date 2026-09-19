use std::vec::Vec;

fn main() {
  let a: i16 = 0;
  let b: i16 = 32767;
  let c: i16 = -32768;
  let d: i16 = 100;
  let v: Vec<u64> = Vec::with_capacity(4);

  let a1 = a.to_be(); println!("i16 {:6} {:4x} pg {:6} {:4x}", a, a, a1, a1);
  let b1 = b.to_be(); println!("i16 {:6} {:4x} pg {:6} {:4x}", b, b, b1, b1);
  let c1 = c.to_be(); println!("i16 {:6} {:4x} pg {:6} {:4x}", c, c, c1, c1);
  let d1 = d.to_be(); println!("i16 {:6} {:4x} pg {:6} {:4x}", d, d, d1, d1);
}
