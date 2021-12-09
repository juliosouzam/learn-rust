use std::io;

enum PowerState {
  Off,
  Sleep,
  Reboot,
  Shutdonw,
  Hibernate,
}

impl PowerState {
  fn new(state: &str) -> Option<Self> {
    let state: String = state.trim().to_lowercase();
    match state.as_str() {
      "off" => return Some(Self::Off),
      "sleep" => return Some(Self::Sleep),
      "reboot" => return Some(Self::Reboot),
      "shutdown" => return Some(Self::Shutdonw),
      "hibernate" => return Some(Self::Hibernate),
      _ => return None,
    };
  }
}

fn print_power_action(state: PowerState) {
  use PowerState::*;
  match state {
    Off => println!("turning off"),
    Sleep => println!("sleeping"),
    Reboot => println!("rebooting"),
    Shutdonw => println!("shutting down"),
    Hibernate => println!("hibernating"),
  }
}

fn main() {
  print!("> ");
  let mut buffer = String::new();
  let user_input_status = io::stdin().read_line(&mut buffer);

  if user_input_status.is_ok() {
    match PowerState::new(&buffer) {
      Some(state) => print_power_action(state),
      None => println!("invalid power state"),
    }
  } else {
    println!("error reading input");
  }
}
