struct Student {
  name: String,
  locker: Option<i32>,
}

fn main() {
  let mery = Student {
    name: "Mary".to_owned(),
    locker: Some(3),
  };
  println!("student: {:?}", mery.name);
  match mery.locker {
    Some(num) => println!("locker number {:?}", num),
    None => println!("no locker assigned"),
  }
}
