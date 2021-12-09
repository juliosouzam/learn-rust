use std::collections::HashMap;

mod greet {
  use std::collections::HashMap;

  fn hi() {
    println!("hi, boss");
  }

  fn goodbye() {
    println!("goodbye");
  }
}

mod math {
  fn add(a: i32, b: i32) -> i32 {
    a + b
  }

  fn sub(a: i32, b: i32) -> i32 {
    a - b
  }
}

fn main() {
  // use greet::*;
  // use greet::hello;
  greet::hi();

  math::add(3, 6);
}
