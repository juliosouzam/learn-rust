struct User {
  name: String,
  email: String,
  age: i32,
}

impl User {
  fn new(name: String, email: String, age: i32) -> Self {
    Self {
      name: name,
      email: email,
      age: age,
    }
  }

  fn info(&self) {
    println!(
      "Name: {:?}, E-mail: {:?}, Age: {:?}",
      self.name, self.email, self.age
    );
  }
}

fn main() {
  let users = vec![
    User::new("Júlio César".to_owned(), "julio@mail.com".to_owned(), 24),
    User::new("Anne".to_owned(), "anne@mail.com".to_owned(), 22),
  ];

  for user in users {
    user.info();
  }
}
