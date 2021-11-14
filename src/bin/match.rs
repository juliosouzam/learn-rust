fn main() {
  let some_bool = false;

  match some_bool {
    true => println!("its true"),
    false => println!("its false"),
  }

  let some_int = 3;

  match some_int {
    1 => println!("its 1"),
    2 => println!("its 2"),
    3 => println!("its 3"),
    _ => println!("its something else..."),
  }

  let my_name = "Bob";

  match my_name {
    "Júlio" => println!("that is my name"),
    "Bob" => println!("not my name"),
    "John" => println!("not my name"),
    _ => println!("ops, i dunno"),
  }
}
