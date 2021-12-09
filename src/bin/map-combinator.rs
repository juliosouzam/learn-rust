fn maybe_num() -> Option<i32> {
  Some(3)
}

fn maybe_word() -> Option<String> {
  Some("lorem".to_owned())
}

fn main() {
  // let plus_one = match maybe_num() {
  //   Some(num) => Some(num + 1),
  //   None => None,
  // };

  // let plus_one = maybe_num().map(|num| num + 1);

  let word_length = maybe_word().map(|w| w.len()).map(|l| l * 2);
}
