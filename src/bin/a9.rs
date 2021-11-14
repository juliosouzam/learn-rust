fn get_coordinates() -> (i32, i32) {
  (6, 5)
}

fn main() {
  let (_x, y) = get_coordinates();

  if y < 5 {
    println!("less then 5");
  } else if y > 5 {
    println!("greater then 5");
  } else {
    println!("equals");
  }
}
