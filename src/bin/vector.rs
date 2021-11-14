struct User {
  name: String,
  age: i32,
}

fn main() {
  let _my_num = vec![1, 2, 3];
  let mut my_new_num = Vec::new();
  my_new_num.push(1);
  my_new_num.push(2);
  my_new_num.push(3);
  my_new_num.len();

  let my_vector = vec![1, 2, 3];

  for i in my_vector {
    println!("{:?}", i);
  }

  let users = vec![
    User {
      name: String::from("John"),
      age: 30,
    },
    User {
      name: String::from("Jane"),
      age: 31,
    },
  ];

  users.iter().for_each(|user| println!("{:?}", user.name));

  for user in users {
    println!("Name {:?}, Age {:?}", user.name, user.age);
  }
}
