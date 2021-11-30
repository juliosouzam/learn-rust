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

fn can_access_file(user: &User) -> bool {
  let can_access_file = match user.level {
    Access::Admin => true,
    Access::Manager => true,
    _ => false,
  };

  can_access_file
}

fn main() {
  let user_manager = User {
    name: "Manager".to_string(),
    email: "manager@mail.com".to_string(),
    level: Access::Manager,
  };

  if can_access_file(&user_manager) {
    println!(
      "user {:?} with email {:?} can access the file",
      user_manager.name, user_manager.email
    );
  }

  let user_admin = User {
    name: "Admin".to_string(),
    email: "admin@mail.com".to_string(),
    level: Access::Admin,
  };

  if can_access_file(&user_admin) {
    println!(
      "user {:?} with email {:?} can access the file",
      user_admin.name, user_admin.email
    );
  }

  let user = User {
    name: "User".to_string(),
    email: "user@mail.com".to_string(),
    level: Access::User,
  };

  if can_access_file(&user) {
    println!(
      "user {:?} with email {:?} can access the file",
      user.name, user.email
    );
  }

  let user_guest = User {
    name: "User".to_string(),
    email: "user@mail.com".to_string(),
    level: Access::Guest,
  };

  if can_access_file(&user_guest) {
    println!(
      "user {:?} with email {:?} can access the file",
      user_guest.name, user_guest.email
    );
  }
}
