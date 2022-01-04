fn main() {
    let mut data = Some(3);

    while let Some(_i) = data {
        println!("loop");
        data = None;
    }

    let numbers = vec![1, 2, 3, 4];
    let mut number_iter = numbers.iter();
    while let Some(i) = number_iter.next() {
        println!("num = {:?}", i);
    }

    println!("done");
}
