use std::vec::Vec;
use std::ffi::{CString};                                                                                  

fn testVec(val: Vec<u8>) {
  println!("testVec '{:?}'", val);
}

fn testVecRef(val: &Vec<u8>) {
  println!("testVecRef '{:?}'", val);
}

fn testCString(val: CString) {
  println!("testCString '{:?}'", val);
}

fn testCStringRef(val: &CString) {
  println!("testCStringRef '{:?}'", val);
}

fn main() {
  testCString(CString::new("test1").expect("bad ascii"));
  testCStringRef(&CString::new("test2").expect("bad ascii"));

  let t3 = CString::new("test3").expect("bad ascii");
  testCString(t3);
  println!("t3 is '{:?}'", t3);

  let t4 = CString::new("test4").expect("bad ascii");
  testCStringRef(&t4);
  println!("t4 is '{:?}'", t4);

  testVec(vec![1,2,3]);
  testVecRef(&vec![1,2,3]);

  let t5 = vec![5,6,7];
  testVec(t5);
  println!("t5 is '{:?}'", t5);

  let t6 = vec![11,12,13];
  testVecRef(&t6);
  println!("t6 is '{:?}'", t6);
}
