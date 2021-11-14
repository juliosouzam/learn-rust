fn print_msg(gt_100: bool) {
  match gt_100 {
    true => println!("its big"),
    false => println!("its small or equal"),
  }
}

fn main() {
  let value = 100;
  let is_gt_100 = value > 100;
  print_msg(is_gt_100);
}
