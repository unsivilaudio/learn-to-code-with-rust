fn main() {
    println!("Color number: {}", color_to_number("red"));
    println!("Factorial of 5: {}", factorial(5));
    println!("Factorial (loop) of 5: {}", factorial_loop(5));
}

fn color_to_number(color: &str) -> i32 {
    // if color == "red" {
    //     1
    // } else if color == "green" {
    //     2
    // } else if color == "blue" {
    //     3
    // } else {
    //     0
    // }

    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial(num: i32) -> i32 {
    let next_lower = num - 1;
    if next_lower == 1 {
        return num;
    }
    num * factorial(next_lower)
}

fn factorial_loop(num: i32) -> i32 {
    let mut value = num;
    let mut next_lower = num - 1;
    while next_lower > 0 {
        value *= next_lower;
        next_lower -= 1;
    }
    value
}
