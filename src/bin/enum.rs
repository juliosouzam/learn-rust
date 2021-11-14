enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn main() {
  let go = Direction::Left;
  which_direction(go);
}

fn which_direction(go: Direction) {
  match go {
    Direction::Up => println!("Up"),
    Direction::Down => println!("Down"),
    Direction::Left => println!("Left"),
    Direction::Right => println!("Right"),
  }
}
