use std::string;

fn foo(a: Option<&String>) {
  if let Some(s) = a {
    if !s.is_empty() {
      println!("got '{}'", s);
    } else {
      println!("got empty string");
    }
  } else {
    println!("didn't get anything");
  }
}


fn main() {
  let s = String::from("bubba");
  let t = String::from("");
  foo(Some(&s));
  foo(Some(&t));
  foo(None);
}
