enum Flavor {
  Sparkling,
  Sweet,
  Fruity,
}

struct Drink {
  flavor: Flavor,
  fluid_oz: f64,
}

fn print_drink(drink: Drink) {
  match drink.flavor {
    Flavor::Fruity => println!("flavor: Fruity"),
    Flavor::Sparkling => println!("flavor: Sparkling"),
    Flavor::Sweet => println!("flavor: Sweet"),
  };
  println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
  let sweet = Drink {
    flavor: Flavor::Sweet,
    fluid_oz: 10.33,
  };
  print_drink(sweet);

  let fruity = Drink {
    flavor: Flavor::Fruity,
    fluid_oz: 4.0,
  };
  print_drink(fruity);

  let sparkling = Drink {
    flavor: Flavor::Sparkling,
    fluid_oz: 4.2,
  };
  print_drink(sparkling);
}
