fn double_the_length<T>(items: &Vec<T>) -> usize {
    items.len() * 2
}

fn last_two<T>(items: &[T]) -> &[T] {
    let first = items.len() - 2;
    &items[first..]
}

fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{announcement}");
    &text[..=5]
}

fn find_string_that_has_content<'a>(
    first: &'a str,
    second: &'a str,
    target: &str,
) -> Result<&'a str, String> {
    if first.contains(target) {
        Ok(&first)
    } else if second.contains(target) {
        Ok(&second)
    } else {
        let error_msg = format!("Target: {target} was not found in either string.");
        Err(error_msg)
    }
}

fn main() {
    println!("{}", double_the_length(&vec![1, 2, 3]));
    println!("{}", double_the_length(&vec![1, 2, 3, 4, 5, 6]));

    println!("{:?}", last_two(&vec![12, 36, 11, 45, 90]));
    println!("{:?}", last_two(&vec![21, 3, 84, 101, 52]));

    let result = first_five("Super Awesome", "This is your captain speaking");
    println!("{result}");

    let result = find_string_that_has_content("programming", "dining", "din");
    match result {
        Ok(val) => println!("{val}"),
        Err(msg) => println!("{msg}"),
    }
}
