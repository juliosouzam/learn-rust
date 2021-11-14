enum Access {
  Admin,
  Manager,
  User,
  Guest,
}

struct User {
  name: String,
  email: String,
  level: Access,
}

fn main() {
  let user = User {
    name: "Júlio".to_string(),
    email: "julio@mail.com".to_string(),
    level: Access::Manager,
  };

  let can_access_file = match user.level {
    Access::Admin => true,
    _ => false,
  };

  if can_access_file {
    println!("User {:?} can access the file", user.name);
  } else {
    println!("User {:?} can not access the file", user.name);
  }
}
