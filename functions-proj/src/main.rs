fn main() {
    apply_to_jobs(35, "Rust Developer");
    println!("{}", is_even(8));
    println!("{}", is_even(9));
    println!("{:#?}", alphabets("aardvark"));
    println!("{:#?}", alphabets("zoology"));
    println!("{:#?}", alphabets("zebra"));
}

fn apply_to_jobs(n_jobs: i32, title: &str) {
    println!("I'm applying to {n_jobs} {title} jobs");
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("z"))
}
