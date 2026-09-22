#[repr(C)]
union Stuff {
    uint64: u64,
    uint32: u32,
    uint16: u16,
    bytes:  [u8; 8],
}

impl Stuff {
  fn new() -> Self {
    Self {
      uint64: 0,
    }
  }

  fn setU16(&mut self, val: u16) {
    unsafe {
      self.uint64 = 0;
      self.uint16 = val.to_be();
    }
  }

  fn print(&self) {
    unsafe {
      println!("uint64 {:8x} uint32 {:4x} uint16 {:2x} bytes {:?}",
        self.uint64, self.uint32, self.uint16, self.bytes);
    }
  }
}

fn main() {
  let a: u16 = 0;
  let b: u16 = 0xffff;
  let c: u16 = 100;
  let mut v = Stuff::new();

  v.setU16(a); v.print();
  v.setU16(b); v.print();
  v.setU16(c); v.print();

  println!("size of Stuff {}", size_of::<Stuff>());
}
