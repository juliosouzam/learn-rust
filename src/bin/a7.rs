enum Colors {
  Orange,
  Green,
  Red,
  Blue,
}

fn main() {
  display_name_color(Colors::Blue);
  display_name_color(Colors::Green);
  display_name_color(Colors::Orange);
  display_name_color(Colors::Red);
}

fn display_name_color(color: Colors) {
  match color {
    Colors::Blue => println!("Color is blue"),
    Colors::Green => println!("Color is green"),
    Colors::Orange => println!("Color is orange"),
    Colors::Red => println!("Color is red"),
  }
}
