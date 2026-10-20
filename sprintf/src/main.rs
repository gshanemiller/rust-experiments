use core::fmt::Write;
use core::fmt::Result;

struct Sprintf {
}

impl Write for Sprintf {
  fn write_str(&mut self, s: &str) -> Result {
    println!("{}", s);
    return Ok(());
  }
}

fn main() {
  let mut s = Sprintf{};
  let args = format_args!("Hello {} of age {}\n", "Shane", 4);
  match core::fmt::write(&mut s, args) {
    Ok(()) => println!("ok"),
    Err(e) => println!("error {:?}", e),
  }
}
