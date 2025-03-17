fn main() {
    let mut current_value = "10000".to_string();
    make_money(&mut current_value);
    println!("{current_value}");

    let raw_string = "  Here is some sentence.   ";
    let normalized = trim_and_capitalize(raw_string);
    println!("{normalized}");

    let string_vals = "Gold!Silver!Platinum";
    println!("{:?}", elements(string_vals));

    let full_name = get_identity();
    println!("{full_name}");
}

fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();

    print!("What is your first name?\n");
    match std::io::stdin().read_line(&mut first_name) {
        Ok(_) => {}
        Err(_) => println!("Failed to collect first name."),
    }

    print!("What is your last name?\n");
    match std::io::stdin().read_line(&mut last_name) {
        Ok(_) => {}
        Err(_) => println!("Failed to collect last name."),
    }

    format!("{} {}", first_name.trim(), last_name.trim())
}

fn make_money(val: &mut String) {
    val.push_str("$$$")
}

fn trim_and_capitalize(val: &str) -> String {
    val.trim().to_string().to_uppercase()
}

fn elements(base: &str) -> Vec<&str> {
    base.split("!").collect::<Vec<&str>>()
}
