#[derive(Debug, Eq, PartialEq)]
enum Access {
  Admin,
  User,
  Guest,
}

fn maybe_access(name: &str) -> Option<Access> {
  match name {
    "admin" => Some(Access::Admin),
    "user" => Some(Access::User),
    "guest" => Some(Access::Guest),
    _ => None,
  }
}

fn root() -> Option<Access> {
  Some(Access::Admin)
}

fn part_1() -> bool {
  maybe_access("admin")
}

fn part_2() -> Option<Access> {
  maybe_access("root")
}

fn part_3() -> Access {
  maybe_access("Alice")
}

fn main() {
  //
}

#[cfg(test)]
mod test {
  use crate::*;

  #[test]
  fn check_part_1() {
    assert_eq!(part_1(), true, "Admin have an access level");
  }

  #[test]
  fn check_part_2() {
    assert_eq!(
      part_2(),
      Some(Access::Admin),
      "Root users have Admin access"
    );
  }

  #[test]
  fn check_root() {
    assert_eq!(root(), Access::Guest, "Alice is a Guest");
  }
}
