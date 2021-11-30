enum Discount {
  Percent(i32),
  Flat(i32),
}

struct Ticket {
  event: String,
  price: i32,
}

fn main() {
  let n = 3;
  match n {
    3 => println!("three"),
    other => println!("{:?}", other),
  };

  let flat = Discount::Flat(4);
  match flat {
    Discount::Flat(4) => println!("flat 4"),
    Discount::Flat(ammount) => println!("flat discount of {:?}", ammount),
    _ => (),
  };

  let percent = Discount::Percent(2);
  match percent {
    Discount::Percent(2) => println!("percent 2"),
    Discount::Percent(ammount) => println!("percent discont of {:?}", ammount),
    _ => (),
  };

  let concert = Ticket {
    event: "concert".to_owned(),
    price: 60,
  };
  match concert {
    Ticket { price: 60, event } => println!("event @ 60 = {:?}", event),
    Ticket { price, .. } => println!("price = {:?}", price),
  };
}
