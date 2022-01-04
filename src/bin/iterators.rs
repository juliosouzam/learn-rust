fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    let plus_one: Vec<_> = numbers.iter().map(|n| n + 1).collect();
    let new_numbers: Vec<_> = numbers.iter().filter(|n| n >= 3).collect();
    let find_me: Option<i32> = numbers.iter().find(|n| n == 3)
    let count = numbers.iter().count();
    let last: Option<i32> = numbers.iter().last();
    let min: Option<i32> = numbers.iter().min();
    let max: Option<i32> = numbers.iter().max();
    let take: Vec<i32> = numbers.iter().take(3).collect();
}