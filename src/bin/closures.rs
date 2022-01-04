fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let name = "Xulho";
    let add: Box<_> = Box::new(|a, b| {
        println!("adding");
        a + b
    });
    let sub: Box<_> = Box::new(move |a, b| {
        println!("sub a number for {}", name);
        a - b
    });
    let mul: Box<_> = Box::new(|a, b| a * b);
    println!("{}", math(5, 1, add));
    println!("{}", math(5, 1, sub));
    println!("{}", math(5, 1, mul));
}
