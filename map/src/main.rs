use std::collections::HashMap;

enum Error {
  Dup,
}

struct Value {
  pub name: String,
  pub other: Vec<u32>,
  pub more: [u32; 5],
}

impl Value {
  fn new() -> Self {
    Self {
      name: String::new(),
      other: Vec::new(),
      more: [0,0,0,0,0],
    }
  }

  fn print(&self) {
    println!("name: {}", self.name);
    println!("other: {:?}", self.other);
    println!("more: {:?}", self.more);
  }
}

struct Container {
  data: HashMap<String, Value>,
}

impl Container {
  fn new() -> Self {
    Self {
      data: HashMap::new(),
    }
  }

  fn createOrFindValue(&mut self, key: &String) -> Option<&mut Value> {
    if !self.data.contains_key(key) {
      self.data.entry(key.clone()).or_insert(Value::new());
    }
    return self.data.get_mut(key);
  }
}

fn main() {
  let key = "shane".to_string();
  let mut cont: Container = Container::new();
  let mut data = cont.createOrFindValue(&key).unwrap();
  data.print();
  data.name = "stuff".to_string();
  data.other.push(10);
  data.more[1] = 10;
  data.print();
  data = cont.createOrFindValue(&key).unwrap();
  data.print();
}
