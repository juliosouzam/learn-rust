enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn main() {
  let go_left = Direction::Left;
  which_direction(go_left);
  let go_right = Direction::Right;
  which_direction(go_right);
  let go_up = Direction::Up;
  which_direction(go_up);
  let go_dowm = Direction::Down;
  which_direction(go_dowm);
}

fn which_direction(go: Direction) {
  match go {
    Direction::Up => println!("Up"),
    Direction::Down => println!("Down"),
    Direction::Left => println!("Left"),
    Direction::Right => println!("Right"),
  }
}
