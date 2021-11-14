fn add(a: i32, b: i32) -> i32 {
  a + b
}

fn display_result(result: i32) {
  println!("{:?}", result);
}

fn main() {
  let sum = add(9, 6);
  display_result(sum);
}
