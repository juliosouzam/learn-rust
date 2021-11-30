struct User {
  name: String,
  color: String,
  age: i32,
}

fn print(data: &str) {
  println!("{:?}", data);
}

fn main() {
  let users = vec![
    User {
      name: "Alberto".to_owned(),
      color: "red".to_owned(),
      age: 15,
    },
    User {
      name: String::from("Anne"),
      color: "blue".to_owned(),
      age: 9,
    },
    User {
      name: "Julio".to_owned(),
      color: "white".to_owned(),
      age: 7,
    },
  ];

  for user in users {
    if user.age <= 10 {
      print(&user.name);
      print(&user.color);
    }
  }
}
