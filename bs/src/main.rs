use std::cell::Cell;

#[derive(Debug, Default)]
struct MyData {
  data: u32,
}

impl MyData {
  fn new(init: u32) -> Self {
    Self {
      data: init,
    }
  }

  fn increment(&mut self) {
    self.data += 1;
  }

  fn value(&self) -> u32 {
    return self.data;
  }
}

struct MyTest {
  a: u32,
  b: Cell<MyData>,
}

impl MyTest {
  pub fn new(value: u32) -> Self {
    return Self {
      a: value,
      b: MyData::new(10).into(),
    }
  }

  pub fn ex1(&self) {
    let mut data = self.b.take();
    println!("before {} {}", self.a, data.value());
    data.increment();
    println!("after  {} {}", self.a, data.value());
    self.b.set(data);
  }
}

fn main() {
  let test = MyTest::new(57);
  test.ex1();
  test.ex1();
}
