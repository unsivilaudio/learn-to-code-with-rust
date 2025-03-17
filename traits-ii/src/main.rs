use std::ops::Add;

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("{}", add_two_numbers(5, 3));
    println!("{}", add_two_numbers(12.99, 4.76));
}
