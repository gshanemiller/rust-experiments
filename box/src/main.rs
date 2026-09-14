fn modify_box(mut b: Box<i32>) -> Box<i32> {
   *b += 10;
   b
}

fn modify_box1(b: &mut Box<i32>) {
  **b = 1000001;
}

fn main() {
  let mut my_box = Box::new(5);

  my_box = modify_box(my_box);
  println!("Value after modify_box: {}", my_box);

  modify_box1(&mut my_box);
  println!("Value after modify_box1: {}", my_box);
}
