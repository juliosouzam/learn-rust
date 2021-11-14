struct Temperature {
  degrees_f: f64,
}

impl Temperature {
  fn new(degrees_f: f64) -> Self {
    Self {
      degrees_f: degrees_f,
    }
  }

  fn freezing() -> Self {
    Self { degrees_f: 32.0 }
  }

  fn show_temp(&self) {
    println!("{:?} degress F ", self.degrees_f);
  }
}

fn main() {
  let hot = Temperature { degrees_f: 100.0 };
  // Temperature::show_temp(hot);
  hot.show_temp();

  let cold = Temperature::freezing();
  cold.show_temp();

  let new_temp = Temperature::new(50.0);
  new_temp.show_temp();
}
