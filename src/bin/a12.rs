enum Color {
  Brown,
  Red,
}

impl Color {
  fn print(&self) {
    match self {
      Color::Brown => println!("brown"),
      Color::Red => println!("red"),
    }
  }
}

struct Dimensions {
  width: f64,
  height: f64,
  depth: f64,
}

impl Dimensions {
  fn print(&self) {
    println!("width: {:?}", self.width);
    println!("height: {:?}", self.height);
    println!("depth: {:?}", self.depth);
  }
}

struct ShippingBox {
  color: Color,
  weight: f64,
  dimensions: Dimensions,
}

impl ShippingBox {
  fn new(color: Color, weight: f64, dimensions: Dimensions) -> Self {
    Self {
      color,
      weight,
      dimensions,
    }
  }

  fn print(&self) {
    self.color.print();
    self.dimensions.print();
    println!("weight: {:?}", self.weight);
  }
}

fn main() {
  let small_dimensions = Dimensions {
    width: 1.0,
    height: 2.0,
    depth: 3.0,
  };

  let small_box = ShippingBox::new(Color::Brown, 2.0, small_dimensions);
  small_box.print();

  let large_dimensions = Dimensions {
    width: 10.0,
    height: 40.0,
    depth: 80.0,
  };

  let large_box = ShippingBox::new(Color::Red, 129.54, large_dimensions);
  large_box.print();
}
