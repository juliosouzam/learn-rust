use std::collections::HashMap;
use std::io;
use uuid::Uuid;

#[derive(Debug)]
struct Bill {
  id: String,
  name: String,
  amount: f64,
}

struct Bills {
  inner: HashMap<String, Bill>,
}

impl Bills {
  fn new() -> Self {
    Self {
      inner: HashMap::new(),
    }
  }

  fn add(&mut self, bill: Bill) {
    self.inner.insert(bill.id.clone(), bill);
  }

  fn get_all(&self) -> &HashMap<String, Bill> {
    &self.inner
  }

  fn remove(&mut self, id: &str) -> bool {
    self.inner.remove(id).is_some()
  }

  fn update(&mut self, id: &str, name: &str, amount: f64) -> bool {
    match self.inner.get_mut(id) {
      Some(bill) => {
        bill.amount = amount;
        bill.name = name.to_owned();

        true
      }
      None => false,
    }
  }
}

fn get_input() -> Option<String> {
  let mut buffer = String::new();
  while io::stdin().read_line(&mut buffer).is_err() {
    println!("Please enter your data again");
  }

  let input = buffer.trim().to_owned();
  if &input == "" {
    None
  } else {
    Some(input)
  }
}

fn get_bill_amount() -> Option<f64> {
  println!("Amount:");
  loop {
    let input = match get_input() {
      Some(input) => input,
      None => return None,
    };
    if &input == "" {
      return None;
    }
    let parsed_input: Result<f64, _> = input.parse();
    match parsed_input {
      Ok(amount) => return Some(amount),
      Err(_) => println!("Please enter a number"),
    }
  }
}

fn add_bill_menu(bills: &mut Bills) {
  println!("Bill name: ");
  let name = match get_input() {
    Some(input) => input,
    None => return,
  };
  let amount = match get_bill_amount() {
    Some(amount) => amount,
    None => return,
  };
  let id = Uuid::new_v4();
  let bill = Bill {
    id: id.to_string(),
    name,
    amount,
  };
  bills.add(bill);
  println!("Bill added");
}

fn remove_bill_menu(bills: &mut Bills) {
  for bill in bills.get_all().values() {
    println!("{:?}", bill);
  }
  println!("Enter bill id to remove:");
  let input = match get_input() {
    Some(input) => input,
    None => return,
  };
  if bills.remove(&input) {
    println!("removed");
  } else {
    println!("bill not found");
  }
}

fn update_bill_menu(bills: &mut Bills) {
  for bill in bills.get_all().values() {
    println!("{:?}", bill);
  }
  println!("Enter bill id to update:");
  let input_id = match get_input() {
    Some(input) => input,
    None => return,
  };
  println!("Enter bill name to update:");
  let input_name = match get_input() {
    Some(input) => input,
    None => return,
  };
  let amount = match get_bill_amount() {
    Some(input) => input,
    None => return,
  };
  if bills.update(input_id.as_str(), input_name.as_str(), amount) {
    println!("update successful");
  } else {
    println!("bill not found");
  }
}

fn view_bills_menu(bills: &Bills) {
  for bill in bills.get_all() {
    println!("{:?}", bill);
  }
}

fn main_menu() {
  fn show() {
    println!("");
    println!("== Manage Bills ==");
    println!("1. Add bills");
    println!("2. View bills");
    println!("3. Remove bill");
    println!("4. Update bill");
    println!("");
    println!("Enter selection: ");
  }

  let mut bills = Bills::new();

  loop {
    show();
    let input = match get_input() {
      Some(input) => input,
      None => return,
    };
    match input.as_str() {
      "1" => add_bill_menu(&mut bills),
      "2" => view_bills_menu(&bills),
      "3" => remove_bill_menu(&mut bills),
      "4" => update_bill_menu(&mut bills),
      _ => break,
    }
  }
}

fn main() {
  main_menu();
}
