fn main() {
  let mut numbers = vec![1, 2, 3];

  numbers.first();

  assert_eq!(numbers.pop(), Some(3));
}
