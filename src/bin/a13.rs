fn main() {
  let num = vec![10, 20, 30, 40];

  for i in &num {
    match i {
      30 => println!("thirty"),
      _ => println!("{:?}", i),
    }
  }

  println!("number of elements = {:?}", num.len());
}
