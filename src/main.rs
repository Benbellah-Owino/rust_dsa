use binary_search::{searching::*, sorting::*};
fn main() {
    let mut numbers: Vec<i32> = vec![9, 8, 7, 6, 5, 5, 4, 33, 2, 1, 3];

    bubble_sort(&mut numbers);
    println!("{:?}\n\n", numbers);
    let n = linear_search(7  , &numbers);
    println!("{}", n.unwrap())
}
