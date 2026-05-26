use std::fs;
use tinyjson::JsonValue;
use std::collections::HashMap;

fn visit (obj: &JsonValue, prefix: String) {
  if obj.is_object() {
    visitObject(obj, prefix.clone());
    return;
  }

  if obj.is_array() {
    visitArray(obj, prefix.clone());
    return;
  }

  if obj.is_null() {
    return;
  }

  if obj.is_number() {
    let value: Option<&f64> = obj.get();
    if !value.is_none() {
      println!("'{}' = '{}'", prefix, value.unwrap());
      return;
    }
  }

  if obj.is_bool() {
    let value: Option<&bool> = obj.get();
    if !value.is_none() {
      println!("'{}' = '{}'", prefix, value.unwrap());
      return;
    }
  }

  if obj.is_string() {
    let value: Option<&String> = obj.get();
    if !value.is_none() {
      println!("'{}' = '{}'", prefix, value.unwrap());
      return;
    }
  }

  panic!("unhandled case: {:?}", obj);
}

fn visitObject(obj: &JsonValue, prefix: String) {
  let map: &HashMap<_, _> = obj.get().unwrap();
  for (k, v) in map {
    let prefix = format!("{}.{}", prefix, k);
    visit(&v, prefix.clone());
  }
}

fn visitArray(obj: &JsonValue, prefix: String) {
  let vect: &Vec<_> = obj.get().unwrap();
  for item in vect {
    visit(&item, prefix.clone());
  }
}

fn main() {
  let filename = "./transport.json";
  let input = fs::read_to_string(filename).unwrap();
  let parsed: JsonValue = input.parse().unwrap();

  if parsed.is_object() {
    println!("top level object object");
  }

  visit(&parsed, "root".to_string());
}
