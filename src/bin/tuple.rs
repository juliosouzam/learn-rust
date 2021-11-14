fn main() {
  let coord = (2, 3);
  println!("{:?} x {:?}", coord.0, coord.1);
  let (x, y) = coord;
  println!("{:?} x {:?}", x, y);
}
