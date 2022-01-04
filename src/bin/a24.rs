fn main() {
    let data: Vec<_> = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|n| n * 3)
        .filter(|n| n > &10)
        .collect();

    for n in data {
        println!("{:?}", n);
    }
}
